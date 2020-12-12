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

use dcli::moment::Moment;
use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use dcli::error::Error;
use dcli::mode::Mode;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::response::activities::Activity;
use dcli::standing::Standing;
use dcli::statscontainer::ActivityStatsContainer;

use dcli::manifestinterface::ManifestInterface;
use std::path::PathBuf;

use dcli::utils::{format_f32, repeat_str, uppercase_first_char};
use dcli::{apiinterface::ApiInterface, utils::EXIT_FAILURE};

use structopt::StructOpt;

//use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};

/*
fn print_tsv(
    data: ActivityStatsContainer,
    display_limit:i32,
    moment: Moment,
) {
    let mut name_values: Vec<(&str, String)> = Vec::new();

    name_values.push(("member_id", member_id.to_string()));
    print!("{}", build_tsv(name_values));
}
*/


fn parse_and_validate_moment(src: &str) -> Result<Moment, String> {
    let moment = Moment::from_str(src)?; 

    //note, we positive capture what we want in case new properties
    //are added in the future
    match moment {
        Moment::Daily => {}
        Moment::Weekend => {},
        Moment::Weekly => {},
        Moment::Day => {},
        Moment::Week => {},
        Moment::Month => {}
        Moment::AllTime => {},
        Moment::Custom => {},
        _ => {
            return Err(format!("Unsupported moment specified : {}", src));
        },
    };

    Ok(moment)
}



async fn get_manifest(manifest_path: PathBuf) -> Result<ManifestInterface, Error> {
    //TODO: may need to make this mutable
    let manifest = ManifestInterface::new(manifest_path, false).await?;

    Ok(manifest)
}

async fn print_default(
    manifest_path: PathBuf,
    data: ActivityStatsContainer,
    display_limit: i32,
    mode: Mode,
    moment: Moment,
    date_time: DateTime<Utc>,
) -> Result<(), Error> {
    //todo: might want to look at buffering output
    //https://rust-cli.github.io/book/tutorial/output.html

    let activity_count = data.activities.len();

    if activity_count == 0 {
        println!("No activities found.");
        return Ok(());
    }

    let mut manifest = get_manifest(manifest_path).await?;

    let display_count = std::cmp::min(activity_count, display_limit as usize);
    let is_limited = activity_count != display_count;

    let date_time_label = if Utc::now() - date_time > Duration::days(6) {
        date_time.format("%B %-d, %Y")
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
                if display_count == 1 {
                    "y"
                } else {
                    "ies"
                }
            },
        );
    }
    println!();

    let col_w = 8;
    let map_col_w = 18;
    let str_col_w = 10;

    //TODO: maybe format this to yellow background
    let header = format!(
        "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
        "MAP",
        "W / L",
        "STREAK",
        "KILLS",
        "ASTS",
        "K+A",
        "DEATHS",
        "K/D",
        "KD/A",
        "EFF",
        col_w = col_w,
        map_col_w = map_col_w,
        str_col_w=str_col_w,
    );
    println!("{}", header);
    let header_divider = repeat_str(&"=", header.chars().count());
    println!("{}", header_divider);

    let slice: &[Activity] = if is_limited {
        println!(
            "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            "...", "...", "...", "...", "...", "...", "...","...","...","...",
            col_w = col_w,
            map_col_w = map_col_w,
            str_col_w=str_col_w,
        );

        &data.activities[..display_limit as usize]
    } else {
        &data.activities[..]
    };

    let mut last_mode = Mode::None;
    let mut streak: i32 = 0;
    let mut last_standing: Standing = Standing::Unknown;
    let highest_flag: &str = "^";

    for activity in slice.iter().rev() {
        if activity.details.mode != last_mode {
            println!();
            println!("{}", activity.details.mode);
            println!("{}", repeat_str(&"-", col_w + map_col_w));
            last_mode = activity.details.mode;
        }

        if activity.values.standing == last_standing {
            streak = match last_standing {
                Standing::Unknown => 0,
                Standing::Victory => streak + 1,
                Standing::Defeat => streak - 1,
            };
        } else {
            last_standing = activity.values.standing;
            streak = match last_standing {
                Standing::Unknown => 0,
                Standing::Victory => 1,
                Standing::Defeat => -1,
            };
        }

        //todo: we can get better mode name from director_activity_hash i.e. Control as opposed to Control Quickplay
        let mut map_name = match manifest
            .get_activity_definition(activity.details.reference_id)
            .await
        {
            Ok(e) => e.display_properties.name,
            Err(_e) => "Unknown".to_string(),
        };

        //todo: move this into reusable util function
        if map_name.chars().count() > map_col_w - 1 {
            map_name = map_name[..(col_w - 3)].to_string();
            map_name.push_str("..")
        }

        let highest_kills_flag = if activity.values.kills == data.highest_kills() {
            highest_flag
        } else {
            ""
        };

        let highest_assists_flag = if activity.values.assists == data.highest_assists() {
            highest_flag
        } else {
            ""
        };

        let highest_deaths_flag = if activity.values.deaths == data.highest_deaths() {
            highest_flag
        } else {
            ""
        };

        let highest_opponents_defeated_flag =
            if activity.values.opponents_defeated == data.highest_opponents_defeated() {
                highest_flag
            } else {
                ""
            };

        let highest_efficiency_flag = if activity.values.efficiency == data.highest_efficiency() {
            highest_flag
        } else {
            ""
        };

        let highest_highest_kills_deaths_ratio_flag =
            if activity.values.kills_deaths_ratio == data.kills_deaths_ratio() {
                highest_flag
            } else {
                ""
            };

        let highest_kills_deaths_assists_flag =
            if activity.values.kills_deaths_assists == data.highest_kills_deaths_assists() {
                highest_flag
            } else {
                ""
            };

        println!(
            "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            map_name,
            format!("{}", activity.values.standing),
            format!("{}", streak),
            format!("{}{}", highest_kills_flag, activity.values.kills),
            format!("{}{}", highest_assists_flag, activity.values.assists),
            format!("{}{}", highest_opponents_defeated_flag, activity.values.opponents_defeated),
            format!("{}{}", highest_deaths_flag, activity.values.deaths),
            format!("{}{}", highest_highest_kills_deaths_ratio_flag, format_f32(activity.values.kills_deaths_ratio, 2)),
            format!("{}{}", highest_kills_deaths_assists_flag, format_f32(activity.values.kills_deaths_assists, 2)),
            format!("{}{}", highest_efficiency_flag, format_f32(activity.values.efficiency, 2)),
            col_w = col_w,
            map_col_w=map_col_w,
            str_col_w=str_col_w,
        );
    }

    println!("{}", repeat_str(&"-", header.chars().count()));

    println!("{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "HIGHS",
    format!("{}-{}", data.wins(), data.losses()),
    format!("{}W {}L", data.longest_win_streak(), data.longest_loss_streak()),
    format!("{}", data.highest_kills()),
    format!("{}", data.highest_assists()),
    format!("{}", data.highest_opponents_defeated()),
    format!("{}", data.highest_deaths()),

    format_f32(data.highest_kills_deaths_ratio(), 2),
    format_f32(data.highest_kills_deaths_assists(), 2),
    format_f32(data.highest_efficiency(), 2),

    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
);

    println!("{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "PER GAME",
    format!("{}% w", format_f32(data.win_percentage(), 2)),
    format!("{}", ""),
    format_f32(data.per_activity_average(data.kills()), 2),
    format_f32(data.per_activity_average(data.assists()), 2),
    format_f32(data.per_activity_average(data.opponents_defeated()), 2),
    format_f32(data.per_activity_average(data.deaths()), 2),
    format_f32(data.kills_deaths_ratio(), 2),
    format_f32(data.kills_deaths_assists(), 2),
    format_f32(data.efficiency(), 2),
    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    );

    println!("{}", header_divider);
    println!("{}", header);

    println!();
    println!("{} - denotes highest over all", highest_flag);
    println!();

    manifest.close().await?;
    Ok(())
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
    mode: &Mode,
    custom_time: &DateTime<Utc>,
    verbose: bool,
) -> Result<Option<ActivityStatsContainer>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let activities: Vec<Activity> = match client
        .retrieve_activities_since(&member_id, &character_id, &platform, &mode, &custom_time)
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    //TODO: check if we get back and empty vector
    let container = ActivityStatsContainer::with_activities(activities);

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
    #[structopt(short = "t", long = "custom-time", parse(try_from_str = parse_rfc3339), required_if("start-moment", "custom"))]
    custom_time: Option<DateTime<Utc>>,

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
    /// be specified with the --custom-time argument.
    ///
    /// For example:
    /// --start-moment custom --custom-time 2020-12-08T17:00:00.774187+00:00
    ///
    /// Specifying alltime retrieves all activitiy history and may take an extended
    /// amount of time to retrieve depending on the number of activities.
    #[structopt(long = "moment", parse(try_from_str=parse_and_validate_moment), short = "s", default_value = "day")]
    moment: Moment,

    /// Activity mode to return stats for
    ///
    /// Valid values are all (default), control, clash, mayhem, ironbanner,
    /// private, rumble, comp, quickplay and trialsofosiris.
    #[structopt(long = "mode", short = "a", default_value = "all_pvp")]
    mode: Mode,

    /// Limit the number of activity details that will be displayed.
    ///
    /// Summary information will be generated based on all activities. Ignored if
    /// --output is tsv.
    #[structopt(long = "display-limit", short = "d", default_value = "10")]
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

    ///Local path for Destiny 2 manifest database file.
    #[structopt(short = "f", long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,
    //TODO: need to standardize on arg long / short names
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let custom_time = match opt.moment {
        Moment::Custom => {
            opt.custom_time.unwrap() //note, this should be ok, because struct opt should ensure valid value
        }
        _ => opt.moment.get_date_time(),
    };

    eprintln!("Retrieving activities. This may take a moment...");
    //todo: is there any need to send a reference to an enum?
    let data = match retrieve_activities_since(
        &opt.member_id,
        &opt.character_id,
        &opt.platform,
        &opt.mode,
        &custom_time,
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

    let container = match data {
        Some(e) => e,
        None => {
            println!("No activities found.");
            return;
        }
    };

    match opt.output {
        Output::Default => {
            match print_default(
                opt.manifest_path,
                container,
                opt.display_limit,
                opt.mode,
                opt.moment,
                custom_time,
            )
            .await
            {
                Ok(_e) => {}
                Err(e) => {
                    print_error("Error generating output. (Check the --manifest-path) : ", e);
                    std::process::exit(EXIT_FAILURE);
                }
            };
        }
        Output::Tsv => {
            println!("TSV OUTPUT GOES HERE TAB TAB TAB");
        }
    }
}
