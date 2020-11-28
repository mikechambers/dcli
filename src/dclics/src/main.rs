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

use structopt::StructOpt;

use dcli::apiinterface::ApiInterface;
use dcli::error::Error;
use dcli::mode::CrucibleMode;
use dcli::platform::Platform;
use dcli::response::stats::{DailyPvPStatsValuesData, PvpStatsData};
use dcli::timeperiod::TimePeriod;

use dcli::cruciblestats::CrucibleStats;
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_standard};

#[derive(StructOpt)]
/// Command line tool for retrieving current Destiny 2 activity for player.
///
///
struct Opt {
    /// Platform for specified id
    ///
    /// Platform for specified member id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Time range to pull stats from. Valid values include day, reset, week,
    /// month, alltime (default)
    ///
    /// Time range to pull stats from. Valid values include  day (last day),
    /// reset (since reset), week (last week), month (last month), alltime (default)
    #[structopt(long = "period")]
    period: Option<TimePeriod>,

    /// Destiny 2 API member id
    ///
    /// Destiny 2 API member id. This is not the user name, but the member id
    /// retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Crucible mode to return stats for.
    ///
    /// Crucible mode to return stats for. Valid values are all (default),
    /// control, clash, mayhem, ironbanner, private, trialsofnine, rumble,
    /// comp, quickplay and trialsofosiris
    #[structopt(long = "mode")]
    mode: Option<CrucibleMode>,

    /// Destiny 2 API character id. If not specified, data for all characters will be returned.
    /// Required when period is set to day, reset, week or month
    ///
    /// Destiny 2 API character id. If not specified, data for all characters will be returned.
    /// Required when period is set to day, reset, week or month
    #[structopt(short = "c", long = "character-id", required_ifs=&[("period","day"),("period","reset"),("period","week"),("period","month"),])]
    character_id: Option<String>,

    ///Terse output. Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    _terse: bool,

    ///Print out additional information
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

async fn retrieve_all_time_stats(
    member_id: String,
    character_id: String,
    platform: Platform,
    mode: CrucibleMode,
    verbose: bool,
) -> Result<CrucibleStats, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let data: PvpStatsData = client
        .retrieve_alltime_crucible_stats(member_id, character_id, platform, mode)
        .await?;

    let p_stats: CrucibleStats = data.get_crucible_stats();

    Ok(p_stats)
}

//move PStats to the a getter on the data instance
//allow Pstats to add

async fn retrieve_aggregate_crucible_stats(
    member_id: String,
    character_id: String,
    platform: Platform,
    mode: CrucibleMode,
    period: TimePeriod,
    verbose: bool,
) -> Result<CrucibleStats, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let start_date = period.get_date_time();

    let data: Vec<DailyPvPStatsValuesData> = client
        .retrieve_aggregate_crucible_stats(member_id, character_id, platform, mode, start_date)
        .await?;

    let mut p_stats: CrucibleStats = CrucibleStats::default();

    for d in data.iter() {
        let cs = d.values.get_crucible_stats();
        p_stats = cs + p_stats;
    }

    Ok(p_stats)
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let character_id: String = opt.character_id.unwrap_or("0".to_string());
    let mode: CrucibleMode = opt.mode.unwrap_or(CrucibleMode::AllPvP);
    let period: TimePeriod = opt.period.unwrap_or(TimePeriod::Alltime);

    let data = match period {
        TimePeriod::Alltime => {
            match retrieve_all_time_stats(
                opt.member_id,
                character_id,
                opt.platform,
                mode,
                opt.verbose,
            )
            .await
            {
                Ok(e) => e,
                Err(e) => {
                    print_error(&format!("Error : {:#?}", e), true);
                    std::process::exit(EXIT_FAILURE);
                }
            }
        }
        _ => {
            match retrieve_aggregate_crucible_stats(
                opt.member_id,
                character_id,
                opt.platform,
                mode,
                TimePeriod::Reset,
                opt.verbose,
            )
            .await
            {
                Ok(e) => e,
                Err(e) => {
                    print_error(&format!("Error : {:#?}", e), true);
                    std::process::exit(EXIT_FAILURE);
                }
            }
        }
    };


    //clear the screen
    print!("{}[2J", 27 as char);

    let p = precision;
    let title: String = format!("Displaying stats for {:#} {:#}", mode, period);
    println!("{}", title);
    println!("{}", repeat_str("=", title.chars().count()));

    println!(
        "{wins} wins and {losses} losses for a {win_percentage}% win rate",
        wins = data.activities_won,
        losses = data.activities_lost,
        win_percentage = (data.activities_won / data.activities_entered) * 100.0,
    );

    println!("{}", "");

    let mut col_w = 10;
    println!(
        "{:<0col_w$}{:<0col_w$}{:<0col_w$}",
        "K/D",
        "KD/A",
        "Efficiency",
        col_w = col_w
    );
    println!("{}", repeat_str("-", col_w * 3));
    println!(
        "{kd:<0col_w$}{kda:<0col_w$}{e:<0col_w$}",
        kd = p(data.kills_deaths_ratio, 2),
        kda = p(data.kills_deaths_assists, 2),
        e = p(data.efficiency, 2),
        col_w = col_w
    );

    println!("{}", "");

    col_w = 15;
    println!(
        "{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}{:<0col_w$}",
        "",
        "KILLS",
        "DEFEATS",
        "ASSISTS",
        "DEATHS",
        "SUICIDES",
        col_w = col_w
    );
    println!("{}", repeat_str("-", col_w * 6));
    println!(
        "{t:>0col_w$}{k:<0col_w$}{o:<0col_w$}{a:<0col_w$}{d:<0col_w$}{s:<0col_w$}",
        t = "PER GAME  ",
        k = p(data.kills / data.activities_entered, 2),
        o = p(data.opponents_defeated / data.activities_entered, 2),
        a = p(data.assists / data.activities_entered, 2),
        d = p(data.deaths / data.activities_entered, 2),
        s = p(data.suicides / data.activities_entered, 2),
        col_w = col_w,
    );
    println!(
        "{t:>0col_w$}{k:<0col_w$}{o:<0col_w$}{a:<0col_w$}{d:<0col_w$}{s:<0col_w$}",
        t = "TOTAL  ",
        k = p(data.kills, 0),
        o = p(data.opponents_defeated, 0),
        a = p(data.assists, 0),
        d = p(data.deaths, 0),
        s = p(data.suicides, 0),
        col_w = col_w,
    );

    //TODO : add precision

    println!("{}", "");

    println!("{:<25} : {}", "TIME PLAYED", p(data.seconds_played, 0));
    println!("{:<25} : {}", "AVERAGE LIFE SPAN", p(data.average_lifespan, 2));
    println!("{:<25} : {}", "AVERAGE KILL DISTANCE", p(data.average_kill_distance, 2));

}

fn precision(val: f32, precision: usize) -> String {
    format!("{:.prec$}", val, prec = precision)
}

fn repeat_str(s: &str, count: usize) -> String {
    std::iter::repeat(s).take(count).collect::<String>()
}
