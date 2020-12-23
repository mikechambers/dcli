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

use crate::mode::Mode;
use crate::platform::Platform;
use chrono::{DateTime, Utc};

pub struct CruciblePlayerPerformance {
    player: Player,
    activity_detail: ActivityDetail,

    stats: CrucibleStats,
}

pub struct CrucibleStats {
    pub assists: f32,
    pub score: u32,
    pub kills: u32,
    pub deaths: u32,
    pub average_score_per_kill: f32,
    pub average_score_per_life: f32,
    pub completed: u32,
    pub opponents_defeated: u32,
    pub efficiency: f32,
    pub kills_deaths_ratio: f32,
    pub kills_deaths_assists: f32,
    pub activity_duration_seconds: u32,
    pub standing: i32,
    pub team: u32,
    pub completion_reason: u32,
    pub start_seconds: u32,
    pub time_played_seconds: u32,
    pub player_count: u32,
    pub team_score: u32,

    extended: Option<ExtendedCrucibleStats>,
}

pub struct ExtendedCrucibleStats {
    pub precision_kills: u32,
    pub weapon_kills_ability: u32,
    pub weapon_kills_grenade: u32,
    pub weapon_kills_melee: u32,
    pub weapon_kills_super: u32,
    pub all_medals_earned: u32,

    weapons: Vec<WeaponStat>,
    medals: Vec<MedalStat>,
}

pub struct Player {
    member_id: String,
    character_id: String,
    platform: String,
}

pub struct WeaponStat {
    id: String,
    name: String,
    description: String,
    pub kills: u32,
    pub precision_kills: u32,
    pub precision_kills_percent: f32,
}

pub struct MedalStat {
    id: u32,
    name: String,
    count: u32,
}

pub struct PlayerCruciblePerformances {
    pub performances: Vec<CruciblePlayerPerformance>,
    pub aggregate: CrucibleStats,
}

#[derive(Debug)]
pub struct ActivityDetail {
    pub id: i64,
    pub period: DateTime<Utc>,
    pub map_name: String,
    pub mode: Mode,
    pub platform: Platform,
    pub director_activity_hash: u32,
    pub reference_id: u32,
}
