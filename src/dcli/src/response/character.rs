/*
* Copyright 2022 Mike Chambers
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

use crate::enums::character::{CharacterClass, CharacterGender, CharacterRace};

use crate::response::utils::str_to_int;

use crate::emblem::Emblem;
use crate::response::utils::str_to_datetime;

use crate::response::utils::string_to_i64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CharacterData {
    #[serde(rename = "characterId", deserialize_with = "string_to_i64")]
    pub id: i64,

    #[serde(
        rename = "dateLastPlayed",
        skip_serializing,
        deserialize_with = "str_to_datetime"
    )]
    pub date_last_played: DateTime<Utc>, //TODO: parse 2020-10-05T18:49:25Z

    #[serde(rename = "minutesPlayedTotal", deserialize_with = "str_to_int")]
    pub minutes_played_total: u32,

    #[serde(rename = "raceType")]
    pub race: CharacterRace,

    #[serde(rename = "classType")]
    pub class_type: CharacterClass,

    #[serde(rename = "genderType")]
    pub gender: CharacterGender,

    #[serde(rename = "emblemHash")]
    pub emblem_hash: u32, //TODO: check int type

    #[serde(rename = "baseCharacterLevel")]
    pub base_character_level: u32,

    #[serde(skip)]
    pub emblem: Option<Emblem>,

    pub stats: CharacterStatsData,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterStatsData {
    #[serde(rename = "1935470627")]
    pub power: i32,

    #[serde(rename = "2996146975")]
    pub mobility: i32,

    #[serde(rename = "392767087")]
    pub resilience: i32,

    #[serde(rename = "1943323491")]
    pub recovery: i32,

    #[serde(rename = "1735777505")]
    pub discipline: i32,

    #[serde(rename = "144602215")]
    pub intellect: i32,

    #[serde(rename = "4244567218")]
    pub strength: i32,
}
