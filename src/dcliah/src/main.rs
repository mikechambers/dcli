/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

mod startmoment;
use startmoment::StartMoment;

use chrono::{DateTime, Utc, Duration};
use dcli::error::Error;
use dcli::mode::ActivityMode;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::response::activities::Activity;
use dcli::statscontainer::ActivityStatsContainer;
use dcli::utils::{repeat_str, uppercase_first_char, format_f32};
use dcli::{apiinterface::ApiInterface, utils::EXIT_FAILURE};

use structopt::StructOpt;

//use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};

/*
fn print_tsv(
    data: ActivityStatsContainer,
    display_limit:i32,
    moment: StartMoment,
) {
    let mut name_values: Vec<(&str, String)> = Vec::new();

    name_values.push(("member_id", member_id.to_string()));
    print!("{}", build_tsv(name_values));
}
*/

fn print_default(
    data: ActivityStatsContainer,
    display_limit: i32,
    mode: ActivityMode,
    moment: StartMoment,
    date_time: DateTime<Utc>,
) {

    //todo: might want to look at buffering output
    //https://rust-cli.github.io/book/tutorial/output.html

    let activity_count = data.activities.len();

    if activity_count == 0 {
        println!("No activities found.");
        return;
    }

    let display_count = std::cmp::min(activity_count, display_limit as usize);
    let is_limited = activity_count != display_count;

    let date_time_label = if Utc::now() - date_time > Duration::days(6) {
        date_time.format("%B %-d %Y")
    } else {
        date_time.format("%A, %B %-d, %Y")
    };

    let title = format!(
        "{mode} activities since {date_time} ({moment})",
        mode = uppercase_first_char(&format!("{}", mode)),
        date_time = date_time_label,
        moment = moment,
    );
    
    println!();
    println!("{}", title);
    println!("{}", repeat_str(&"-", title.chars().count()));
    println!();
    println!("ACTIVITIES");
    println!("==================");

    if is_limited {
        println!(
            "Displaying details for the last {display_count} of {activity_count} activities.",
            display_count = display_count,
            activity_count = activity_count,
        );
    } else {
        println!(
            "Displaying details for the last {display_count} activit{ies}.",
            display_count = display_count,
            ies = {
                if (display_count) == 1 {
                    "y"
                } else {
                    "ies"
                }
            },
        );
    }
    println!();

    let col_w = 9;
    let col_wx2 = col_w * 2;

    //TODO: maybe format this yello background
    let header = format!(
        "{:>0col_wx2$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
        "MODE",
        "RESULT",
        "STREAK",
        "KILLS",
        "ASSISTS",
        "K+A",
        "DEATHS",
        "K/D",
        "KD/A",
        "EFF",
        col_w = col_w,
        col_wx2=col_wx2
    );
    println!("{}", header);
    println!("{}", repeat_str(&"-", header.chars().count()));

    let start_index = if is_limited {

        println!(
            "{:>0col_wx2$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            "...", "...", "...", "...", "...", "...", "...","...","...","...",
            col_w = col_w,
            col_wx2=col_wx2
        );

        activity_count - display_limit as usize
    } else {0};

    let slice = &data.activities[start_index..];

    for activity in slice {

        let mut mode_str = format!("{}", activity.details.mode);

        if mode_str.chars().count() > col_wx2 - 5 {
            mode_str = mode_str[..(col_wx2-5)].to_string();
            mode_str.push_str("..")
        }

        println!(
            "{:>0col_wx2$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            mode_str,
            format!("{}", activity.values.standing),
            "0",
            format!("{}", activity.values.kills),
            format!("{}", activity.values.assists),
            format!("{}", activity.values.opponents_defeated),
            format!("{}", activity.values.deaths),
            format_f32(activity.values.kills_deaths_ratio, 2),
            format_f32(activity.values.kills_deaths_assists, 2),
            format_f32(activity.values.efficiency, 2),
    
            col_w = col_w,
            col_wx2=col_wx2
        );
    }
    println!();
}

//TODO: this is called twice. need to track down.
fn parse_rfc3339(src: &str) -> Result<DateTime<Utc>, String> {
    let d =
        match DateTime::parse_from_rfc3339(src) {
            Ok(e) => e,
            Err(_e) => return Err(
                "Invalid RFC 3339 Date / Time String : Example : 2020-12-08T17:00:00.774187+00:00"
                    .to_string(),
            ),
        };

    let d = d.with_timezone(&Utc);

    if d > Utc::now() {
        return Err("start-date must be in the past.".to_string());
    }

    Ok(d)
}

async fn retrieve_activities_since(
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &ActivityMode,
    start_time: &DateTime<Utc>,
    verbose: bool,
) -> Result<Option<ActivityStatsContainer>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let activities: Vec<Activity> = match client
        .retrieve_activities_since(&member_id, &character_id, &platform, &mode, &start_time)
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    //TODO: check if we get back and empty vector
    let container = ActivityStatsContainer::with_activities(activities);

    println!("{:#?}", container.total_activities());

    Ok(Some(container))
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving Destiny 2 activity history.
///
/// Enables control of which stats are retrieved via game mode, start time
/// (to present) and character.
///
/// Created by Mike Chambers.
/// https://www.mikechambers.com
///
/// Get support, request features or just chat on the dcli Discord server:
/// https://discord.gg/2Y8bV2Mq3p
///
/// Get the latest version, download the source and log issues at:
/// https://github.com/mikechambers/dcli
///
/// Released under an MIT License.
struct Opt {
    /// Destiny 2 API member id
    ///
    /// This is not the user name, but the member id retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Custom start time in RFC 3339 date / time format
    ///
    /// Must be a valid date in the past.
    ///
    /// Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00
    ///
    /// Required when start-moment is set to custom, but otherwise not applicable.
    #[structopt(short = "d", long = "start-time", parse(try_from_str = parse_rfc3339), required_if("start-moment", "custom"))]
    start_time: Option<DateTime<Utc>>,

    /// Start moment from which to pull activities from
    ///
    /// Activities will be retrieved from start moment to the current time.
    /// For example, Specifying:
    /// --start-moment weekly_reset
    ///
    /// will return all activities since the last weekly reset on Tuesday.
    ///
    /// Valid values include daily (last daily reset), weekend
    /// (last weekend reset on Friday), weekly (last weekly reset on Tuesday),
    /// day (last day), week (last week), month (last month), alltime and custom.
    ///
    /// When custom is specified, the custom start date in RFC3339 format must
    /// be specified with the --start-time argument.
    ///
    /// For example:
    /// --start-moment custom --start-time 2020-12-08T17:00:00.774187+00:00
    ///
    /// Specifying alltime retrieves all activitiy history and may take an extended
    /// amount of time to retrieve depending on the number of activities.
    #[structopt(long = "start-moment", default_value = "day")]
    start_moment: StartMoment,

    /// Activity mode to return stats for
    ///
    /// Valid values are all (default), control, clash, mayhem, ironbanner,
    /// private, rumble, comp, quickplay and trialsofosiris.
    #[structopt(long = "mode", default_value = "all")]
    mode: ActivityMode,

    /// Limit the number of activity details that will be displayed.
    ///
    /// Summary information will be generated based on all activities. Ignored if
    /// --output is tsv.
    #[structopt(long = "display-limit", default_value = "10")]
    display_limit: i32,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(short = "o", long = "output", default_value = "default")]
    output: Output,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id for the character to retrieve activities for.
    #[structopt(short = "c", long = "character-id")]
    character_id: String,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    if opt.start_time.is_some() {
        println!("{}", opt.start_time.unwrap());
    }

    let start_time = match opt.start_moment {
        StartMoment::Custom => {
            opt.start_time.unwrap() //note, this should be ok, because struct opt should ensure valid value
        }
        _ => opt.start_moment.get_date_time(),
    };

    //todo: is there any need to send a reference to an enum?
    let data = match retrieve_activities_since(
        &opt.member_id,
        &opt.character_id,
        &opt.platform,
        &opt.mode,
        &start_time,
        opt.verbose,
    )
    .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error("Error Loading Activities", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    match data {
        Some(e) => match opt.output {
            Output::Default => {
                print_default(e, opt.display_limit, opt.mode, opt.start_moment, start_time);
            }
            Output::Tsv => {
                println!("TSV OUTPUT GOES HERE TAB TAB TAB");
            }
        },
        None => {
            println!("No activities found.");
        }
    };
}
