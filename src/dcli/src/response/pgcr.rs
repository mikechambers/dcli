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
use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::str_to_datetime;
use crate::response::utils::{property_to_standing, property_to_value};
use crate::standing::Standing;
use crate::response::activities::{ActivityDetails, ActivityValues};

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

pub const MAX_ACTIVITIES_REQUEST_COUNT: i32 = 250;

//https://bungie-net.github.io/multi/operation_get_Destiny2-GetActivityHistory.html#operation_get_Destiny2-GetActivityHistory
#[derive(Serialize, Deserialize, Debug)]
pub struct PGCRResponse {
    #[serde(rename = "Response")]
    pub response: Option<PGCRResponseData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for PGCRResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PGCRResponseData {
    #[serde(rename = "activityDetails")]
    activity_details:ActivityDetails,

    entries:Vec<EntryData>,

    #[serde(skip_serializing, deserialize_with = "str_to_datetime")]
    pub period: DateTime<Utc>,

    //teams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryData {
    #[serde(rename = "characterId")]
    character_id:String,


    //extended,
    //player,
    //score,
    //standing,
    values:ActivityValues,
}