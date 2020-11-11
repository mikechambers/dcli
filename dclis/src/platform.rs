//use std::string::ParseError;
use std::fmt;
use std::str::FromStr;

///Destiny 2 Platforms
pub enum Platform {
    ///Xbox
    Xbox,

    ///Playstation
    Playstation,

    ///Steam
    Steam,

    ///Stadia
    Stadia,
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
            Platform::Xbox => 1,
            Platform::Playstation => 2,
            Platform::Steam => 3,
            Platform::Stadia => 5,
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
        };

        write!(f, "{}", out)
    }
}
