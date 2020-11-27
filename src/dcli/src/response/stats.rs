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
    #[serde(alias = "allPvP", alias = "ironBanner")]
    pub data:Option<AllTimePvPStatsData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTimePvPStatsData {
    #[serde(rename = "allTime")]
    pub all_time:PvpStatsData
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

    pub suicides:PvpAllTimeStatItemData,
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
    #[serde(alias = "allPvP", alias = "ironBanner")]
    pub data:Option<DailyPvPStatsDailyData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPvPStatsDailyData {
    pub daily:Vec<DailyPvPStatsValuesData>
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


/*
    "Response": {
        "allPvP": {
            "daily": [
                {
                    "period": "2020-11-24T00:00:00Z",
                    "values": {
                        */