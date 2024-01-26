/*
* Copyright 2023 Mike Chambers
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

use tell::{Tell, TellLevel};

use std::str::FromStr;
use std::{collections::HashMap, path::PathBuf};

use dcli::crucible::{Member, PlayerName};
use dcli::utils::{format_error, truncate_ascii_string};
use dcli::{
    apiinterface::ApiInterface,
    crucible::{
        AggregateCruciblePerformances, CrucibleActivity,
        CruciblePlayerPerformance, Player,
    },
    enums::completionreason::CompletionReason,
    utils::{calculate_avg, f32_are_equal},
};

use dcli::enums::mode::Mode;
use dcli::manifestinterface::ManifestInterface;

use dcli::enums::character::CharacterClassSelection;
use dcli::error::Error;

use dcli::activitystoreinterface::ActivityStoreInterface;

use dcli::utils::{
    determine_data_dir, format_f32, human_date_format, human_duration,
    repeat_str,
};

use dcli::utils::EXIT_FAILURE;
use structopt::StructOpt;

const ELO_SCALE: f32 = 10.0;

fn parse_and_validate_mode(src: &str) -> Result<Mode, String> {
    let mode = Mode::from_str(src)?;

    if !mode.is_crucible() {
        return Err(format!("Unsupported mode specified : {}", src));
    }

    Ok(mode)
}

fn generate_score(data: &CrucibleActivity) -> String {
    let mut tokens: Vec<String> = Vec::new();

    for t in data.teams.values() {
        tokens.push(t.score.to_string());
        tokens.push("-".to_string());
    }

    tokens.pop();

    tokens.join("")
}

async fn get_combat_ratings(data: &CrucibleActivity) -> HashMap<u64, f32> {
    let mut players: Vec<&Player> = Vec::new();

    for t in data.teams.values() {
        for p in &t.player_performances {
            players.push(&p.player);
        }
    }

    let elo_hash: HashMap<u64, f32> = match ApiInterface::new() {
        Ok(e) => {
            let mut player_refs: Vec<&Player> = Vec::new();
            for t in data.teams.values() {
                for p in &t.player_performances {
                    player_refs.push(&p.player);
                }
            }

            match e
                .retrieve_combat_ratings(&player_refs, &data.details.mode)
                .await
            {
                Ok(e) => e,
                Err(_e) => HashMap::new(),
            }
        }
        Err(_e) => HashMap::new(),
    };
    elo_hash
}

fn print_default(
    data: &CrucibleActivity,
    elo_hash: &HashMap<u64, f32>,
    member: &Member,
    details: bool,
    weapon_count: u32,
) {
    let member_id = &member.id;

    let col_w = 8;
    let name_col_w = 24;

    let mut activity_duration = "".to_string();
    let mut completion_reason = "".to_string();
    let mut standing_str = "".to_string();

    if let Some(e) = data.get_member_performance(member_id) {
        completion_reason =
            if e.stats.completion_reason == CompletionReason::Unknown {
                "".to_string()
            } else {
                format!("({})", e.stats.completion_reason)
            };

        activity_duration =
            format!("({})", human_duration(e.stats.activity_duration_seconds));
        standing_str = format!("{}!", e.stats.standing);
    };

    let team_title_border = repeat_str("-", name_col_w + col_w);
    let activity_title_border = repeat_str("=", name_col_w + col_w + col_w);

    tell::update!("\nACTIVITY");
    tell::update!("{}", activity_title_border);

    tell::update!(
        "{} on {} :: {} {}",
        data.details.mode,
        data.details.map_name,
        human_date_format(&data.details.period),
        activity_duration
    );

    tell::verbose!("Activity ID : {}", data.details.id);

    tell::update!("{}", standing_str);
    tell::update!("{} {}\n", generate_score(data), completion_reason);

    let header = format!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "PLAYER",
    "KILLS",
    "ASTS",
    "K+A",
    "DEATHS",
    "K/D",
    "KD/A",
    "EFF",
    "SUP",
    "GREN",
    "MEL",
    "MED",
    "RATING",
    "STATUS",
    col_w=col_w,
    name_col_w = name_col_w,
    );

    let table_width = header.chars().count();
    let header_border = repeat_str("=", table_width);
    let entry_border = repeat_str(".", table_width);
    let footer_border = repeat_str("-", table_width);

    let mut all_performances: Vec<&CruciblePlayerPerformance> = Vec::new();
    let mut elo_total_count = 0;
    let mut elo_total_total = 0.0;
    for v in data.teams.values() {
        let mut elo_team_count = 0;
        let mut elo_team_total = 0.0;

        tell::update!("[{}] {} Team {}!", v.score, v.display_name, v.standing);
        tell::update!("{}", team_title_border);
        tell::update!("{}", header);
        tell::update!("{}", header_border);

        let mut first_performance = true;

        let mut player_performances = v.player_performances.clone();
        player_performances.sort_by(|a, b| {
            b.stats.opponents_defeated.cmp(&a.stats.opponents_defeated)
        });

        for p in &player_performances {
            let elo = *elo_hash.get(&p.player.calculate_hash()).unwrap_or(&0.0)
                * ELO_SCALE;

            let mut elo_str = "".to_string();
            if !f32_are_equal(elo, 0.0) {
                elo_team_count += 1;
                elo_team_total += elo;

                elo_total_count += 1;
                elo_total_total += elo;

                elo_str = format_f32(elo, 0);
            }

            let extended = p.stats.extended.as_ref().unwrap();
            tell::update!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
                truncate_ascii_string(&p.player.name.get_bungie_name(), name_col_w),
                p.stats.kills.to_string(),
                p.stats.assists.to_string(),
                p.stats.opponents_defeated.to_string(),
                p.stats.deaths.to_string(),
                format_f32(p.stats.kills_deaths_ratio, 2),
                format_f32(p.stats.kills_deaths_assists, 2),
                format_f32(p.stats.efficiency, 2),
                extended.weapon_kills_super.to_string(),
                extended.weapon_kills_grenade.to_string(),
                extended.weapon_kills_melee.to_string(),
                extended.all_medals_earned.to_string(),
                elo_str,
                p.stats.generate_status(),
                col_w=col_w,
                name_col_w = name_col_w,
            );

            //todo: what if they dont have weapon kills (test)
            if details && !extended.weapons.is_empty() {
                tell::update!("{}", entry_border);

                let mut weapons = extended.weapons.clone();
                weapons.sort_by(|a, b| b.kills.cmp(&a.kills));

                let mut min_index = 2;
                if first_performance {
                    tell::update!(
                        //"{:>0w_name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w2$}",
                        "{:<0col_w$}{:>0w_name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w2$}",
                        format!("{}", p.player.class_type),
                        "NAME",
                        "KILLS",
                        "PREC",
                        "%",
                        "TYPE",
                        w_name_col_w = col_w + col_w + name_col_w,
                        col_w = col_w,
                        col_w2 = col_w * 3,
                    );
                    first_performance = false;
                    min_index = 1;
                }

                for i in 0..std::cmp::max(min_index, weapons.len()) {
                    let modifier = 2 - min_index;
                    let meta = match i + modifier {
                        0 => format!("{}", p.player.class_type),
                        1 => p.player.light_level.to_string(),
                        _ => "".to_string(),
                    };

                    let mut weapon_name = "".to_string();
                    let mut weapon_kills = "".to_string();
                    let mut precision_kills = "".to_string();
                    let mut precision_kills_percent = "".to_string();
                    let mut weapon_type = "".to_string();

                    if i < weapons.len() {
                        let w = &weapons[i];
                        weapon_name = w.weapon.name.to_string();
                        weapon_kills = w.kills.to_string();
                        precision_kills = w.precision_kills.to_string();
                        precision_kills_percent =
                            format_f32(w.precision_kills_percent * 100.0, 0)
                                .to_string();
                        weapon_type = format!("{}", w.weapon.item_sub_type);
                    }

                    tell::update!(
                        "{:<0col_w$}{:>0w_name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w2$}",
                        meta,
                        weapon_name,
                        weapon_kills,
                        precision_kills,
                        precision_kills_percent,
                        weapon_type,
                        w_name_col_w = col_w + col_w + name_col_w,
                        col_w = col_w,
                        col_w2 = col_w * 3,
                    );
                }
                tell::update!();
            }
        }
        tell::update!("{}", footer_border);

        let mut cpp: Vec<&CruciblePlayerPerformance> = Vec::new();

        for p in &v.player_performances {
            cpp.push(p);
            all_performances.push(p);
        }

        let aggregate = AggregateCruciblePerformances::with_performances(&cpp);

        let agg_extended = aggregate.extended.as_ref().unwrap();
        let agg_supers = agg_extended.weapon_kills_super;
        let agg_grenades = agg_extended.weapon_kills_grenade;
        let agg_melees = agg_extended.weapon_kills_melee;

        let team_elo = calculate_avg(elo_team_total, elo_team_count);
        let team_elo_str = if f32_are_equal(team_elo, 0.0) {
            "".to_string()
        } else {
            format_f32(team_elo, 0)
        };

        tell::update!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
            "TOTAL",
            aggregate.kills.to_string(),
            aggregate.assists.to_string(),
            aggregate.opponents_defeated.to_string(),
            aggregate.deaths.to_string(),
            format_f32(aggregate.kills_deaths_ratio, 2),
            format_f32(aggregate.kills_deaths_assists, 2),
            format_f32(aggregate.efficiency, 2),
            agg_supers.to_string(),
            agg_grenades.to_string(),
            agg_melees.to_string(),
            aggregate.extended.as_ref().unwrap().all_medals_earned.to_string(),
            "",
            "",
            col_w=col_w,
            name_col_w = name_col_w,
        );

        tell::update!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}\n",
            "AVG",
            format_f32(aggregate.kills as f32 / player_performances.len() as f32, 2),
            format_f32(aggregate.assists as f32 / player_performances.len() as f32,2),
            format_f32(aggregate.opponents_defeated as f32 / player_performances.len() as f32,2),
            format_f32(aggregate.deaths as f32 / player_performances.len() as f32,2),
            "",
            "",
            "",
            format_f32(agg_supers as f32 / player_performances.len() as f32,2),
            format_f32(agg_grenades as f32 / player_performances.len() as f32,2),
            format_f32(agg_melees as f32 / player_performances.len() as f32,2),
            format_f32(aggregate.extended.as_ref().unwrap().all_medals_earned as f32 / player_performances.len() as f32,2),
            team_elo_str,
            "", //MAKE THIS REASON FOR COMPLETEION
            col_w=col_w,
            name_col_w = name_col_w,
        );

        //tell::update!("{}", header_border);
        //tell::update!("{}", header);
    }

    tell::update!("Combined");
    tell::update!("{}", team_title_border);

    let aggregate =
        AggregateCruciblePerformances::with_performances(&all_performances);

    let agg_extended = aggregate.extended.as_ref().unwrap();
    let agg_supers = agg_extended.weapon_kills_super;
    let agg_grenades = agg_extended.weapon_kills_grenade;
    let agg_melees = agg_extended.weapon_kills_melee;

    tell::update!("{}", header);
    tell::update!("{}", header_border);
    tell::update!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
        "TOTAL",
        aggregate.kills.to_string(),
        aggregate.assists.to_string(),
        aggregate.opponents_defeated.to_string(),
        aggregate.deaths.to_string(),
        format_f32(aggregate.kills_deaths_ratio, 2),
        format_f32(aggregate.kills_deaths_assists, 2),
        format_f32(aggregate.efficiency, 2),
        agg_supers.to_string(),
        agg_grenades.to_string(),
        agg_melees.to_string(),
        aggregate.extended.as_ref().unwrap().all_medals_earned.to_string(),
        "",
        "", //MAKE THIS REASON FOR COMPLETEION
        col_w=col_w,
        name_col_w = name_col_w,
    );

    let total_elo = calculate_avg(elo_total_total, elo_total_count);
    let total_elo_str = if f32_are_equal(total_elo, 0.0) {
        "".to_string()
    } else {
        format_f32(total_elo, 0)
    };

    tell::update!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "AVG",
    format_f32(aggregate.kills as f32 / all_performances.len() as f32, 2),
    format_f32(aggregate.assists as f32 / all_performances.len() as f32,2),
    format_f32(aggregate.opponents_defeated as f32 / all_performances.len() as f32,2),
    format_f32(aggregate.deaths as f32 / all_performances.len() as f32,2),
    "",
    "",
    "",
    format_f32(agg_supers as f32 / all_performances.len() as f32,2),
    format_f32(agg_grenades as f32 / all_performances.len() as f32,2),
    format_f32(agg_melees as f32 / all_performances.len() as f32,2),
    format_f32(aggregate.extended.as_ref().unwrap().all_medals_earned as f32 / all_performances.len() as f32,2),
    total_elo_str,
    "", //MAKE THIS REASON FOR COMPLETEION
    col_w=col_w,
    name_col_w = name_col_w,
);
    tell::update!();

    let wep_col = name_col_w + col_w;
    let wep_header_str = format!(
        "{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0name_col_w$}",
        "WEAPON",
        "KILLS",
        "% TOTAL",
        "PREC",
        "% PREC",
        "TYPE",
        col_w = col_w,
        name_col_w = wep_col,
    );

    let wep_divider = repeat_str("=", wep_header_str.chars().count());
    tell::update!("{}", wep_header_str);
    tell::update!("{}", wep_divider);

    let weapons = &aggregate.extended.as_ref().unwrap().weapons;
    let max_weps = std::cmp::min(weapon_count as usize, weapons.len());

    let wep_col = name_col_w + col_w;
    for w in &weapons[..max_weps] {
        tell::update!(
            "{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0name_col_w$}",
            w.weapon.name,
            w.kills.to_string(),
            format!(
                "{}%",
                format_f32((w.kills as f32 / aggregate.kills as f32) * 100.0, 2)
            ),
            w.precision_kills.to_string(),
            format!("{}%", format_f32(w.precision_kills_percent, 2)),
            format!("{}", w.weapon.item_sub_type),
            col_w = col_w,
            name_col_w = wep_col,
        );
    }

    tell::update!();
    tell::update!("STATUS : L - Joined late, E - Left early");
    tell::update!();
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving and viewing Destiny 2 Crucible activity details.
///
/// By default the details on the last activity will be displayed, and you can
/// specify the specific activity via the --activity-index argument. The index
/// can be retrieved from dcliah, as well as directly from the sqlite datastore
/// (activity.id)
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

    /// Activity mode from which to return last activity
    ///
    /// Supported values are all_pvp (default), control, clash, elimination,
    /// mayhem, iron_banner, all_private, rumble, pvp_competitive,
    /// quickplay and trials_of_osiris.
    ///
    /// Addition values available are crimsom_doubles, supremacy, survival,
    /// countdown, all_doubles, doubles, private_clash, private_control,
    /// private_survival, private_rumble, showdown_competitive, survival_competitive,
    /// rift_competitive, showdown, lockdown, iron_banner_rift,
    /// zone_control, iron_banner_zone_control, rift,
    /// scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine, relic, countdown_competitive, checkmate_all, checkmate_control, checkmate_rumble, checkmate_survival, checkmate_rumble, checkmate_clash, checkmate_countdown, iron_banner_tribute, iron_banner_fortress
    #[structopt(long = "mode", short = "M", 
        parse(try_from_str=parse_and_validate_mode), default_value = "all_pvp")]
    mode: Mode,

    /// Character class to retrieve data for
    ///
    /// Valid values include hunter, titan, warlock, last_active and all.
    #[structopt(short = "C", long = "class", default_value = "all")]
    character_class_selection: CharacterClassSelection,

    ///Print out additional information

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Sync player activities
    #[structopt(long = "sync", short = "s")]
    sync: bool,

    /// Display extended activity details
    ///
    /// If flag is set, additional information will be displayed, including per
    /// user weapon stats.
    #[structopt(short = "d", long = "details")]
    details: bool,

    /// The number of weapons to display details for
    #[structopt(long = "weapon-count", short = "w", default_value = "5")]
    weapon_count: u32,

    /// The activity id of the activity to display data about
    ///
    /// By default, the last activity will be displayed. The index can be retrieved
    /// from other dcli apps, such as dcliah, or directly from the sqlite datastore.
    #[structopt(long = "activity-id", short = "a")]
    activity_id: Option<i64>,

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

    let level = if opt.verbose {
        TellLevel::Verbose
    } else {
        TellLevel::Progress
    };
    Tell::init(level);

    tell::verbose!("{:#?}", opt);

    let data_dir = match determine_data_dir(opt.data_dir) {
        Ok(e) => e,
        Err(e) => {
            tell::error!(
                "{}",
                format_error("Error initializing data directory.", e)
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut store =
        match ActivityStoreInterface::init_with_path(&data_dir, opt.api_key)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                tell::error!("{}",format_error(
                "Could not initialize activity store. Have you run dclisync?",
                e,
            ));
                std::process::exit(EXIT_FAILURE);
            }
        };

    let mut manifest = match ManifestInterface::new(&data_dir, false).await {
        Ok(e) => e,
        Err(e) => {
            tell::error!(
                "{}",
                format_error(
                    "Could not initialize manifest. Have you run dclim?",
                    e,
                )
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let member = match store.find_member(&opt.name, true).await {
        Ok(e) => e,
        Err(e) => {
            tell::error!(
                "Could not find Bungie id. Please check name and try again. {}",
                e
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    if opt.sync {
        match store.sync_member(&member).await {
            Ok(_e) => (),
            Err(e) => {
                tell::error!("Could not sync activity store {}", e);
                tell::update!("Using existing data");
            }
        };
    }

    let data_result = match opt.activity_id {
        Some(e) => store.retrieve_activity(e, &mut manifest).await,
        None => {
            store
                .retrieve_last_activity(
                    &member,
                    &opt.character_class_selection,
                    &opt.mode,
                    &mut manifest,
                )
                .await
        }
    };

    let data = match data_result {
        Ok(e) => e,
        Err(e) => {
            if e == Error::ActivityNotFound {
                tell::update!("No activities found");
                return;
            }

            tell::error!(
                "{}",
                format_error("Could not retrieve data from activity store.", e)
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let elo_hash = get_combat_ratings(&data).await;

    print_default(&data, &elo_hash, &member, opt.details, opt.weapon_count);
}
