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

use crate::response::drs::{DestinyResponseStatus, HasDestinyResponseStatus};
use serde_derive::{Deserialize, Serialize};
use crate::apiutils::str_to_datetime;
use crate::cruciblestats::CrucibleStats;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTimePvPStatsResponse {

    #[serde(rename = "Response")]
    pub response: Option<AllPvPStatsData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for AllTimePvPStatsResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllPvPStatsData {
    #[serde(alias = "allPvP", 
    alias = "ironBanner", alias = "control", 
    alias="clash", alias="allMayhem",
    alias="privateMatches", alias="trialsofthenine",
    alias="rumble", alias="pvpCompetitive",
    alias="pvpQuickplay", alias="trials_of_osiris"
)]
    pub data:Option<AllTimePvPStatsData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTimePvPStatsData {
    #[serde(rename = "allTime")]
    pub all_time:Option<PvpStatsData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PvpStatsData {
    #[serde(rename = "activitiesEntered")]
    pub activities_entered:PvpAllTimeStatItemData,

    #[serde(rename = "activitiesWon")]
    pub activities_won:PvpAllTimeStatItemData,

    pub assists:PvpAllTimeStatItemData,

    pub kills:PvpAllTimeStatItemData,

    #[serde(rename = "averageKillDistance")]
    pub average_kill_distance:PvpAllTimeStatItemData,

    #[serde(rename = "totalKillDistance")]
    pub total_kill_distance:PvpAllTimeStatItemData,

    #[serde(rename = "secondsPlayed")]
    pub seconds_played:PvpAllTimeStatItemData,

    pub deaths:PvpAllTimeStatItemData,

    #[serde(rename = "averageLifespan")]
    pub average_lifespan:PvpAllTimeStatItemData,

    #[serde(rename = "bestSingleGameKills")]
    pub best_single_game_kills:Option<PvpAllTimeStatItemData>,

    #[serde(rename = "opponentsDefeated")]
    pub opponents_defeated:PvpAllTimeStatItemData,

    pub efficiency:PvpAllTimeStatItemData,

    #[serde(rename = "killsDeathsRatio")]
    pub kills_deaths_ratio:PvpAllTimeStatItemData,

    #[serde(rename = "killsDeathsAssists")]
    pub kills_deaths_assists:PvpAllTimeStatItemData,

    #[serde(rename = "precisionKills")]
    pub precision_kills:PvpAllTimeStatItemData,

    pub suicides:PvpAllTimeStatItemData,
}

impl PvpStatsData {
    pub fn get_crucible_stats(&self) -> CrucibleStats {

        let best_single_game_kills:Option<f32> = match self.best_single_game_kills.as_ref() {
            Some(ref e) => Some(e.basic.value),
            None => None,
        };

        CrucibleStats {
            activities_entered : self.activities_entered.basic.value,
            activities_won : self.activities_won.basic.value,
            activities_lost : self.activities_entered.basic.value - self.activities_won.basic.value,
            assists : self.assists.basic.value,
            kills : self.kills.basic.value,
            average_kill_distance : self.average_kill_distance.basic.value,
            total_kill_distance : self.total_kill_distance.basic.value,
            seconds_played : self.seconds_played.basic.value,
            deaths : self.deaths.basic.value,
            average_lifespan : self.average_lifespan.basic.value,
            total_lifespan: self.average_lifespan.basic.value * self.deaths.basic.value, //estimate
            opponents_defeated : self.opponents_defeated.basic.value,
            efficiency : self.efficiency.basic.value,
            kills_deaths_ratio : self.kills_deaths_ratio.basic.value,
            kills_deaths_assists : self.kills_deaths_assists.basic.value,
            suicides : self.suicides.basic.value,
            precision_kills: self.precision_kills.basic.value,
            best_single_game_kills : best_single_game_kills,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PvpAllTimeStatItemData {
    #[serde(rename = "statId")]
    pub stat_id:String,
    pub basic:BasicFloatData,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicFloatData {
    pub value:f32,

    #[serde(rename = "displayValue")]
    pub display_value:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsResponse {

    #[serde(rename = "Response")]
    pub response: Option<DailyPvPStatsData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for DailyPvPStatsResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsData {
        #[serde(alias = "allPvP", 
            alias = "ironBanner", alias = "control", 
            alias="clash", alias="allMayhem",
            alias="privateMatches", alias="trialsofthenine",
            alias="rumble", alias="pvpCompetitive",
            alias="pvpQuickplay", alias="trials_of_osiris"

        )]
    pub data:Option<DailyPvPStatsDailyData>,
    //todo: this might not need to be an option
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsDailyData {
    pub daily:Option<Vec<DailyPvPStatsValuesData>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsValuesData {

    #[serde(
        skip_serializing,
        deserialize_with = "str_to_datetime"
    )]
    pub period:DateTime<Utc>,

    pub values:PvpStatsData,
}
