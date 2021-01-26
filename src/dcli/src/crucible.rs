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

use crate::enums::mode::Mode;
use crate::enums::platform::Platform;
use crate::enums::standing::Standing;
use crate::enums::{
    character::CharacterClass,
    itemtype::{ItemSubType, ItemType},
};
use crate::enums::{completionreason::CompletionReason, medaltier::MedalTier};
use chrono::{DateTime, Utc};

use std::{cmp::max, collections::hash_map::DefaultHasher, hash::Hasher};
use std::{collections::HashMap, hash::Hash};

use crate::utils::{
    calculate_efficiency, calculate_kills_deaths_assists, calculate_kills_deaths_ratio,
};

const PLAYER_START_BUFFER: u32 = 30;

#[derive(Debug, Clone)]
pub struct Team {
    pub id: i32,
    pub standing: Standing,
    pub score: u32,
    pub player_performances: Vec<CruciblePlayerPerformance>,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct CrucibleActivity {
    pub details: ActivityDetail,
    pub teams: HashMap<i32, Team>,
}

impl CrucibleActivity {
    pub fn get_member_performance(&self, member_id: &str) -> Option<&CruciblePlayerPerformance> {
        for t in self.teams.values() {
            for p in &t.player_performances {
                if p.player.member_id == member_id {
                    return Some(p);
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct CruciblePlayerPerformance {
    pub player: Player,
    pub stats: CrucibleStats,
}

#[derive(Debug, Clone)]
pub struct CruciblePlayerActivityPerformance {
    pub performance: CruciblePlayerPerformance,
    pub activity_detail: ActivityDetail,
}

#[derive(Debug, Clone)]
pub struct CrucibleStats {
    pub assists: u32,
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
    pub average_score_per_kill: f32,
    pub average_score_per_life: f32,
    pub completed: bool,
    pub opponents_defeated: u32,
    pub efficiency: f32,
    pub kills_deaths_ratio: f32,
    pub kills_deaths_assists: f32,
    pub activity_duration_seconds: u32,
    pub standing: Standing,
    pub team: i32,
    pub completion_reason: CompletionReason,
    pub start_seconds: u32,
    pub time_played_seconds: u32,
    pub player_count: u32,
    pub team_score: u32,

    pub extended: Option<ExtendedCrucibleStats>,
}

impl CrucibleStats {
    pub fn generate_status(&self) -> String {
        let mut out: Vec<String> = Vec::new();

        if self.start_seconds > PLAYER_START_BUFFER {
            out.push("L".to_string());
        }

        if !self.completed {
            out.push("E".to_string());
        }

        out.join("")
    }
}

#[derive(Debug, Clone)]
pub struct ExtendedCrucibleStats {
    pub precision_kills: u32,
    pub weapon_kills_ability: u32,
    pub weapon_kills_grenade: u32,
    pub weapon_kills_melee: u32,
    pub weapon_kills_super: u32,
    pub all_medals_earned: u32,

    pub weapons: Vec<WeaponStat>,
    pub medals: Vec<MedalStat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub member_id: String,
    pub character_id: String,
    pub platform: Platform,
    pub display_name: String,
    pub light_level: i32,
    pub class_type: CharacterClass,
}

impl Player {
    pub fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

#[derive(Debug, Clone)]
pub struct WeaponStat {
    pub weapon: Item,
    pub kills: u32,
    pub precision_kills: u32,
    pub precision_kills_percent: f32,
    pub activity_count: u32,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub item_sub_type: ItemSubType,
}

#[derive(Debug, Clone)]
pub struct MedalStat {
    pub medal: Medal,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct Medal {
    pub id: String,
    pub icon_image_path: Option<String>,
    pub tier: MedalTier,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct AggregateCruciblePerformances {
    pub total_activities: u32,
    pub wins: u32,
    pub losses: u32,
    pub win_rate: f32,

    pub assists: u32,
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
    pub opponents_defeated: u32,
    pub efficiency: f32,
    pub kills_deaths_ratio: f32,
    pub kills_deaths_assists: f32,
    pub activity_duration_seconds: u32,
    pub time_played_seconds: u32,

    pub highest_assists: u32,
    pub highest_score: u32,
    pub highest_kills: u32,
    pub highest_deaths: u32,
    pub highest_opponents_defeated: u32,
    pub highest_efficiency: f32,
    pub highest_kills_deaths_ratio: f32,
    pub highest_kills_deaths_assists: f32,

    pub longest_win_streak: u32,
    pub longest_loss_streak: u32,

    pub total_mercy: u32,

    pub extended: Option<ExtendedCruciblePlayerActivityPerformances>,
}

impl AggregateCruciblePerformances {
    pub fn with_performances(
        performances: &[&CruciblePlayerPerformance],
    ) -> AggregateCruciblePerformances {
        let mut out = AggregateCruciblePerformances::default();
        let mut extended = ExtendedCruciblePlayerActivityPerformances::default();

        out.total_activities = performances.len() as u32;

        let mut medal_hash: HashMap<String, MedalStat> = HashMap::new();
        let mut weapon_hash: HashMap<u32, WeaponStat> = HashMap::new();

        let mut streak: i32 = 0;
        let mut longest_win_streak: u32 = 0;
        let mut longest_loss_streak: u32 = 0;
        let mut last_standing = Standing::Unknown;

        let mut has_extended = false;
        for p in performances {
            if p.stats.completion_reason == CompletionReason::Mercy {
                out.total_mercy += 1;
            };

            out.assists += p.stats.assists;
            out.score += p.stats.score;
            out.kills += p.stats.kills;
            out.deaths += p.stats.deaths;
            out.opponents_defeated += p.stats.opponents_defeated;

            out.activity_duration_seconds += p.stats.activity_duration_seconds;
            out.time_played_seconds += p.stats.time_played_seconds;

            out.highest_assists = max(p.stats.assists, out.highest_assists);
            out.highest_score = max(p.stats.score, out.highest_score);
            out.highest_kills = max(p.stats.kills, out.highest_kills);
            out.highest_deaths = max(p.stats.deaths, out.highest_deaths);
            out.highest_opponents_defeated =
                max(p.stats.opponents_defeated, out.highest_opponents_defeated);
            out.highest_efficiency = out.highest_efficiency.max(p.stats.efficiency);
            out.highest_kills_deaths_ratio = out
                .highest_kills_deaths_ratio
                .max(p.stats.kills_deaths_ratio);
            out.highest_kills_deaths_assists = out
                .highest_kills_deaths_assists
                .max(p.stats.kills_deaths_assists);

            match p.stats.standing {
                Standing::Victory => {
                    out.wins += 1;

                    if last_standing == Standing::Victory {
                        streak += 1;
                    } else {
                        streak = 1;
                    }
                }
                Standing::Defeat => {
                    out.losses += 1;
                    if last_standing == Standing::Defeat {
                        streak -= 1;
                    } else {
                        streak = -1;
                    }
                }
                Standing::Unknown => (),
            };

            #[allow(clippy::comparison_chain)]
            if streak > 0 {
                longest_win_streak = std::cmp::max(longest_win_streak, streak as u32);
            } else if streak < 0 {
                longest_loss_streak = std::cmp::max(longest_loss_streak, streak.abs() as u32);
            }

            last_standing = p.stats.standing;

            if p.stats.extended.is_some() {
                has_extended = true;
                let e = p.stats.extended.as_ref().unwrap();
                extended.weapon_kills_ability += e.weapon_kills_ability;
                extended.weapon_kills_grenade += e.weapon_kills_grenade;
                extended.weapon_kills_melee += e.weapon_kills_melee;
                extended.weapon_kills_super += e.weapon_kills_super;
                extended.all_medals_earned += e.all_medals_earned;
                extended.precision_kills += e.precision_kills;

                extended.highest_precision_kills =
                    max(extended.highest_precision_kills, e.precision_kills);
                extended.highest_weapon_kills_ability = max(
                    extended.highest_weapon_kills_ability,
                    e.weapon_kills_ability,
                );
                extended.highest_weapon_kills_grenade = max(
                    extended.highest_weapon_kills_grenade,
                    e.weapon_kills_grenade,
                );

                extended.highest_weapon_kills_melee =
                    max(extended.highest_weapon_kills_melee, e.weapon_kills_melee);
                extended.highest_weapon_kills_super =
                    max(extended.highest_weapon_kills_super, e.weapon_kills_super);
                extended.highest_all_medals_earned =
                    max(extended.highest_all_medals_earned, e.all_medals_earned);

                for m in &e.medals {
                    let key = &m.medal.id;

                    match medal_hash.get_mut(key) {
                        Some(e) => {
                            e.count += m.count;
                        }
                        None => {
                            let mut c = m.clone();
                            if c.count == 0 {
                                c.count = 1;
                            }

                            medal_hash.insert(key.clone(), c);
                        }
                    }
                }

                for w in &e.weapons {
                    let key = &w.weapon.id;

                    let mut ws = match weapon_hash.get_mut(key) {
                        Some(e) => {
                            e.activity_count += 1;
                            e.kills += w.kills;
                            e.precision_kills += w.precision_kills;
                            e
                        }
                        None => {
                            weapon_hash.insert(*key, w.clone());
                            weapon_hash.get_mut(key).unwrap()
                        }
                    };

                    ws.precision_kills_percent = {
                        if ws.kills == 0 {
                            0.0
                        } else {
                            (ws.precision_kills as f32 / ws.kills as f32) * 100.0
                        }
                    };
                }
            }
        }

        out.longest_win_streak = longest_win_streak;
        out.longest_loss_streak = longest_loss_streak;

        if has_extended {
            let mut medals: Vec<MedalStat> = medal_hash.into_iter().map(|(_id, m)| m).collect();

            medals.sort_by(|a, b| b.count.cmp(&a.count));

            let mut weapons: Vec<WeaponStat> = weapon_hash.into_iter().map(|(_id, w)| w).collect();
            weapons.sort_by(|a, b| b.kills.cmp(&a.kills));

            extended.medals = medals;
            extended.weapons = weapons;

            out.extended = Some(extended);
        } else {
            out.extended = None;
        }

        if out.total_activities > 0 {
            out.win_rate = (out.wins as f32 / out.total_activities as f32) * 100.0;
        }

        out.efficiency = calculate_efficiency(out.kills, out.deaths, out.assists);
        out.kills_deaths_ratio = calculate_kills_deaths_ratio(out.kills, out.deaths);
        out.kills_deaths_assists =
            calculate_kills_deaths_assists(out.kills, out.deaths, out.assists);

        out
    }

    pub fn stat_per_game(&self, value: u32) -> f32 {
        if self.total_activities == 0 {
            return 0.0;
        }

        value as f32 / self.total_activities as f32
    }
}

#[derive(Debug, Default)]
pub struct ExtendedCruciblePlayerActivityPerformances {
    pub precision_kills: u32,
    pub weapon_kills_ability: u32,
    pub weapon_kills_grenade: u32,
    pub weapon_kills_melee: u32,
    pub weapon_kills_super: u32,
    pub all_medals_earned: u32,

    pub highest_precision_kills: u32,
    pub highest_weapon_kills_ability: u32,
    pub highest_weapon_kills_grenade: u32,
    pub highest_weapon_kills_melee: u32,
    pub highest_weapon_kills_super: u32,
    pub highest_all_medals_earned: u32,

    pub weapons: Vec<WeaponStat>,
    pub medals: Vec<MedalStat>,
}

#[derive(Debug, Clone)]
pub struct ActivityDetail {
    pub index_id: u32,
    pub id: i64,
    pub period: DateTime<Utc>,
    pub map_name: String,
    pub mode: Mode,
    pub platform: Platform,
    pub director_activity_hash: u32,
    pub reference_id: u32,
}
