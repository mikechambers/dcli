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


use crate::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use crate::response::utils::str_to_datetime;
use crate::response::utils::{property_to_value, standing_default};
use crate::response::activities::{DestinyHistoricalStatsActivity, ActivityHistoricalStatsValues};

use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

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
    starting_phase_index:i32,

    #[serde(rename = "activityDetails")]
    activity_details:DestinyHistoricalStatsActivity,

    entries:Vec<DestinyPostGameCarnageReportEntry>,

    #[serde(skip_serializing, deserialize_with = "str_to_datetime")]
    pub period: DateTime<Utc>,

    //teams,
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportEntry.html#schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportEntry
#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyPostGameCarnageReportEntry {

    #[serde(rename = "characterId")]
    character_id:String,

    extended:DestinyPostGameCarnageReportExtendedData,

    player:DestinyPlayer,

    #[serde(deserialize_with = "property_to_value")]
    score:f32,

    #[serde(default="standing_default")]
    standing:i32,

    values:ActivityHistoricalStatsValues,
}

//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPlayer.html#schema_Destiny-HistoricalStats-DestinyPlayer
#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyPlayer {

    #[serde(rename = "destinyUserInfo")]
    user_info:UserInfoCard,

    #[serde(rename = "characterClass")]
    #[serde(default)]
    character_class: String,

    #[serde(rename = "classHash")]
    class_hash: u32,

    #[serde(rename = "raceHash")]
    race_hash: u32,

    #[serde(rename = "genderHash")]
    gender_hash: u32,

    #[serde(rename = "characterLevel")]
    character_level: i32,

    #[serde(rename = "lightLevel")]
    light_level: i32,

    #[serde(rename = "emblemHash")]
    emblem_hash: u32,
}

//https://bungie-net.github.io/multi/schema_User-UserInfoCard.html#schema_User-UserInfoCard
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoCard {

    #[serde(rename = "iconPath")]
    #[serde(default)]
    icon_path: String,

    #[serde(rename = "crossSaveOverride")]
    cross_save_override: i32,


    #[serde(rename = "applicableMembershipTypes")]
    applicable_membership_types: Option<Vec<i32>>,

    #[serde(rename = "isPublic")]
    is_public: bool,

    #[serde(rename = "membershipType")]
    membership_type: i32,

    #[serde(rename = "membershipId")]
    membership_id: String,

    #[serde(rename = "displayName")]
    #[serde(default)]
    display_name: String,
}


//https://bungie-net.github.io/multi/schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportExtendedData.html#schema_Destiny-HistoricalStats-DestinyPostGameCarnageReportExtendedData
#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyPostGameCarnageReportExtendedData {
    values:ExtendedActivityHistoricalStatsValues,
    weapons:Option<Vec<DestinyHistoricalWeaponStats>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyHistoricalWeaponStats {

    //maps to manifest DestinyInventoryItemDefinition
    #[serde(rename = "referenceId")]
    pub reference_id:u32, //TODO: should we make all ids u64?

    #[serde(rename = "uniqueWeaponKills", deserialize_with = "property_to_value")]
    #[serde(default)]
    pub unique_weapon_kills:f32,

    #[serde(rename = "uniqueWeaponPrecisionKills", deserialize_with = "property_to_value")]
    #[serde(default)]
    pub unique_weapon_precision_kills:f32,

    #[serde(rename = "uniqueWeaponKillsPrecisionKills", deserialize_with = "property_to_value")]
    #[serde(default)]
    pub unique_weapon_kills_precision_kills:f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedActivityHistoricalStatsValues {

    #[serde(rename = "allMedalsEarned", deserialize_with = "property_to_value")]
    #[serde(default)]
    pub all_medals_earned: f32,

    /*
    //TODO: these properties are dynamic, need to figure out how to parse / 
    //represent
    pub medalAvenger:f32,
    pub medalControlAdvantageStreak:f32,
    pub medalControlMostAdvantage:f32,
    pub medalDefeatHunterDodge:f32,
    pub medalMatchMostDamage:f32,
    pub medalMulti2x:f32,
    pub medalPayback:f32,
    pub medalStreak10x:f32,
    pub medalStreak5x:f32,
    pub medalStreakCombined:f32,
    pub medalStreakTeam:f32,
    pub medalWeaponHandCannon:f32,
    pub medalWeaponSword:f32,
    */

    #[serde(rename = "precisionKills", deserialize_with = "property_to_value")]
    pub precision_kills:f32,

    #[serde(rename = "weaponKillsAbility", deserialize_with = "property_to_value")]
    pub weapon_kills_ability:f32,

    #[serde(rename = "weaponKillsGrenade", deserialize_with = "property_to_value")]
    pub weapon_kills_grenade:f32,

    #[serde(rename = "weaponKillsMelee", deserialize_with = "property_to_value")]
    pub weapon_kills_melee:f32,

    #[serde(rename = "weaponKillsSuper", deserialize_with = "property_to_value")]
    pub weapon_kills_super:f32,
}