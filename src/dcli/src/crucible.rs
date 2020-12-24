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

use crate::enums::itemtype::{ItemSubType, ItemType};
use crate::enums::medaltier::MedalTier;
use crate::enums::mode::Mode;
use crate::enums::platform::Platform;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CruciblePlayerPerformance {
    pub player: Player,
    pub activity_detail: ActivityDetail,

    pub stats: CrucibleStats,
}

#[derive(Debug)]
pub struct CrucibleStats {
    pub assists: u32,
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
    pub standing: u32,
    pub team: u32,
    pub completion_reason: u32,
    pub start_seconds: u32,
    pub time_played_seconds: u32,
    pub player_count: u32,
    pub team_score: u32,

    pub extended: Option<ExtendedCrucibleStats>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Player {
    pub member_id: String,
    pub character_id: String,
    pub platform: Platform,
}

#[derive(Debug)]
pub struct WeaponStat {
    pub weapon: Item,
    pub kills: u32,
    pub precision_kills: u32,
    pub precision_kills_percent: f32,
}

#[derive(Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub item_sub_type: ItemSubType,
}

#[derive(Debug)]
pub struct MedalStat {
    pub medal: Medal,
    pub count: u32,
}

#[derive(Debug)]
pub struct Medal {
    pub id: String,
    pub icon_image_path: Option<String>,
    pub tier: MedalTier,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
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
