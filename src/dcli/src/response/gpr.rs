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

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::mode::Mode;
use crate::response::character::CharacterData;
use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::str_to_datetime;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfileResponse {
    #[serde(rename = "Response")]
    pub response: Option<ProfileResponse>, //should this be an option?

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for GetProfileResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileResponse {
    pub characters: Option<CharacterDataFieldData>,

    #[serde(rename = "characterActivities")]
    pub character_activities: Option<CharacterActivitiesFieldData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterActivitiesDataField {
    pub data: HashMap<String, CharacterActivitiesData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterDataFieldData {
    pub data: HashMap<String, CharacterData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterActivitiesFieldData {
    pub data: HashMap<String, CharacterActivitiesData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterActivitiesData {
    #[serde(
        rename = "dateActivityStarted",
        skip_serializing,
        deserialize_with = "str_to_datetime"
    )]
    pub date_activity_started: DateTime<Utc>,

    #[serde(rename = "currentActivityHash")]
    pub current_activity_hash: u32,

    #[serde(rename = "currentActivityModeHash")]
    pub current_activity_mode_hash: u32, //these both point to the same data (0 if not active)

    #[serde(rename = "currentActivityModeType")]
    //todo: could default this to none / 0
    pub current_activity_mode_type: Option<Mode>, // (0 if not active)

    #[serde(rename = "currentPlaylistActivityHash")]
    pub current_playlist_activity_hash: Option<u32>, //how is this different than currentActivityHash?
}

/*
                    "currentActivityHash": 1813752023, //destination (will be 0 if not active)
                    "currentActivityModeHash": 3497767639, //activity (will be 0 if not active)
                    "currentActivityModeType": 6, //activity (patrol)
                    "currentActivityModeHashes": [
                        3497767639, //activty
                        1164760493 //pve
                    ],
                    "currentActivityModeTypes": [
                        6, //patrol
                        7 //AllPVE
                    ],
                    "currentPlaylistActivityHash": 1813752023, //destination
                    "lastCompletedStoryHash": 0
*/
