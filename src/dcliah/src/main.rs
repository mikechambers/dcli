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

use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Datelike, Duration, Local, Utc};
use dcli::enums::moment::Moment;
use dcli::enums::platform::Platform;
use dcli::enums::standing::Standing;
use dcli::manifestinterface::ManifestInterface;
use dcli::output::Output;
use dcli::{
    crucible::{CruciblePlayerPerformance, CruciblePlayerPerformances},
    enums::mode::Mode,
    utils::{calculate_ratio, human_duration},
};

use dcli::activitystoreinterface::ActivityStoreInterface;

use dcli::utils::{determine_data_dir, format_f32, repeat_str, uppercase_first_char};
//use dcli::utils::EXIT_FAILURE;
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};
use structopt::StructOpt;

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

/*
async fn print_tsv(
    manifest_dir: &PathBuf,
    data: ActivityStatsContainer,
    mode: Mode,
    moment: Moment,
    start_time: DateTime<Utc>,
) -> Result<(), Error> {
    let mut manifest = get_manifest(manifest_dir).await?;

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
        kills = format_f32(data.stat_per_game(data.kills()), 2),
        deaths = format_f32(data.stat_per_game(data.deaths()), 2),
        assists = format_f32(data.stat_per_game(data.assists()), 2),
        opp_defeated = format_f32(data.stat_per_game(data.opponents_defeated()), 2),
        kd = format_f32(data.kills_deaths_ratio(), 2),
        kda = format_f32(data.kills_deaths_assists(), 2),
        eff = format_f32(data.efficiency(), 2),
        eol = TSV_EOL,
        delim = TSV_DELIM,
    );

    Ok(())
}
*/
fn print_default(
    data: &CruciblePlayerPerformances,
    activity_limit: &u32,
    mode: &Mode,
    moment: &Moment,
    start_time: &DateTime<Utc>,
    weapon_count: &u32,
) {
    //todo: might want to look at buffering output
    //https://rust-cli.github.io/book/tutorial/output.html

    let performances = data.get_performances();
    let activity_count = performances.len();

    let display_count = std::cmp::min(activity_count, *activity_limit as usize);
    let is_limited = activity_count != display_count;

    let local = start_time.with_timezone(&Local);
    let format_str = if Utc::now() - *start_time > Duration::days(6) {
        "%B %-d, %Y"
    } else if local.day() == Local::now().day() {
        "Today at %-I:%M %p"
    } else {
        "%A at %-I:%M %p"
    };

    let start_time_label = local.format(format_str);

    println!();
    println!();

    let title = format!(
        "{mode} activities since {start_time} ({moment})",
        mode = uppercase_first_char(&format!("{}", mode)),
        start_time = start_time_label,
        moment = moment,
    );

    println!();
    println!();
    println!("ACTIVITIES");
    println!("==================");
    println!("{}", title);
    println!(
        "Total time played is {}",
        human_duration(data.time_played_seconds)
    );
    println!();

    if is_limited {
        println!(
            "Displaying details for the last {display_count} of {activity_count} activities",
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

    let col_w = 10;
    let map_col_w = 18;
    let str_col_w = 10;

    //TODO: maybe format this to yellow background
    let header = format!(
        "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
        "MAP",
        "W/L",
        "STREAK",
        "KILLS",
        "ASTS",
        "K+A",
        "DEATHS",
        "K/D",
        "KD/A",
        "EFF",
        "SUPERS",
        "GRENADES",
        col_w = col_w,
        map_col_w = map_col_w,
        str_col_w=str_col_w,
    );
    println!("{}", header);
    let header_divider = repeat_str(&"=", header.chars().count());
    println!("{}", header_divider);

    let slice: &[CruciblePlayerPerformance] = if is_limited {
        println!(
            "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            "...", "...", "...", "...", "...", "...", "...","...","...","...","...","...",
            col_w = col_w,
            map_col_w = map_col_w,
            str_col_w=str_col_w,
        );

        &performances[..*activity_limit as usize]
    } else {
        &performances[..]
    };

    let mut last_mode = Mode::None;
    let mut streak: i32 = 0;
    let mut last_standing: Standing = Standing::Unknown;

    for activity in slice.iter().rev() {
        if activity.activity_detail.mode != last_mode {
            println!();
            println!("{}", activity.activity_detail.mode);
            println!("{}", repeat_str(&"-", col_w + map_col_w));
            last_mode = activity.activity_detail.mode;
        }

        let standing = activity.stats.standing;
        if standing == last_standing {
            streak = match last_standing {
                Standing::Unknown => 0,
                Standing::Victory => streak + 1,
                Standing::Defeat => streak - 1,
            };
        } else {
            last_standing = standing;
            streak = match last_standing {
                Standing::Unknown => 0,
                Standing::Victory => 1,
                Standing::Defeat => -1,
            };
        }

        let mut map_name = activity.activity_detail.map_name.clone();

        //todo: move this into reusable util function
        if map_name.chars().count() > map_col_w - 1 {
            map_name = map_name[..(col_w - 3)].to_string();
            map_name.push_str("..")
        }

        let extended = activity.stats.extended.as_ref().unwrap();
        let supers = extended.weapon_kills_super;
        let grenades = extended.weapon_kills_grenade;

        println!(
            "{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            map_name,
            format!("{}", activity.stats.standing),
            format!("{}", streak),
            format!("{a}", a=activity.stats.kills),
            format!("{a}", a=activity.stats.assists),
            format!("{a}", a=activity.stats.opponents_defeated),
            format!("{a}", a=activity.stats.deaths),
            format!("{a}", a=format_f32(activity.stats.kills_deaths_ratio, 2)),
            format!("{a}", a=format_f32(activity.stats.kills_deaths_assists, 2)),
            format!("{a}", a=format_f32(activity.stats.efficiency, 2)),
            supers.to_string(),
            grenades.to_string(),
            col_w = col_w,
            map_col_w=map_col_w,
            str_col_w=str_col_w,
        );
    }

    let extended = data.extended.as_ref().unwrap();
    println!("{}", repeat_str(&"-", header.chars().count()));

    println!("{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "HIGHS",
    format!("{}-{}", data.wins, data.losses),
    format!("{}W {}L", data.longest_win_streak, data.longest_loss_streak),
    format!("{}", data.highest_kills),
    format!("{}", data.highest_assists),
    format!("{}", data.highest_opponents_defeated),
    format!("{}", data.highest_deaths),

    format_f32(data.highest_kills_deaths_ratio, 2),
    format_f32(data.highest_kills_deaths_assists, 2),
    format_f32(data.highest_efficiency, 2),
    format!("{}", extended.highest_weapon_kills_super),
    format!("{}", extended.highest_weapon_kills_grenade),

    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    );

    println!("{:<0map_col_w$}{:<0col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "PER GAME",
    format!("{}% w", format_f32(data.win_rate, 2)),
    "",
    format_f32(data.stat_per_game(data.kills), 2),
    format_f32(data.stat_per_game(data.assists), 2),
    format_f32(data.stat_per_game(data.opponents_defeated), 2),
    format_f32(data.stat_per_game(data.deaths), 2),
    format_f32(data.kills_deaths_ratio, 2),
    format_f32(data.kills_deaths_assists, 2),
    format_f32(data.efficiency, 2),
    format_f32(data.stat_per_game(extended.weapon_kills_super), 2),
    format_f32(data.stat_per_game(extended.weapon_kills_grenade), 2),
    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    );

    println!("{}", header_divider);
    println!("{}", header);

    println!();
    println!();

    let wep_col = map_col_w + col_w;
    let wep_header_str = format!(
        "{:<0map_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0map_col_w$}",
        "WEAPON",
        "KILLS",
        "GAMES",
        "K/G",
        "PREC",
        "%",
        "TYPE",
        col_w = col_w,
        map_col_w = wep_col,
    );
    let wep_divider = repeat_str(&"=", wep_header_str.chars().count());

    println!("{}", wep_header_str);
    println!("{}", wep_divider);

    let max_weps = std::cmp::min(*weapon_count as usize, extended.weapons.len());

    for w in &extended.weapons[..max_weps] {
        println!(
            "{:<0map_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0map_col_w$}",
            w.weapon.name,
            w.kills.to_string(),
            w.activity_count.to_string(),
            format_f32(calculate_ratio(w.kills, w.activity_count), 2),
            w.precision_kills.to_string(),
            format_f32(w.precision_kills_percent, 2),
            format!("{}", w.weapon.item_sub_type),
            col_w = col_w,
            map_col_w = wep_col,
        );
    }
    println!();
}

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
        short = "T", default_value = "week")]
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
    #[structopt(long = "activity-limit", short = "L", default_value = "10")]
    activity_limit: u32,

    /// The number of weapons to display details for.
    #[structopt(long = "weapon-count", short = "w", default_value = "5")]
    weapon_count: u32,

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

    /// Directory where Destiny 2 manifest database file is stored. (optional)
    ///
    /// This will normally be downloaded using the dclim tool, and stored in a file
    /// named manifest.sqlite3 (in the manifest directory specified when running
    /// dclim).
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let data_dir = match determine_data_dir(opt.data_dir) {
        Ok(e) => e,
        Err(e) => {
            print_error("Error initializing manifest directory.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let start_time = match opt.moment {
        Moment::Custom => {
            opt.custom_time.unwrap() //note, this should be ok, because struct opt should ensure valid value
        }
        _ => opt.moment.get_date_time(),
    };

    let mut store = match ActivityStoreInterface::init_with_path(&data_dir, opt.verbose).await {
        Ok(e) => e,
        Err(e) => {
            print_error(
                "Could not initialize activity store. Have you run dclias?",
                e,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut manifest = match ManifestInterface::new(&data_dir, false).await {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not initialize manifest. Have you run dcliam?", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    match store
        .sync(&opt.member_id, &opt.character_id, &opt.platform)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not sync activity store.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let data = match store
        .retrieve_activities_since(
            &opt.member_id,
            &opt.character_id,
            &opt.platform,
            &opt.mode,
            &start_time,
            &mut manifest,
        )
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not retrieve data from activity store.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    if data.is_none() {
        //TODO: TSV Output
        println!("No activities found.");
        return;
    }

    let data = data.unwrap();

    if data.get_performances().is_empty() {
        //TODO: TSV Output
        println!("No activities found.");
        return;
    }

    /*
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

    let container = match data {
        Some(e) => e,
        None => {
            println!("No activities found.");
            return;
        }
    };
    */

    match opt.output {
        Output::Default => {
            print_default(
                &data,
                &opt.activity_limit,
                &opt.mode,
                &opt.moment,
                &start_time,
                &opt.weapon_count,
            );
        }
        Output::Tsv => {
            /*
            match print_tsv(&data_dir, container, opt.mode, opt.moment, custom_time).await {
                Ok(_e) => {}
                Err(e) => {
                    print_error(
                        "Error generatating output. (Check the --manifest-path) : ",
                        e,
                    );
                    std::process::exit(EXIT_FAILURE);
                }
            };
            */
        }
    }
}
