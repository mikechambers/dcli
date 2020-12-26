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

use std::ops;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::str_to_datetime;
use crate::response::utils::{property_to_option_float, property_to_value};
use crate::utils::{
    calculate_efficiency, calculate_kills_deaths_assists,
    calculate_kills_deaths_ratio,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTimePvPStatsResponse {
    #[serde(rename = "Response")]
    pub response: Option<AllPvPStatsData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for AllTimePvPStatsResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllPvPStatsData {
    #[serde(
        alias = "allPvP",
        alias = "ironBanner",
        alias = "control",
        alias = "clash",
        alias = "allMayhem",
        alias = "privateMatches",
        alias = "trialsofthenine",
        alias = "rumble",
        alias = "pvpCompetitive",
        alias = "pvpQuickplay",
        alias = "trials_of_osiris"
    )]
    pub data: Option<AllTimePvPStatsData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTimePvPStatsData {
    #[serde(rename = "allTime")]
    pub all_time: Option<PvpStatsData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct PvpStatsData {
    #[serde(
        rename = "activitiesEntered",
        deserialize_with = "property_to_value"
    )]
    pub activities_entered: f32,

    #[serde(rename = "activitiesWon", deserialize_with = "property_to_value")]
    pub activities_won: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub assists: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub kills: f32,

    #[serde(
        rename = "averageKillDistance",
        deserialize_with = "property_to_value"
    )]
    pub average_kill_distance: f32,

    #[serde(
        rename = "totalKillDistance",
        deserialize_with = "property_to_value"
    )]
    pub total_kill_distance: f32,

    #[serde(rename = "secondsPlayed", deserialize_with = "property_to_value")]
    pub seconds_played: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub deaths: f32,

    #[serde(
        rename = "averageLifespan",
        deserialize_with = "property_to_value"
    )]
    pub average_lifespan: f32,

    //TODO: this doesnt get called if the property is not include in the JSON
    //Have set defalt so we dont get a parse error at run time, but that means
    //right now the value will never be None, but will be -1 if the property doesnt
    //exists. will update once i get more info on the issue
    //BUG: https://github.com/serde-rs/json/issues/734
    #[serde(
        rename = "bestSingleGameKills",
        deserialize_with = "property_to_option_float"
    )]
    #[serde(default)]
    pub best_single_game_kills: Option<f32>,

    #[serde(
        rename = "opponentsDefeated",
        deserialize_with = "property_to_value"
    )]
    pub opponents_defeated: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub efficiency: f32,

    #[serde(
        rename = "killsDeathsRatio",
        deserialize_with = "property_to_value"
    )]
    pub kills_deaths_ratio: f32,

    #[serde(
        rename = "killsDeathsAssists",
        deserialize_with = "property_to_value"
    )]
    pub kills_deaths_assists: f32,

    #[serde(rename = "precisionKills", deserialize_with = "property_to_value")]
    pub precision_kills: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub suicides: f32,
}

impl PvpStatsData {
    pub fn get_activities_lost(&self) -> f32 {
        self.activities_entered - self.activities_won
    }

    pub fn get_total_lifespan(&self) -> f32 {
        self.average_lifespan * self.deaths //estimate
    }
}

impl ops::Add<PvpStatsData> for PvpStatsData {
    type Output = PvpStatsData;

    fn add(self, _cs: PvpStatsData) -> PvpStatsData {
        //note, all of this stuff for single game kills is actually not necessary
        //since right now, its only returned when we get all time stats, not daily
        //so we dont really every need to aggregate stats.
        //but we will keep it here for completeness sake and in case the API is
        //ever updated
        let best_single_game_kills: Option<f32>;
        if _cs.best_single_game_kills.is_none()
            || self.best_single_game_kills.is_none()
        {
            if _cs.best_single_game_kills.is_none() {
                best_single_game_kills = self.best_single_game_kills;
            } else {
                best_single_game_kills = _cs.best_single_game_kills;
            }
        } else {
            let a = _cs.best_single_game_kills.unwrap();
            let b = self.best_single_game_kills.unwrap();
            let c = if a > b { a } else { b };
            best_single_game_kills = Some(c);
        }

        let kills = self.kills + _cs.kills;
        let total_kill_distance =
            self.total_kill_distance + _cs.total_kill_distance;
        let assists = self.assists + _cs.assists;
        let deaths = self.deaths + _cs.deaths;

        let total_lifespan =
            self.get_total_lifespan() + _cs.get_total_lifespan();

        //this doesnt completely work, since there are times where a lifespan
        //does not end in death (i.e. end of game)
        //so when aggregating values, this is an estimate
        let average_lifespan = total_lifespan / deaths;

        //todo : add activities_lost
        PvpStatsData {
            activities_entered: self.activities_entered
                + _cs.activities_entered,
            activities_won: self.activities_won + _cs.activities_won,
            //activities_lost: self.activities_lost + _cs.activities_lost,
            assists,
            kills,
            average_kill_distance: total_kill_distance / kills,
            total_kill_distance,
            seconds_played: self.seconds_played + _cs.seconds_played,
            deaths,
            average_lifespan,
            //total_lifespan,
            opponents_defeated: self.opponents_defeated
                + _cs.opponents_defeated,
            efficiency: calculate_efficiency(
                kills as u32,
                deaths as u32,
                assists as u32,
            ),
            kills_deaths_ratio: calculate_kills_deaths_ratio(
                kills as u32,
                deaths as u32,
            ),
            kills_deaths_assists: calculate_kills_deaths_assists(
                kills as u32,
                deaths as u32,
                assists as u32,
            ),
            suicides: self.suicides + _cs.suicides,
            best_single_game_kills,
            precision_kills: self.precision_kills + _cs.precision_kills,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsResponse {
    #[serde(rename = "Response")]
    pub response: Option<DailyPvPStatsData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for DailyPvPStatsResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsData {
    #[serde(
        alias = "allPvP",
        alias = "ironBanner",
        alias = "control",
        alias = "clash",
        alias = "allMayhem",
        alias = "privateMatches",
        alias = "trialsofthenine",
        alias = "rumble",
        alias = "pvpCompetitive",
        alias = "pvpQuickplay",
        alias = "trials_of_osiris"
    )]
    pub data: Option<DailyPvPStatsDailyData>,
    //todo: this might not need to be an option
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsDailyData {
    pub daily: Option<Vec<DailyPvPStatsValuesData>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsValuesData {
    #[serde(skip_serializing, deserialize_with = "str_to_datetime")]
    pub period: DateTime<Utc>,

    pub values: PvpStatsData,
}
