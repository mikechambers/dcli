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
use std::str::FromStr;

///Destiny 2 Platforms
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Platform {
    ///Xbox
    Xbox = 1,

    ///Playstation
    Playstation = 2,

    ///Steam
    Steam = 3,

    ///Stadia
    Stadia = 5,

    Blizzard = 4,

    Unknown = 0,
}

/*
    https://bungie-net.github.io/multi/schema_BungieMembershipType.html#schema_BungieMembershipType
    None: 0
    TigerXbox: 1
    TigerPsn: 2
    TigerSteam: 3
    TigerBlizzard: 4
    TigerStadia: 5
    TigerDemon: 10
    BungieNext: 254
*/
impl Platform {
    pub fn to_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u64) -> Platform {
        match id {
            1 => Platform::Xbox,
            2 => Platform::Playstation,
            3 => Platform::Steam,
            4 => Platform::Blizzard,
            5 => Platform::Stadia,
            _ => Platform::Unknown,
        }
    }
}

impl FromStr for Platform {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "xbox" => Ok(Platform::Xbox),
            "playstation" => Ok(Platform::Playstation),
            "steam" => Ok(Platform::Steam),
            "stadia" => Ok(Platform::Stadia),
            "blizzard" => Ok(Platform::Blizzard),
            _ => Err("Unknown platform type"),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Platform::Xbox => "Xbox",
            Platform::Playstation => "Playstation",
            Platform::Steam => "Steam",
            Platform::Stadia => "Stadia",
            Platform::Blizzard => "Blizzard",
            Platform::Unknown => "Unknown",
        };

        write!(f, "{}", out)
    }
}
