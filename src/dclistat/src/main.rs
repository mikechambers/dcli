/*
* Copyright 2022 Mike Chambers
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

use chrono::{DateTime, Utc};
use dcli::crucible::{Member, PlayerName};
use dcli::enums::moment::{DateTimePeriod, Moment};
use dcli::enums::stat::Stat;
use dcli::manifestinterface::ManifestInterface;
use dcli::utils::parse_rfc3339;
use dcli::{
    crucible::{
        AggregateCruciblePerformances, CruciblePlayerActivityPerformance,
        CruciblePlayerPerformance,
    },
    enums::mode::Mode,
};
use std::path::PathBuf;

use dcli::enums::character::CharacterClassSelection;

use dcli::activitystoreinterface::ActivityStoreInterface;

use dcli::utils::{
    determine_data_dir, format_f32, parse_and_validate_crucible_mode,
};

use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};
use structopt::StructOpt;

#[allow(clippy::too_many_arguments)]
fn print_default(data: &[CruciblePlayerActivityPerformance], stats: &[Stat]) {
    let performances = data;
    let cpp: Vec<&CruciblePlayerPerformance> =
        performances.iter().map(|x| &x.performance).collect();
    let aggregate = AggregateCruciblePerformances::with_performances(&cpp);

    let activity_count = performances.len();

    let mut out = Vec::<String>::new();
    for m in stats.iter() {
        let o: String = match m {
            Stat::Assists => format!("{}", aggregate.assists as f32),
            Stat::AssistsAvg => {
                format_f32(aggregate.stat_per_game(aggregate.assists), 2)
            }
            Stat::AssistsMax => format!("{}", aggregate.highest_assists),
            Stat::Deaths => format!("{}", aggregate.deaths as f32),
            Stat::DeathsAvg => {
                format_f32(aggregate.stat_per_game(aggregate.deaths), 2)
            }
            Stat::DeathsMax => format!("{}", aggregate.highest_deaths),
            Stat::Kills => format!("{}", aggregate.kills),
            Stat::KillsAvg => {
                format_f32(aggregate.stat_per_game(aggregate.kills), 2)
            }
            Stat::KillsMax => format!("{}", aggregate.highest_kills),
            Stat::OpponentsDefeated => {
                format!("{}", aggregate.opponents_defeated as f32)
            }
            Stat::OpponentsDefeatedAvg => format_f32(
                aggregate.stat_per_game(aggregate.opponents_defeated),
                2,
            ),
            Stat::OpponentsDefeatedMax => {
                format!("{}", aggregate.highest_opponents_defeated)
            }
            Stat::Efficiency => format_f32(aggregate.efficiency, 2),
            Stat::EfficiencyMax => format_f32(aggregate.highest_efficiency, 2),
            Stat::KD => format_f32(aggregate.kills_deaths_ratio, 2),
            Stat::KDMax => format_f32(aggregate.highest_kills_deaths_ratio, 2),
            Stat::KDA => format_f32(aggregate.kills_deaths_assists, 2),
            Stat::KDAMax => {
                format_f32(aggregate.highest_kills_deaths_assists, 2)
            }
            Stat::Games => format!("{}", activity_count),
            Stat::Wins => format!("{}", aggregate.wins),
            Stat::Losses => format!("{}", aggregate.losses),
            Stat::Mercies => format!("{}", aggregate.total_mercy),
        };

        out.push(o);
    }

    println!("{}", out.join(","));
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving and viewing Destiny 2 Crucible activity history.
///
/// Enables control of which stats are displayed based on game mode, moment range
/// from which to retrieve them and character.
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
    /// Bungie name for player
    ///
    /// Name must be in the format of NAME#CODE. Example: foo#3280
    /// You can find your name in game, or on Bungie's site at:
    /// https://www.bungie.net/7/en/User/Account/IdentitySettings
    #[structopt(long = "name", short = "n", required = true)]
    name: PlayerName,

    /// Custom start time in RFC 3339 date / time format
    ///
    /// Must be a valid date in the past.
    ///
    /// Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00
    ///
    /// Required when --moment is set to custom, but otherwise not applicable.
    #[structopt(short = "t", long = "custom-time", parse(try_from_str = parse_rfc3339), required_if("moment", "custom"))]
    custom_time: Option<DateTime<Utc>>,

    /// Custom end time in RFC 3339 date / time format
    ///
    /// Must be a valid date in the past.
    ///
    /// Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00
    ///
    /// Required when --end-moment is set to custom, but otherwise not applicable.
    #[structopt(short = "e", long = "end-custom-time", parse(try_from_str = parse_rfc3339), required_if("end-moment", "custom"))]
    end_custom_time: Option<DateTime<Utc>>,

    /// Start moment from which to pull activities from
    ///
    /// Activities will be retrieved from moment to end-moment.
    ///
    /// For example, Specifying: --moment weekly
    /// will return all activities since the last weekly reset on Tuesday.
    ///
    /// Valid values include daily (last daily reset), weekend
    /// (last weekend reset on Friday), weekly (last weekly reset on Tuesday),
    /// day (last day), week (last week), month (last month), all_time and custom
    /// as well as the following season moments launch, curse_of_osiris, warmind,
    /// season_of_the_outlaw, season_of_the_forge, season_of_the_drifter,
    /// season_of_opulence, season_of_the_undying, season_of_dawn,
    /// season_of_the_worthy, season_of_arrivals, season_of_the_hunt,
    /// season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen,
    /// witch_queen, season_of_the_haunted, season_of_the_plunder.
    ///
    /// When custom is specified, the custom start date in RFC3339 format must
    /// be specified with the --custom-time argument.
    ///
    /// For example:
    /// --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00
    #[structopt(long = "moment", short = "T", default_value = "week")]
    moment: Moment,

    /// End moment from which to pull activities from
    ///
    /// Activities will be retrieved from moment to end-moment. End moment
    /// must be greater than moment
    ///
    /// For example, Specifying: --moment month --end-moment weekly
    /// will return all activities from a month ago up to the most recent weekly
    /// reset.
    ///
    /// Valid values include daily (last daily reset), weekend
    /// (last weekend reset on Friday), weekly (last weekly reset on Tuesday),
    /// day (last day), week (last week), month (last month), all_time and custom
    /// as well as the following season moments launch, curse_of_osiris, warmind,
    /// season_of_the_outlaw, season_of_the_forge, season_of_the_drifter,
    /// season_of_opulence, season_of_the_undying, season_of_dawn,
    /// season_of_the_worthy, season_of_arrivals, season_of_the_hunt,
    /// season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen,
    /// witch_queen, season_of_the_haunted, season_of_the_plunder.
    ///
    /// When custom is specified, the custom start date in RFC3339 format must
    /// be specified with the --end-custom-time argument.
    ///
    /// For example:
    /// --moment custom --end-custom-time 2020-12-08T17:00:00.774187+00:00
    #[structopt(long = "end-moment", short = "E", default_value = "now")]
    end_moment: Moment,

    /// Activity mode to return stats for
    ///
    /// Supported values are all_pvp (default), control, clash, elimination,
    /// mayhem, iron_banner, all_private, rumble, pvp_competitive,
    /// quickplay and trials_of_osiris.
    ///
    /// Addition values available are crimsom_doubles, supremacy, survival,
    /// countdown, all_doubles, doubles, private_clash, private_control,
    /// private_survival, private_rumble, showdown, lockdown, iron_banner_rift, rift,
    /// scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine
    #[structopt(long = "mode", short = "M", 
        parse(try_from_str=parse_and_validate_crucible_mode), default_value = "all_pvp")]
    mode: Mode,

    /// Character to retrieve data for
    ///
    /// Valid values include hunter, titan, warlock, last_active and all.
    #[structopt(short = "C", long = "class", default_value = "last_active")]
    character_class_selection: CharacterClassSelection,

    /// Stat to retrieve data for
    ///
    /// Valid values include kd, kda, efficiency, kills, opponents_defeated, deaths,
    /// assists, kills_avg, opponents_defeated_avg, deaths_avg, assists_avg,
    /// kd_max, kda_max, efficiency_max, kills_max, opponents_defeated_max,
    /// deaths_max, games, wins, losses, mercies.
    #[structopt(short = "x", long = "stat", required = true)]
    stat: Vec<Stat>,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Sync player activities
    #[structopt(long = "sync", short = "s")]
    sync: bool,

    /// Directory where Destiny 2 manifest and activity database files are stored. (optional)
    ///
    /// This will normally be downloaded using the dclim tool, and uses
    /// a system appropriate directory by default.
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,

    /// API key from Bungie required for some actions.
    ///
    /// If specified the key will be passed to all Destiny API calls.
    ///
    /// You can obtain a key from https://www.bungie.net/en/Application
    #[structopt(short = "k", long = "api-key", env = "DESTINY_API_KEY")]
    api_key: Option<String>,
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

    let end_time = match opt.end_moment {
        Moment::Custom => {
            opt.end_custom_time.unwrap() //note, this should be ok, because struct opt should ensure valid value
        }
        _ => opt.end_moment.get_date_time(),
    };

    let time_period =
        match DateTimePeriod::with_start_end_time(start_time, end_time) {
            Ok(e) => e,
            Err(_e) => {
                eprintln!("--end-moment must be greater than --moment");
                std::process::exit(EXIT_FAILURE);
            }
        };

    let mut store = match ActivityStoreInterface::init_with_path(
        &data_dir,
        opt.verbose,
        opt.api_key,
    )
    .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error(
                "Could not initialize activity store. Have you run dclisync?",
                e,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut manifest = match ManifestInterface::new(&data_dir, false).await {
        Ok(e) => e,
        Err(e) => {
            print_error(
                "Could not initialize manifest. Have you run dclim?",
                e,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let member: Member = match store.find_member(&opt.name, true).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "Could not find bungie name. Please check name and try again. {}",
                e
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    if opt.sync {
        match store.sync_member(&member).await {
            Ok(_e) => (),
            Err(e) => {
                eprintln!("Could not sync activity store {}", e);
                eprintln!("Using existing data");
            }
        };
    }

    let data = match store
        .retrieve_activities_since(
            &member,
            &opt.character_class_selection,
            &opt.mode,
            &time_period,
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
        println!("No activities found");
        return;
    }

    let data: Vec<CruciblePlayerActivityPerformance> = data.unwrap();

    if data.is_empty() {
        println!("No activities found");
        return;
    }

    print_default(&data, &opt.stat);
}