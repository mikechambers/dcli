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

use chrono::{DateTime, Datelike, Duration, Local, Utc};
use dcli::error::Error;
use dcli::mode::Mode;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::response::activities::Activity;
use dcli::standing::Standing;
use dcli::statscontainer::ActivityStatsContainer;

use dcli::manifestinterface::ManifestInterface;
use std::path::PathBuf;

use dcli::utils::{
    f32_are_equal, format_f32, repeat_str, uppercase_first_char, TSV_DELIM, TSV_EOL,
};
use dcli::{apiinterface::ApiInterface, utils::EXIT_FAILURE};

use structopt::StructOpt;

//use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};

fn parse_and_validate_mode(src: &str) -> Result<Mode, String> {
    let mode = Mode::from_str(src)?;

    if !mode.is_crucible() {
        return Err(format!("Unsupported mode specified : {}", src));
    }

    Ok(mode)
}

fn parse_and_validate_moment(src: &str) -> Result<Moment, String> {
    let moment = Moment::from_str(src)?;

    //note, we positive capture what we want in case new properties
    //are added in the future
    match moment {
        Moment::Daily => {}
        Moment::Weekend => {}
        Moment::Weekly => {}
        Moment::Day => {}
        Moment::Week => {}
        Moment::Month => {}
        Moment::AllTime => {}
        Moment::Custom => {}
        _ => {
            return Err(format!("Unsupported moment specified : {}", src));
        }
    };

    Ok(moment)
}

async fn get_manifest(manifest_path: PathBuf) -> Result<ManifestInterface, Error> {
    //TODO: may need to make this mutable
    let manifest = ManifestInterface::new(manifest_path, false).await?;

    Ok(manifest)
}

async fn print_tsv(
    manifest_path: PathBuf,
    data: ActivityStatsContainer,
    mode: Mode,
    moment: Moment,
    start_time: DateTime<Utc>,
) -> Result<(), Error> {
    let mut manifest = get_manifest(manifest_path).await?;

    print!(
        "VAR{delim}START_TIME{delim}{start_time}{eol}",
        start_time = start_time.to_rfc3339(),
        delim = TSV_DELIM,
        eol = TSV_EOL,
    );

    print!(
        "VAR{delim}MOMENT{delim}{moment}{eol}",
        moment = format!("{}", moment),
        delim = TSV_DELIM,
        eol = TSV_EOL,
    );

    print!(
        "VAR{delim}MODE{delim}{mode}{eol}",
        mode = format!("{}", mode),
        delim = TSV_DELIM,
        eol = TSV_EOL,
    );

    print!(
        "DATA_HEADER{delim}MODE{delim}MAP{delim}DATE{delim}RESULT{delim}KILLS{delim}\
        DEATHS{delim}ASSISTS{delim}OPP_DEFEATED{delim}KD{delim}KDA{delim}\
        EFFICIENCY{eol}",
        delim = TSV_DELIM,
        eol = TSV_EOL,
    );

    for activity in &data.activities {
        let map_name = match manifest
            .get_activity_definition(activity.details.reference_id)
            .await
        {
            Ok(e) => e.display_properties.name,
            Err(_e) => "Unknown".to_string(),
        };

        print!(
            "DATA_ROW{delim}{mode}{delim}{map}{delim}{date}{delim}{result}{delim}\
            {kills}{delim}{deaths}{delim}{assists}{delim}{opp_defeated}{delim}\
            {kd}{delim}{kda}{delim}{eff}{eol}",
            mode = activity.details.mode,
            map = map_name,
            date = activity.period.to_rfc3339(),
            result = activity.values.standing,
            kills = activity.values.kills,
            deaths = activity.values.deaths,
            assists = activity.values.assists,
            opp_defeated = activity.values.opponents_defeated,
            kd = format_f32(activity.values.kills_deaths_ratio, 2),
            kda = format_f32(activity.values.kills_deaths_assists, 2),
            eff = format_f32(activity.values.efficiency, 2),
            eol = TSV_EOL,
            delim = TSV_DELIM,
        );
    }

    print!(
        "SUMMARY_HIGHS{delim}{delim}{delim}{delim}{result}{delim}\
        {kills}{delim}{deaths}{delim}{assists}{delim}{opp_defeated}{delim}\
        {kd}{delim}{kda}{delim}{eff}{eol}",
        result = format!("{}:{}", data.wins(), data.losses()),
        kills = data.highest_kills(),
        deaths = data.highest_deaths(),
        assists = data.highest_assists(),
        opp_defeated = data.highest_opponents_defeated(),
        kd = format_f32(data.highest_kills_deaths_ratio(), 2),
        kda = format_f32(data.highest_kills_deaths_assists(), 2),
        eff = format_f32(data.highest_efficiency(), 2),
        eol = TSV_EOL,
        delim = TSV_DELIM,
    );

    print!(
        "SUMMARY_PER_GAME{delim}{delim}{delim}{delim}{result}{delim}\
        {kills}{delim}{deaths}{delim}{assists}{delim}{opp_defeated}{delim}\
        {kd}{delim}{kda}{delim}{eff}{eol}",
        result = format_f32(data.win_percentage(), 2),
        kills = format_f32(data.per_activity_average(data.kills()), 2),
        deaths = format_f32(data.per_activity_average(data.deaths()), 2),
        assists = format_f32(data.per_activity_average(data.assists()), 2),
        opp_defeated = format_f32(data.per_activity_average(data.opponents_defeated()), 2),
        kd = format_f32(data.kills_deaths_ratio(), 2),
        kda = format_f32(data.kills_deaths_assists(), 2),
        eff = format_f32(data.efficiency(), 2),
        eol = TSV_EOL,
        delim = TSV_DELIM,
    );

    Ok(())
}

async fn print_default(
    manifest_path: PathBuf,
    data: ActivityStatsContainer,
    display_limit: i32,
    mode: Mode,
    moment: Moment,
    start_time: DateTime<Utc>,
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

    let local = start_time.with_timezone(&Local);
    let format_str = if Utc::now() - start_time > Duration::days(6) {
        "%B %-d, %Y"
    } else {
        if local.day() == Local::now().day() {
            "Today at %-I:%M %p"
        } else {
            "%A at %-I:%M %p"
        }
    };

    let start_time_label = local.format(format_str);

    let title = format!(
        "{mode} activities since {start_time} ({moment})",
        mode = uppercase_first_char(&format!("{}", mode)),
        start_time = start_time_label,
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
    //let highest_flag: &str = "^";
    let highest_flag : &str = "^";
    

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

        let highest_kills_flag = if f32_are_equal(activity.values.kills, data.highest_kills()) {
            highest_flag
        } else {
            ""
        };

        let highest_assists_flag = if f32_are_equal(activity.values.assists, data.highest_assists())
        {
            highest_flag
        } else {
            ""
        };

        let highest_deaths_flag = if f32_are_equal(activity.values.deaths, data.highest_deaths()) {
            highest_flag
        } else {
            ""
        };

        let highest_opponents_defeated_flag = if f32_are_equal(
            activity.values.opponents_defeated,
            data.highest_opponents_defeated(),
        ) {
            highest_flag
        } else {
            ""
        };

        let highest_efficiency_flag =
            if f32_are_equal(activity.values.efficiency, data.highest_efficiency()) {
                highest_flag
            } else {
                ""
            };

        let highest_highest_kills_deaths_ratio_flag = if f32_are_equal(
            activity.values.kills_deaths_ratio,
            data.kills_deaths_ratio(),
        ) {
            highest_flag
        } else {
            ""
        };

        let highest_kills_deaths_assists_flag = if f32_are_equal(
            activity.values.kills_deaths_assists,
            data.highest_kills_deaths_assists(),
        ) {
            highest_flag
        } else {
            ""
        };

        println!(
            "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            map_name,
            format!("{}", activity.values.standing),
            format!("{}", streak),
            format!("{b}{a}", b=highest_kills_flag, a=activity.values.kills),
            format!("{b}{a}", b=highest_assists_flag, a=activity.values.assists),
            format!("{b}{a}", b=highest_opponents_defeated_flag, a=activity.values.opponents_defeated),
            format!("{b}{a}", b=highest_deaths_flag, a=activity.values.deaths),
            format!("{b}{a}", b=highest_highest_kills_deaths_ratio_flag, a=format_f32(activity.values.kills_deaths_ratio, 2)),
            format!("{b}{a}", b=highest_kills_deaths_assists_flag, a=format_f32(activity.values.kills_deaths_assists, 2)),
            format!("{b}{a}", b=highest_efficiency_flag, a=format_f32(activity.values.efficiency, 2)),
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
    "",
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
    println!("{}-highest overall", highest_flag);
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
/// Enables control of which stats are retrieved based on game mode, moment
/// from which to retrieve them (to present) and character.
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
    /// Required when --moment is set to custom, but otherwise not applicable.
    #[structopt(short = "t", long = "custom-time", parse(try_from_str = parse_rfc3339), required_if("start-moment", "custom"))]
    custom_time: Option<DateTime<Utc>>,

    /// Start moment from which to pull activities from
    ///
    /// Activities will be retrieved from moment to the current time.
    /// For example, Specifying: --moment weekly
    ///
    /// will return all activities since the last weekly reset on Tuesday.
    ///
    /// Valid values include daily (last daily reset), weekend
    /// (last weekend reset on Friday), weekly (last weekly reset on Tuesday),
    /// day (last day), week (last week), month (last month), all_time and custom.
    ///
    /// When custom is specified, the custom start date in RFC3339 format must
    /// be specified with the --custom-time argument.
    ///
    /// For example:
    /// --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00
    ///
    /// Specifying all_time retrieves all activitiy history and may take an extended
    /// amount of time to retrieve depending on the number of activities.
    #[structopt(long = "moment", parse(try_from_str=parse_and_validate_moment), 
        short = "s", default_value = "day")]
    moment: Moment,

    /// Activity mode to return stats for
    ///
    /// Supported values are all_pvp (default), control, clash, elimination,
    /// mayhem, iron_banner, private, rumble, pvp_competitive,
    /// quickplay and trials_of_osiris.
    ///
    /// Addition values available are crimsom_doubles, supremacy, survival,
    /// countdown, all_doubles, doubles, private_matches_clash, private_matches_control,
    /// private_matches_survival, private_matches_rumble, showdown, lockdown,
    /// scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine
    #[structopt(long = "mode", short = "M", 
        parse(try_from_str=parse_and_validate_mode), default_value = "all_pvp")]
    mode: Mode,

    /// Limit the number of activity details that will be displayed.
    ///
    /// Summary information will be generated based on all activities. Ignored if
    /// --output-format is tsv.
    #[structopt(long = "limit", short = "L", default_value = "10")]
    display_limit: i32,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value or column
    /// pairs with lines ending in a new line character (\n).
    #[structopt(short = "O", long = "output-format", default_value = "default")]
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
    #[structopt(short = "P", long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,
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

    eprintln!(
        "Retrieving activities for {}. This may take a moment...",
        &opt.mode
    );
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
            match print_tsv(
                opt.manifest_path,
                container,
                opt.mode,
                opt.moment,
                custom_time,
            )
            .await
            {
                Ok(_e) => {}
                Err(e) => {
                    print_error(
                        "Error generatating output. (Check the --manifest-path) : ",
                        e,
                    );
                    std::process::exit(EXIT_FAILURE);
                }
            };
        }
    }
}
