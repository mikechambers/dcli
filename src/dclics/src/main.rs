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
use dcli::timeperiod::TimePeriod;
use dcli::platform::Platform;
use dcli::response::stats::{PvpStatsData, DailyPvPStatsValuesData};

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
    terse: bool,

    ///Print out additional information
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

#[derive(Default, Debug)]
struct PStats {
    pub activities_entered:f32,
    pub activities_won:f32,
    pub assists:f32,
    pub kills:f32,
    pub average_kill_distance:f32,
    pub seconds_played:f32,
    pub deaths:f32,
    pub average_lifespan:f32,
    pub best_single_game_kills:Option<f32>,
    pub opponents_defeated:f32,
    pub efficiency:f32,
    pub kills_deaths_ratio:f32,
    pub kills_deaths_assists:f32,
    pub suicides:f32,
}

async fn retrieve_all_time_stats (
    member_id: String,
    character_id: String,
    platform: Platform,
    mode: CrucibleMode,
    verbose: bool,
) -> Result<PStats, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let data: PvpStatsData = client
        .retrieve_alltime_crucible_stats(member_id, character_id, platform, mode)
        .await?;

        let best_single_game_kills:Option<f32> = match data.best_single_game_kills {
            Some(e) => Some(e.basic.value),
            None => None,
        };

        let p_stats:PStats = PStats {

            activities_entered : data.activities_entered.basic.value,
            activities_won : data.activities_won.basic.value,
            assists : data.assists.basic.value,
            kills : data.kills.basic.value,
            average_kill_distance : data.average_kill_distance.basic.value,
            seconds_played : data.seconds_played.basic.value,
            deaths : data.deaths.basic.value,
            average_lifespan : data.average_lifespan.basic.value,
            opponents_defeated : data.opponents_defeated.basic.value,
            efficiency : data.efficiency.basic.value,
            kills_deaths_ratio : data.kills_deaths_ratio.basic.value,
            kills_deaths_assists : data.kills_deaths_assists.basic.value,
            suicides : data.suicides.basic.value,
            best_single_game_kills : best_single_game_kills,
        };

    Ok(p_stats)
}

async fn retrieve_aggregate_crucible_stats (
    member_id: String,
    character_id: String,
    platform: Platform,
    mode: CrucibleMode,
    period:TimePeriod,
    verbose: bool,
) -> Result<PStats, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let start_date = period.get_date_time();

    let data: Vec<DailyPvPStatsValuesData> = client
        .retrieve_aggregate_crucible_stats(member_id, character_id, platform, mode, start_date)
        .await?;

    let mut p_stats:PStats = PStats::default();

    for d in data.iter() {
        p_stats.activities_entered += d.values.activities_entered.basic.value;
        p_stats.activities_won += d.values.activities_won.basic.value;
        p_stats.assists += d.values.assists.basic.value;
        p_stats.kills += d.values.kills.basic.value;
        p_stats.average_kill_distance += d.values.average_kill_distance.basic.value;
        p_stats.seconds_played += d.values.seconds_played.basic.value;
        p_stats.deaths += d.values.deaths.basic.value;
        p_stats.average_lifespan += d.values.average_lifespan.basic.value;
        p_stats.opponents_defeated += d.values.opponents_defeated.basic.value;
        p_stats.efficiency += d.values.efficiency.basic.value;
        p_stats.kills_deaths_ratio += d.values.kills_deaths_ratio.basic.value;
        p_stats.kills_deaths_assists += d.values.kills_deaths_assists.basic.value;
        p_stats.suicides += d.values.suicides.basic.value;
    }

        
    Ok(p_stats)
}




#[tokio::main]
async fn main() {

    let opt = Opt::from_args();

    let character_id:String = opt.character_id.unwrap_or("0".to_string());
    let mode:CrucibleMode = opt.mode.unwrap_or(CrucibleMode::AllPvP);
    let period:TimePeriod = opt.period.unwrap_or(TimePeriod::Alltime);

    let data = match period {
        TimePeriod::Alltime => {
            match retrieve_all_time_stats(
                opt.member_id,
                character_id,
                opt.platform,
                mode,
                opt.verbose
            ).await {
                Ok(e) => e,
                Err(e) => {
                    print_standard(&format!("Error : {:#?}", e), true);
                    std::process::exit(EXIT_FAILURE);
                },
            }
        },
        _ => {
            match retrieve_aggregate_crucible_stats(
                opt.member_id,
                character_id,
                opt.platform,
                mode,
                TimePeriod::Reset,
                opt.verbose
            ).await {
                Ok(e) => e,
                Err(e) => {
                    print_standard(&format!("Error : {:#?}", e), true);
                    std::process::exit(EXIT_FAILURE);
                },
            }
    
        },
    };
    println!("Displaying stats for {:#} {:#}.", mode, period);
    println!("{:#?}", data);

}
