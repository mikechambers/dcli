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

use std::str::FromStr;

use dcli::apiinterface::ApiInterface;
use dcli::enums::mode::Mode;
use dcli::enums::moment::{Moment, MomentPeriod};
use dcli::enums::platform::Platform;
use dcli::error::Error;
use dcli::output::Output;
use dcli::response::stats::{DailyPvPStatsValuesData, PvpStatsData};
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{
    build_tsv, format_f32, human_duration, print_error, print_verbose,
    repeat_str,
};
use structopt::StructOpt;

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
        _ => {
            return Err(format!("Unsupported moment specified : {}", src));
        }
    };

    Ok(moment)
}

fn print_tsv(
    data: PvpStatsData,
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &Mode,
    period: &MomentPeriod,
) {
    let mut name_values: Vec<(&str, String)> = Vec::new();

    name_values.push(("member_id", member_id.to_string()));
    name_values.push(("platform", format!("{}", platform)));
    name_values.push(("platform_id", format!("{}", platform.to_id())));
    name_values.push(("character_id", character_id.to_string()));

    name_values.push(("start_moment_dt", format!("{}", period.start)));
    name_values.push(("end_moment_dt", format!("{}", period.end)));

    name_values.push(("moment_human", format!("{}", period.moment)));
    name_values.push(("mode", format!("{}", mode)));
    name_values.push(("mode_id", format!("{}", mode.to_id())));
    name_values
        .push(("activities_entered", format!("{}", data.activities_entered)));
    name_values.push(("activities_won", format!("{}", data.activities_won)));
    name_values
        .push(("activities_lost", format!("{}", data.get_activities_lost())));
    name_values.push(("assists", format!("{}", data.assists)));
    name_values.push(("kills", format!("{}", data.kills)));
    name_values.push((
        "average_kill_distance",
        format!("{}", data.average_kill_distance),
    ));
    name_values.push((
        "total_kill_distance",
        format!("{}", data.total_kill_distance),
    ));
    name_values.push(("seconds_played", format!("{}", data.seconds_played)));

    name_values.push((
        "human_time_played",
        human_duration(data.seconds_played as u32),
    ));

    name_values.push(("deaths", format!("{}", data.deaths)));
    name_values
        .push(("average_lifespan", format!("{}", data.average_lifespan)));

    name_values.push((
        "human_average_lifespan",
        human_duration(data.average_lifespan as u32),
    ));

    name_values.push((
        "total_lifespan",
        format!("{}", data.get_total_lifespan() as u32),
    ));
    name_values
        .push(("opponents_defeated", format!("{}", data.opponents_defeated)));
    name_values.push(("efficiency", format!("{}", data.efficiency)));
    name_values
        .push(("kills_deaths_ratio", format!("{}", data.kills_deaths_ratio)));
    name_values.push((
        "kills_deaths_assists",
        format!("{}", data.kills_deaths_assists),
    ));
    name_values.push(("suicides", format!("{}", data.suicides)));
    name_values.push(("precision_kills", format!("{}", data.precision_kills)));

    let best_single_game_kills = data.best_single_game_kills.unwrap_or(0.0);

    name_values.push((
        "best_single_game_kills",
        format!("{}", best_single_game_kills),
    ));

    print!("{}", build_tsv(name_values));
}

//TODO: should pass in by reference here
fn print_default(data: PvpStatsData, mode: Mode, moment: Moment) {
    let p = format_f32;

    let moment_string = match moment {
        Moment::Daily => "since the daily reset",
        Moment::Weekend => "since last Friday",
        Moment::Weekly => "since the weekly reset",
        Moment::Day => "for the last day",
        Moment::Week => "for the last week",
        Moment::Month => "for the last month",
        Moment::AllTime => "for all time",
        _ => "",
    };

    let title: String =
        format!("Destiny 2 stats for {:#} {}", mode, moment_string);

    println!();
    println!("{}", title);
    println!("{}", repeat_str("=", title.chars().count()));

    println!(
        "Time played is {}",
        human_duration(data.seconds_played as u32)
    );
    println!(
        "{wins} wins and {losses} losses for a {win_percentage}% win rate",
        wins = data.activities_won,
        losses = data.get_activities_lost(),
        win_percentage =
            p((data.activities_won / data.activities_entered) * 100.0, 2),
    );

    println!();
    println!();

    let col_w = 12;

    println!(
        "{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}",
        "",
        "K/D",
        "KD/A",
        "EFFICIENCY",
        "KILLS",
        "ASSISTS",
        "DEFEATS",
        "DEATHS",
        "SUICIDES",
        col_w = col_w
    );
    println!("{}", repeat_str("-", col_w * 9));
    println!(
        "{t:>0col_w$}{sp:<0col_w$}{sp:<0col_w$}{sp:<0col_w$}{k:<0col_w$}{a:<0col_w$}{o:<0col_w$}{d:<0col_w$}{s:<0col_w$}",
        t = "PER GAME  ",
        sp = "",
        k = p(data.kills / data.activities_entered, 2),
        o = p(data.opponents_defeated / data.activities_entered, 2),
        a = p(data.assists / data.activities_entered, 2),
        d = p(data.deaths / data.activities_entered, 2),
        s = p(data.suicides / data.activities_entered, 2),
        col_w = col_w,
    );

    //
    println!(
        "{t:>0col_w$}{kd:<0col_w$}{kda:<0col_w$}{e:<0col_w$}{k:<0col_w$}{a:<0col_w$}{o:<0col_w$}{d:<0col_w$}{s:<0col_w$}",
        t = "TOTAL  ",
        kd = p(data.kills_deaths_ratio, 2),
        kda = p(data.kills_deaths_assists, 2),
        e = p(data.efficiency, 2),
        k = p(data.kills, 0),
        o = p(data.opponents_defeated, 0),
        a = p(data.assists, 0),
        d = p(data.deaths, 0),
        s = p(data.suicides, 0),
        col_w = col_w,
    );

    println!();
    println!();

    println!("You have had an average life span of {lifespan} with an average kill distance of {kill_distance} meters. {precision_percent}% of your kills were precision kills.",
        lifespan = human_duration(data.average_lifespan as u32),
        kill_distance = p(data.average_kill_distance, 2),
        precision_percent = p((data.precision_kills / data.kills) * 100.0, 2),
    );
    println!();
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving historic Destiny 2 Crucible activity stats.
///
/// Retrieves stats based on the moment specified, up to, but excluding the current day.
/// Enables control of which stats are retrieved via game mode, past time moment and
/// character.
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
    /// This is not the user name, but the member id
    /// retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Time range to pull stats from
    ///
    /// Valid values include day (last day), daily (since last daily reset),
    /// week (last week), weekly (since last weekly reset on Tuesday), month
    /// (last month), weekend (since last Friday reset) and all_time.
    ///
    /// All ranges are up to, but not including current day, and thus some values
    /// may not return data depending on time of day.
    #[structopt(long = "moment", parse(try_from_str=parse_and_validate_moment),
    short="T", default_value = "all_time")]
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
    #[structopt(short = "M", long = "mode", default_value = "all_pvp")]
    mode: Mode,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(
        short = "O",
        long = "output-format",
        default_value = "default"
    )]
    output: Output,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id. If not specified, data for all characters
    /// will be returned.
    ///
    /// Required unless moment is set to all_time
    #[structopt(short = "c", long = "character-id", required_ifs=&[("moment","day"),
        ("moment","reset"),("moment","week"),("moment","month"),])]
    character_id: Option<String>,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

async fn retrieve_all_time_stats(
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &Mode,
    verbose: bool,
) -> Result<Option<PvpStatsData>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose)?;

    let data: PvpStatsData = match client
        .retrieve_alltime_crucible_stats(
            &member_id,
            &character_id,
            &platform,
            &mode,
        )
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    Ok(Some(data))
}

async fn retrieve_aggregate_crucible_stats(
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &Mode,
    period: &MomentPeriod,
    verbose: bool,
) -> Result<Option<PvpStatsData>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose)?;

    let data: Vec<DailyPvPStatsValuesData> = match client
        .retrieve_aggregate_crucible_stats(
            &member_id,
            &character_id,
            &platform,
            &mode,
            period,
        )
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    let mut p_stats: PvpStatsData = PvpStatsData::default();

    for d in data.iter() {
        p_stats = d.values + p_stats;
    }

    Ok(Some(p_stats))
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    //use unwrap_or_else as it is lazily evaluated
    let character_id: String =
        opt.character_id.unwrap_or_else(|| "0".to_string());

    //TODO: probably need to pass a reference here and then clone it.
    let moment_period = MomentPeriod::from_moment(opt.moment);
    let data = match opt.moment {
        Moment::AllTime => {
            match retrieve_all_time_stats(
                &opt.member_id,
                &character_id,
                &opt.platform,
                &opt.mode,
                opt.verbose,
            )
            .await
            {
                Ok(e) => match e {
                    Some(e) => e,
                    None => {
                        println!("No results found");
                        return;
                    }
                },
                Err(e) => {
                    print_error("Error Retrieving All Time Data", e);
                    std::process::exit(EXIT_FAILURE);
                }
            }
        }
        _ => {
            match retrieve_aggregate_crucible_stats(
                &opt.member_id,
                &character_id,
                &opt.platform,
                &opt.mode,
                &moment_period,
                opt.verbose,
            )
            .await
            {
                Ok(e) => match e {
                    Some(e) => e,
                    None => {
                        println!("No results found");
                        return;
                    }
                },
                Err(e) => {
                    print_error("Error Retrieving Daily Data", e);
                    std::process::exit(EXIT_FAILURE);
                }
            }
        }
    };

    match opt.output {
        Output::Default => {
            print_default(data, opt.mode, opt.moment);
        }
        Output::Tsv => {
            print_tsv(
                data,
                &opt.member_id,
                &character_id,
                &opt.platform,
                &opt.mode,
                &moment_period,
            );
        }
    }
}
