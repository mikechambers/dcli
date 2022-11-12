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
use std::cmp::Ordering;
use std::path::PathBuf;
use tell::{Tell, TellLevel};

use dcli::crucible::{Member, PlayerName};
use dcli::enums::standing::Standing;
use dcli::enums::{
    completionreason::CompletionReason,
    moment::{DateTimePeriod, Moment},
};
use dcli::manifestinterface::ManifestInterface;
use dcli::utils::{
    calculate_average, calculate_percent, format_error,
    parse_and_validate_crucible_mode, parse_rfc3339, truncate_ascii_string,
};
use dcli::{
    crucible::{
        AggregateCruciblePerformances, CruciblePlayerActivityPerformance,
        CruciblePlayerPerformance,
    },
    enums::mode::Mode,
    utils::{calculate_ratio, human_duration},
};

use dcli::enums::character::CharacterClassSelection;
use dcli::enums::weaponsort::WeaponSort;

use dcli::activitystoreinterface::ActivityStoreInterface;

use dcli::utils::{
    determine_data_dir, format_f32, human_date_format, repeat_str,
    uppercase_first_char,
};

use dcli::utils::EXIT_FAILURE;
use num_format::{Locale, ToFormattedString};
use structopt::StructOpt;

#[allow(clippy::too_many_arguments)]
fn print_default(
    member: &Member,
    data: &[CruciblePlayerActivityPerformance],
    activity_limit: &u32,
    mode: &Mode,
    time_period: &DateTimePeriod,
    moment: &Moment,
    end_moment: &Moment,
    weapon_count: &u32,
    weapon_sort: &WeaponSort,
    medal_count: &u32,
    character_class_selection: &CharacterClassSelection,
) {
    let player_name = member.name.get_bungie_name();

    let start_time = time_period.get_start();
    let end_time = time_period.get_end();

    let performances = data;

    let cpp: Vec<&CruciblePlayerPerformance> =
        performances.iter().map(|x| &x.performance).collect();
    let aggregate = AggregateCruciblePerformances::with_performances(&cpp);

    let activity_count = performances.len();

    let display_count = std::cmp::min(activity_count, *activity_limit as usize);
    let is_limited = activity_count != display_count;

    let start_time_label = human_date_format(&start_time);
    let end_time_label = human_date_format(&end_time);

    tell::update!();
    tell::update!();

    let char_class = match character_class_selection {
        CharacterClassSelection::Hunter => "Hunter",
        CharacterClassSelection::Titan => "Titan",
        CharacterClassSelection::Warlock => "Warlock",
        CharacterClassSelection::All => "all characters",
        CharacterClassSelection::LastActive => "last active character",
    };

    //todo: if player name is empty, then the sentence below will be a little weird
    let title = if end_moment == &Moment::Now {
        format!(
            "{mode} activities for {player_name} on {char_class} since {start_time} ({moment})",
            mode = uppercase_first_char(&format!("{}", mode)),
            start_time = start_time_label,
            moment = moment,
            char_class = char_class,
            player_name = player_name,
        )
    } else {
        format!(
            "{mode} activities for {player_name} on {char_class} from {start_time} ({moment}) to {end_time} ({end_moment})",
            mode = uppercase_first_char(&format!("{}", mode)),
            start_time = start_time_label,
            moment = moment,
            end_time = end_time_label,
            end_moment = end_moment,
            char_class = char_class,
            player_name = player_name,
        )
    };

    tell::update!();
    tell::update!("ACTIVITIES");
    tell::update!("==================");
    tell::update!("{}", title);
    tell::update!(
        "Total time played is {}",
        human_duration(aggregate.time_played_seconds)
    );
    tell::update!();

    if is_limited {
        tell::update!(
            "Displaying details for the last {display_count} of {activity_count} activities",
            display_count = display_count,
            activity_count = activity_count,
        );
    } else {
        tell::update!(
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
    tell::update!();

    let col_w = 8;
    let wl_col_w = 14;
    let map_col_w = 18;
    let str_col_w = 7;
    let id_col_w = 8;

    //TODO: maybe format this to yellow background
    let header = format!(
        "{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
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
        "SUP",
        "GREN",
        "MEL",
        "MERCY",
        "INDEX",
        col_w = col_w,
        map_col_w = map_col_w,
        str_col_w=str_col_w,
        wl_col_w=wl_col_w,
        id_col_w=id_col_w,
    );
    tell::update!("{}", header);
    let header_divider = repeat_str(&"=", header.chars().count());
    tell::update!("{}", header_divider);

    let slice: &[CruciblePlayerActivityPerformance] = if is_limited {
        tell::update!(
            "{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
            "...", "...", "...", "...", "...", "...", "...","...","...","...","...","...", "...", "...","...",
            col_w = col_w,
            map_col_w = map_col_w,
            str_col_w=str_col_w,
            wl_col_w=wl_col_w,
            id_col_w = id_col_w,
        );

        &performances[..*activity_limit as usize]
    } else {
        &performances
    };

    let mut last_mode = Mode::None;
    let mut streak: i32 = 0;
    let mut last_standing: Standing = Standing::Unknown;

    for activity in slice.iter().rev() {
        if activity.activity_detail.mode != last_mode {
            tell::update!();
            tell::update!("{}", activity.activity_detail.mode);
            tell::update!("{}", repeat_str(&"-", col_w + map_col_w));
            last_mode = activity.activity_detail.mode;
        }

        let standing = activity.performance.stats.standing;
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
        /*
        if map_name.chars().count() > map_col_w - 1 {
            map_name = map_name[..(col_w - 3)].to_string();
            map_name.push_str("..")
        }
        */
        map_name = truncate_ascii_string(&map_name, map_col_w);

        let extended = activity.performance.stats.extended.as_ref().unwrap();
        let supers = extended.weapon_kills_super;
        let grenades = extended.weapon_kills_grenade;
        let melees = extended.weapon_kills_melee;

        let mercy_str = if activity.performance.stats.completion_reason
            == CompletionReason::Mercy
        {
            "X"
        } else {
            ""
        };

        tell::update!(
            "{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
            map_name,
            activity.performance.stats.standing.to_string(),
            streak.to_string(),
            activity.performance.stats.kills.to_string(),
            activity.performance.stats.assists.to_string(),
            activity.performance.stats.opponents_defeated.to_string(),
            activity.performance.stats.deaths.to_string(),
            format_f32(activity.performance.stats.kills_deaths_ratio, 2),
            format_f32(activity.performance.stats.kills_deaths_assists, 2),
            format_f32(activity.performance.stats.efficiency, 2),
            supers.to_string(),
            grenades.to_string(),
            melees.to_string(),
            mercy_str,
            activity.activity_detail.index_id.to_string(),
            col_w = col_w,
            map_col_w=map_col_w,
            str_col_w=str_col_w,
            wl_col_w=wl_col_w,
            id_col_w=id_col_w,
        );
    }

    let extended = aggregate.extended.as_ref().unwrap();
    tell::update!("{}", repeat_str(&"-", header.chars().count()));

    tell::update!("{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
    "TOTAL",
    aggregate.total_activities.to_formatted_string(&Locale::en),
    "",
    aggregate.kills.to_formatted_string(&Locale::en),
    aggregate.assists.to_formatted_string(&Locale::en),
    aggregate.opponents_defeated.to_formatted_string(&Locale::en),
    aggregate.deaths.to_formatted_string(&Locale::en),
    "".to_string(),
    "".to_string(),
    "".to_string(),
    extended.weapon_kills_super.to_formatted_string(&Locale::en),
    extended.weapon_kills_grenade.to_formatted_string(&Locale::en),
    extended.weapon_kills_melee.to_formatted_string(&Locale::en),
    aggregate.total_mercy.to_string(),
    "",
    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    wl_col_w=wl_col_w,
    id_col_w=id_col_w,
    );

    tell::update!("{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
    "HIGH",
    format!("{}-{}", aggregate.wins.to_formatted_string(&Locale::en), aggregate.losses.to_formatted_string(&Locale::en)),
    format!("{}W {}L", aggregate.longest_win_streak, aggregate.longest_loss_streak),
    format!("{}", aggregate.highest_kills),
    format!("{}", aggregate.highest_assists),
    format!("{}", aggregate.highest_opponents_defeated),
    format!("{}", aggregate.highest_deaths),

    format_f32(aggregate.highest_kills_deaths_ratio, 2),
    format_f32(aggregate.highest_kills_deaths_assists, 2),
    format_f32(aggregate.highest_efficiency, 2),
    format!("{}", extended.highest_weapon_kills_super),
    format!("{}", extended.highest_weapon_kills_grenade),
    format!("{}", extended.highest_weapon_kills_melee),
    "",
    "",

    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    wl_col_w=wl_col_w,
    id_col_w=id_col_w,
    );

    tell::update!("{:<0map_col_w$}{:<0wl_col_w$}{:>0str_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0id_col_w$}",
    "PER GAME",
    format!("{}%", format_f32(aggregate.win_rate, 2)),
    "",
    format_f32(calculate_average(aggregate.kills, aggregate.total_activities), 2),
    format_f32(calculate_average(aggregate.assists, aggregate.total_activities), 2),
    format_f32(calculate_average(aggregate.opponents_defeated, aggregate.total_activities), 2),
    format_f32(calculate_average(aggregate.deaths, aggregate.total_activities), 2),
    format_f32(aggregate.kills_deaths_ratio, 2),
    format_f32(aggregate.kills_deaths_assists, 2),
    format_f32(aggregate.efficiency, 2),
    format_f32(calculate_average(extended.weapon_kills_super, aggregate.total_activities), 2),
    format_f32(calculate_average(extended.weapon_kills_grenade, aggregate.total_activities), 2),
    format_f32(calculate_average(extended.weapon_kills_melee, aggregate.total_activities), 2),
    format!("{}%",format_f32(calculate_percent(aggregate.total_mercy, aggregate.total_activities), 2)),
    "",
    col_w = col_w,
    map_col_w=map_col_w,
    str_col_w=str_col_w,
    wl_col_w=wl_col_w,
    id_col_w=id_col_w,
    );

    tell::update!("{}", header_divider);
    tell::update!("{}", header);

    tell::update!();
    tell::update!();

    //we have to calculate this even though we might not display weapons as the medal output needs this data
    let wep_col = map_col_w + col_w;
    let col_w_w = col_w + 2;
    let wep_header_str = format!(
        "{:<0map_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0map_col_w$}",
        "WEAPON",
        "GAMES",
        "KILLS",
        "K/Gk",
        "% TOTAL",
        "WIN %",
        "PREC",
        "% PREC",
        "TYPE",
        col_w = col_w_w,
        map_col_w = wep_col,
    );

    let wep_divider_len = wep_header_str.chars().count();
    let wep_divider = repeat_str(&"=", wep_divider_len);

    if weapon_count > &0 {
        tell::update!("{}", wep_header_str);
        tell::update!("{}", wep_divider);

        let mut weapons = extended.weapons.clone();
        match weapon_sort {
            WeaponSort::Name => {
                weapons.sort_by(|a, b| {
                    a.weapon
                        .name
                        .to_lowercase()
                        .cmp(&b.weapon.name.to_lowercase())
                });
            }
            WeaponSort::Kills => {
                //sorted by kills by default so we dont need to sort again
                //weapons.sort_by(|a, b| b.kills.cmp(&a.kills));
            }
            WeaponSort::Games => {
                weapons.sort_by(|a, b| b.activity_count.cmp(&a.activity_count));
            }
            WeaponSort::KillsPerGameKills => {
                weapons.sort_by(|a, b| {
                    let a_kpk = calculate_ratio(a.kills, a.activity_count);
                    let b_kpk = calculate_ratio(b.kills, b.activity_count);
                    b_kpk.partial_cmp(&a_kpk).unwrap()
                });
            }
            WeaponSort::PrecisionTotal => {
                weapons.sort_by(|a, b| {
                    b.precision_kills.partial_cmp(&a.precision_kills).unwrap()
                });
            }
            WeaponSort::PrecisionPercent => {
                weapons.sort_by(|a, b| {
                    b.precision_kills_percent
                        .partial_cmp(&a.precision_kills_percent)
                        .unwrap()
                });
            }
            WeaponSort::WinPercent => {
                weapons.sort_by(|a, b| {
                    let a_wp = calculate_percent(a.wins, a.activity_count);
                    let b_wp = calculate_percent(b.wins, b.activity_count);
                    b_wp.partial_cmp(&a_wp).unwrap()
                });
            }
            WeaponSort::Type => {
                weapons.sort_by(|a, b| {
                    let a_type =
                        format!("{}", a.weapon.item_sub_type).to_lowercase();
                    let b_type =
                        format!("{}", b.weapon.item_sub_type).to_lowercase();

                    a_type.cmp(&b_type)
                });
            }
        }

        let max_weps = std::cmp::min(*weapon_count as usize, weapons.len());

        for w in &weapons[..max_weps] {
            tell::update!(
            "{:<0map_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0map_col_w$}",
            w.weapon.name,
            w.activity_count.to_formatted_string(&Locale::en),
            w.kills.to_formatted_string(&Locale::en),
            format_f32(calculate_ratio(w.kills, w.activity_count), 2),
            format!("{}%", format_f32((w.kills as f32 / aggregate.kills as f32) * 100.0, 2)),
            format!("{}%", format_f32(calculate_percent(w.wins, w.activity_count), 2)),
            w.precision_kills.to_formatted_string(&Locale::en),
            format!("{}%", format_f32(w.precision_kills_percent, 2)),
            format!("{}", w.weapon.item_sub_type),
            col_w = col_w_w,
            map_col_w = wep_col,
        );
        }
        tell::update!();
        tell::update!("% TOTAL - Percentage of all kills");
        tell::update!(
            "K/Gk - Kills per game in games with a kill with the weapon"
        );
        tell::update!(
            "WIN % - Win percentage in games with a kill with the weapon"
        );

        tell::update!();
        tell::update!();
    }

    if medal_count > &0 {
        let med_col = map_col_w + col_w;
        let col_w_w = col_w + 2;
        let col_w_h = col_w / 2;
        let med_header_str = format!(
        "{:<0map_col_w$}{:<col_w_h$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>col_w_h$}{:<0col_w$}",
        "MEDAL",
        "",
        "COUNT",
        "M/G",
        "G/M",
        "",
        "DESCRIPTION",
        col_w = col_w_w,
        map_col_w = med_col,
        col_w_h = col_w_h);

        let med_divider_len = wep_divider_len;
        let med_divider = repeat_str(&"=", med_divider_len);

        let mut medals = extended.medals.clone();
        medals.retain(|m| {
            m.medal.tier != dcli::enums::medaltier::MedalTier::Unknown
        });

        tell::update!("{}", med_header_str);
        tell::update!("{}", med_divider);

        if !medals.is_empty() {
            medals.sort_by(|a, b| {
                let a_gold = a.medal.is_gold();
                let b_gold = b.medal.is_gold();

                if b_gold == a_gold {
                    return b.count.cmp(&a.count);
                }

                if b_gold && !a_gold {
                    return Ordering::Greater;
                }

                Ordering::Less
            });

            //TODO: can make this an option
            let max_medals = std::cmp::min(*medal_count as usize, medals.len());

            for m in &medals[..max_medals] {
                let gold = if m.medal.is_gold() { "Gold" } else { "" };

                let gm = (activity_count as f32 / m.count as f32).ceil();

                tell::update!(
                    "{:<0map_col_w$}{:<col_w_h$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>col_w_h$}{:<0col_w$}",
                    m.medal.name,
                    gold,
                    m.count.to_string(),
                    format_f32(m.count as f32 / activity_count as f32, 2),
                    gm.to_string(),
                    "",
                    truncate_ascii_string(&m.medal.description, med_divider_len - (wep_col + (col_w_w * 3) + col_w_h * 2)),
                    col_w = col_w_w,
                    map_col_w = wep_col,
                    col_w_h = col_w_h
                );
            }
        } else {
            tell::update!("No medals");
        }

        tell::update!();
        tell::update!("M/G - Medals per game");
        tell::update!("G/M - Number of games to get one medal");

        tell::update!();
        tell::update!();
    }
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

    /// Limit the number of activity details that will be displayed
    ///
    /// Summary information will be generated based on all activities.
    #[structopt(long = "activity-limit", short = "L", default_value = "10")]
    activity_limit: u32,

    /// The number of weapons to display details for
    #[structopt(long = "weapon-count", short = "w", default_value = "5")]
    weapon_count: u32,

    /// The number of medals to display details for. Gold medals will be listed first.
    #[structopt(long = "medal-count", short = "m", default_value = "5")]
    medal_count: u32,

    /// Character to retrieve data for
    ///
    /// Valid values include hunter, titan, warlock, last_active and all.
    #[structopt(short = "C", long = "class", default_value = "all")]
    character_class_selection: CharacterClassSelection,

    /// Specify weapon stats sort order
    ///
    /// Valid values include name, kills (default), games, kills_per_game_kills,
    /// precision_total, precision_percent, type, wins_percent
    #[structopt(short = "W", long = "weapon-sort", default_value = "kills")]
    weapon_sort: WeaponSort,

    ///Print out additional information

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

    let level = if opt.verbose {
        TellLevel::Verbose
    } else {
        TellLevel::Progress
    };
    Tell::init(level);

    tell::verbose!("{:#?}", opt);
    log::info!("{:#?}", opt);

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
                tell::error!("--end-moment must be greater than --moment");
                std::process::exit(EXIT_FAILURE);
            }
        };

    let mut store =
        match ActivityStoreInterface::init_with_path(&data_dir, opt.api_key)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                tell::error!("{}", format_error(
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

    let member: Member = match store.find_member(&opt.name, true).await {
        Ok(e) => e,
        Err(e) => {
            tell::error!(
                "Could not find Bungie ID. Please check name and try again. {}",
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
            tell::error!(
                "{}",
                format_error("Could not retrieve data from activity store.", e)
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    if data.is_none() {
        tell::update!("No activities found");
        return;
    }

    let data: Vec<CruciblePlayerActivityPerformance> = data.unwrap();

    if data.is_empty() {
        tell::update!("No activities found");
        return;
    }

    print_default(
        &member,
        &data,
        &opt.activity_limit,
        &opt.mode,
        &time_period,
        &opt.moment,
        &opt.end_moment,
        &opt.weapon_count,
        &opt.weapon_sort,
        &opt.medal_count,
        &opt.character_class_selection,
    );
}
