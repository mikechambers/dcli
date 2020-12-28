use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result};

/****************CharacterGender *******************/
#[derive(PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug)]
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

impl Display for CharacterGender {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let out = match self {
            CharacterGender::Masculine => "Masculine",
            CharacterGender::Feminine => "Feminine",
        };

        check_width(out, f)
    }
}

/****************CharacterClass *******************/
#[derive(PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug)]
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
fn check_width(s: &str, f: &mut Formatter) -> Result {
    if let Some(width) = f.width() {
        write!(f, "{:width$}", s.to_string(), width = width)
    } else {
        write!(f, "{}", s)
    }
}
impl Display for CharacterClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let out = match self {
            CharacterClass::Titan => "Titan",
            CharacterClass::Hunter => "Hunter",
            CharacterClass::Warlock => "Warlock",
        };

        check_width(out, f)
    }
}

/*************************** CharacterRace *************************/

#[derive(PartialEq, Eq, Clone, Copy, Serialize_repr, Deserialize_repr, Debug)]
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

impl Display for CharacterRace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let out = match self {
            CharacterRace::Human => "Human",
            CharacterRace::Awoken => "Awoken",
            CharacterRace::Exo => "Exo",
        };

        check_width(out, f)
    }
}
