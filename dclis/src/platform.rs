//use std::string::ParseError;
use std::fmt;
use std::str::FromStr;

///Destiny 2 Platforms
#[derive(PartialEq)]
pub enum Platform {
    ///Xbox
    Xbox,

    ///Playstation
    Playstation,

    ///Steam
    Steam,

    ///Stadia
    Stadia,

    Blizzard,

    Unknown,
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
        match self {
            Platform::Unknown => 0,
            Platform::Xbox => 1,
            Platform::Playstation => 2,
            Platform::Steam => 3,
            Platform::Blizzard => 4,
            Platform::Stadia => 5,
        }
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
