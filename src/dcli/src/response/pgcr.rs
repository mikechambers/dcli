/*
* Copyright 2021 Mike Chambers
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

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::str_to_datetime;
use crate::response::utils::{property_to_value, standing_default};
use crate::{
    enums::platform::Platform,
    response::activities::{
        ActivityHistoricalStatsValues, DestinyHistoricalStatsActivity,
    },
};

pub const MAX_ACTIVITIES_REQUEST_COUNT: i32 = 250;

//https://bungie-net.github.io/multi/operation_get_Destiny2-GetPostGameCarnageReport.html#operation_get_Destiny2-GetPostGameCarnageReport
#[derive(Serialize, Deserialize, Debug)]
pub struct PGCRResponse {
    #[serde(rename = "Response")]
    pub response: Option<DestinyPostGameCarnageReportData>,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for PGCRResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportData.html#schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportData
#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyPostGameCarnageReportData {
    #[serde(rename = "startingPhaseIndex")]
    pub starting_phase_index: i32,

    #[serde(rename = "activityDetails")]
    pub activity_details: DestinyHistoricalStatsActivity,

    pub entries: Vec<DestinyPostGameCarnageReportEntry>,

    #[serde(skip_serializing, deserialize_with = "str_to_datetime")]
    pub period: DateTime<Utc>,

    pub teams: Vec<DestinyPostGameCarnageReportTeamEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyPostGameCarnageReportTeamEntry {
    #[serde(rename = "teamId")]
    pub team: i32,

    #[serde(rename = "teamName")]
    pub team_name: String,

    #[serde(deserialize_with = "property_to_value")]
    pub score: f32,

    #[serde(deserialize_with = "property_to_value")]
    pub standing: f32,
}

impl DestinyPostGameCarnageReportData {
    pub fn get_entry_for_character(
        &self,
        character_id: &str,
    ) -> Option<DestinyPostGameCarnageReportEntry> {
        for e in self.entries.iter() {
            if e.character_id == character_id {
                return Some(e.clone());
            }
        }

        None
    }
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportEntry.html#schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportEntry
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyPostGameCarnageReportEntry {
    #[serde(rename = "characterId")]
    pub character_id: String,

    pub extended: DestinyPostGameCarnageReportExtendedData,

    pub player: DestinyPlayer,

    #[serde(deserialize_with = "property_to_value")]
    pub score: f32,

    #[serde(default = "standing_default")]
    pub standing: u32,

    pub values: ActivityHistoricalStatsValues,
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPlayer.html#schema_Destiny-HistoricalStats-DestinyPlayer
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyPlayer {
    #[serde(rename = "destinyUserInfo")]
    pub user_info: UserInfoCard,

    #[serde(rename = "characterClass")]
    #[serde(default)]
    pub character_class: String,

    #[serde(rename = "classHash")]
    pub class_hash: u32,

    #[serde(rename = "raceHash")]
    pub race_hash: u32,

    #[serde(rename = "genderHash")]
    pub gender_hash: u32,

    #[serde(rename = "characterLevel")]
    pub character_level: i32,

    #[serde(rename = "lightLevel")]
    pub light_level: i32,

    #[serde(rename = "emblemHash")]
    pub emblem_hash: u32,
}

//https://bungie-net.github.io/multi/schema_User-UserInfoCard.html#schema_User-UserInfoCard
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfoCard {
    #[serde(rename = "iconPath")]
    #[serde(default)]
    pub icon_path: String,

    #[serde(rename = "crossSaveOverride")]
    pub cross_save_override: Platform,

    #[serde(rename = "applicableMembershipTypes")]
    pub applicable_membership_types: Option<Vec<Platform>>,

    #[serde(rename = "isPublic")]
    pub is_public: bool,

    #[serde(rename = "membershipType")]
    pub membership_type: Platform,

    #[serde(rename = "membershipId")]
    pub membership_id: String,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "bungieGlobalDisplayName")]
    pub bungie_display_name: Option<String>,

    #[serde(rename = "bungieGlobalDisplayNameCode")]
    pub bungie_display_name_code: Option<u32>,
}

impl UserInfoCard {
    pub fn get_bungie_display_name_code(&self) -> String {
        match self.bungie_display_name_code {
            Some(display_name_code) => {
                format!("{:0>4}", display_name_code)
            }
            None => "0000".to_string(),
        }
    }

    pub fn get_display_name(&self) -> String {
        match &self.display_name {
            Some(d) => d.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_bungie_display_name(&self) -> String {
        match &self.bungie_display_name {
            Some(d) => d.to_string(),
            None => "".to_string(),
        }
    }

    //TODO: we might not need these here, but somewhere else.
    pub fn has_bungie_display_name_code(&self) -> bool {
        return self.bungie_display_name_code.is_some();
    }

    /*
    pub fn generate_full_name(&self) -> String {
        //TODO: we can check if there is a bungie name, if not, use display name
        let out = if self.has_bungie_display_name_code() {
            format!("{}#{}", self.display_name, self.bungie_display_name_code)
        } else {
            self.bungie_display_name.to_string()
        };

        return out;
    }
    */
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportExtendedData.html#schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportExtendedData
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyPostGameCarnageReportExtendedData {
    pub values: HashMap<String, DestinyHistoricalStatsValue>,
    pub weapons: Option<Vec<DestinyHistoricalWeaponStats>>,
}

use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyHistoricalStatsValue {
    pub basic: DestinyHistoricalStatsValuePair,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyHistoricalStatsValuePair {
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyHistoricalWeaponStats {
    //maps to manifest DestinyInventoryItemDefinition
    #[serde(rename = "referenceId")]
    pub reference_id: u32, //TODO: should we make all ids u64?

    pub values: DestinyHistoricalWeaponsStatsValues,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinyHistoricalWeaponsStatsValues {
    #[serde(
        rename = "uniqueWeaponKills",
        deserialize_with = "property_to_value"
    )]
    #[serde(default)]
    pub unique_weapon_kills: f32,

    #[serde(
        rename = "uniqueWeaponPrecisionKills",
        deserialize_with = "property_to_value"
    )]
    #[serde(default)]
    pub unique_weapon_precision_kills: f32,

    #[serde(
        rename = "uniqueWeaponKillsPrecisionKills",
        deserialize_with = "property_to_value"
    )]
    #[serde(default)]
    pub unique_weapon_kills_precision_kills: f32,
}
