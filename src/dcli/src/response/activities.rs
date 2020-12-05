use crate::response::utils::str_to_datetime;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use crate::response::utils::property_to_float;

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitiesResponse {
    #[serde(rename = "Response")]
    pub response: Option<Vec<Activity>>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    
    #[serde(
        skip_serializing,
        deserialize_with = "str_to_datetime"
    )]
    pub period: DateTime<Utc>,

    #[serde(rename = "activityDetails")]
    activity_details:ActivityDetails,

    values:ActivityValues,

}

pub struct ActivityValues {
    #[serde(deserialize_with="property_to_float")]
    assists:f32,
}

/*
    Parses this structure
    "assists": {
        "statId": "assists",
        "basic": {
            "value": 9.0,
*/


#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityDetails {
    referenceId:u32,
    directorActivityHash:u32,
    instanceId:u32,
    mode:Mode,
    isPrivate:bool,
    //membershipType: //is this platform?

}
