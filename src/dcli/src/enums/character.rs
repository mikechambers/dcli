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

use serde_repr::{Deserialize_repr, Serialize_repr};

use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub enum CharacterClassSelection {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    LastActive = 3,
    All = 4,
}

impl FromStr for CharacterClassSelection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "titan" => Ok(CharacterClassSelection::Titan),
            "hunter" => Ok(CharacterClassSelection::Hunter),
            "warlock" => Ok(CharacterClassSelection::Warlock),
            "last_active" => Ok(CharacterClassSelection::LastActive),
            "all" => Ok(CharacterClassSelection::All),
            _ => Err("Unknown CharacterClassSelection type"),
        }
    }
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
    pub fn as_id(&self) -> u32 {
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
    PartialEq, Eq, Hash, Clone, Copy, Serialize_repr, Deserialize_repr, Debug,
)]
#[repr(u32)]
pub enum CharacterClass {
    Titan = 0,
    Hunter = 1,
    Warlock = 2,
    Unknown = 255,
}

impl CharacterClass {
    pub fn as_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u32) -> CharacterClass {
        match id {
            0 => CharacterClass::Titan,
            1 => CharacterClass::Hunter,
            2 => CharacterClass::Warlock,
            _ => CharacterClass::Unknown,
        }
    }
    pub fn from_hash(id: u32) -> CharacterClass {
        match id {
            3655393761 => CharacterClass::Titan,
            671679327 => CharacterClass::Hunter,
            2271682572 => CharacterClass::Warlock,
            _ => CharacterClass::Unknown,
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
            CharacterClass::Unknown => "Unknown",
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
    pub fn as_id(&self) -> u32 {
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
