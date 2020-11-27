use crate::response::drs::{DestinyResponseStatus, HasDestinyResponseStatus};
use serde_derive::{Deserialize, Serialize};


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
    pub best_single_game_kills:PvpAllTimeStatItemData,

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
    pub displayValue:String,
}
