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

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
//use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::emblem::Emblem;
use crate::response::utils::str_to_datetime;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CharacterData {
    #[serde(rename = "characterId")]
    pub id: String,

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

//todo: move this to more central area
fn str_to_int<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct CharacterStatsData {
    #[serde(rename = "1935470627")]
    pub power: u32,

    #[serde(rename = "2996146975")]
    pub mobility: u32,

    #[serde(rename = "392767087")]
    pub resilience: u32,

    #[serde(rename = "1943323491")]
    pub recovery: u32,

    #[serde(rename = "1735777505")]
    pub discipline: u32,

    #[serde(rename = "144602215")]
    pub intellect: u32,

    #[serde(rename = "4244567218")]
    pub strength: u32,
}

/****************CharacterGender *******************/
#[derive(
    PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug,
)]
#[repr(u32)]
pub enum CharacterGender {
    Masculine = 0,
    Feminine = 1,
}

impl CharacterGender {
    pub fn to_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u64) -> CharacterGender {
        match id {
            0 => CharacterGender::Masculine,
            1 => CharacterGender::Feminine,
            _ => panic!("Unknkown Character Gender Id : {}", id),
        }
    }
}

impl fmt::Display for CharacterGender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            CharacterGender::Masculine => "Masculine",
            CharacterGender::Feminine => "Feminine",
        };

        check_width(out, f)
    }
}

/****************CharacterClass *******************/
#[derive(
    PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug,
)]
#[repr(u32)]
pub enum CharacterClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
}

impl CharacterClass {
    pub fn to_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u64) -> CharacterClass {
        match id {
            0 => CharacterClass::Titan,
            1 => CharacterClass::Hunter,
            2 => CharacterClass::Warlock,
            _ => panic!("Unknkown Character Class Id : {}", id),
        }
    }
}
fn check_width(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(width) = f.width() {
        write!(f, "{:width$}", s.to_string(), width = width)
    } else {
        write!(f, "{}", s)
    }
}
impl fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            CharacterClass::Titan => "Titan",
            CharacterClass::Hunter => "Hunter",
            CharacterClass::Warlock => "Warlock",
        };

        check_width(out, f)
    }
}

/*************************** CharacterRace *************************/

#[derive(
    PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug,
)]
#[repr(u32)]
pub enum CharacterRace {
    Human = 0,
    Awoken = 1,
    Exo = 2,
}

impl CharacterRace {
    pub fn to_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u64) -> CharacterRace {
        match id {
            0 => CharacterRace::Human,
            1 => CharacterRace::Awoken,
            2 => CharacterRace::Exo,
            _ => panic!("Unknkown Character Race Id : {}", id),
        }
    }
}

impl fmt::Display for CharacterRace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            CharacterRace::Human => "Human",
            CharacterRace::Awoken => "Awoken",
            CharacterRace::Exo => "Exo",
        };

        check_width(out, f)
    }
}
