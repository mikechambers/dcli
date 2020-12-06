use crate::response::utils::str_to_datetime;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::property_to_value;
use crate::mode::Mode;
use crate::platform::Platform;

pub const MAX_ACTIVITIES_REQUEST_COUNT: i32 = 250;

//https://bungie-net.github.io/multi/operation_get_Destiny2-GetActivityHistory.html#operation_get_Destiny2-GetActivityHistory
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitiesResponse {
    #[serde(rename = "Response")]
    pub response: Option<ActivitiesResponseData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for ActivitiesResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyActivityHistoryResults.html#schema_Destiny-HistoricalStats-DestinyActivityHistoryResults
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitiesResponseData {
    #[serde(rename = "activities")]
    pub activities: Option<Vec<Activity>>,

}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsPeriodGroup.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsPeriodGroup
#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    
    #[serde(
        skip_serializing,
        deserialize_with = "str_to_datetime"
    )]
    pub period: DateTime<Utc>,

    #[serde(rename = "activityDetails")]
    pub details:ActivityDetails,

    //todo: can we collapse these down?
    pub values:ActivityValues,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityValues {
    #[serde(deserialize_with="property_to_value")]
    assists:f32,
}

/*
    Parses this structure
    "assists": {
        "statId": "assists",
        "basic": {
            "value": 9.0,
*/


//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyHistoricalStatsActivity.html#schema_Destiny-HistoricalStats-DestinyHistoricalStatsActivity#[derive(Serialize, Deserialize, Debug)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityDetails {

    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    /// (Seems to be the same as director_activity_hash)
    #[serde(rename = "referenceId")]
    pub reference_id:u32,

    /// The unique hash identifier of the DestinyActivityDefinition (Manifest) that was played
    #[serde(rename = "directorActivityHash")]
    pub director_activity_hash:u32,

    /// The unique identifier for this *specific* match that was played.
    /// 
    /// This value can be used to get additional data about this activity such 
    /// as who else was playing via the GetPostGameCarnageReport endpoint. 
    #[serde(rename = "instanceId")]
    pub instance_id:String,

    pub mode:Mode,

    pub modes : Vec<Mode>, //may need to make Option?

    /// Whether or not the match was a private match
    #[serde(rename = "isPrivate")]
    pub is_private:bool,

    /// The platform the activitity was played on
    #[serde(rename = "membershipType")]
    pub membership_type: Platform,
}
