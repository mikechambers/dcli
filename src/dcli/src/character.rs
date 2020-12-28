use crate::enums::character::CharacterClass;
use crate::response::character::CharacterData;

pub struct Characters {
    pub characters: Vec<CharacterData>,
}

impl Characters {
    pub fn with_characters(characters: Vec<CharacterData>) -> Characters {
        let mut out = Characters { characters };
        out.characters
            .sort_by(|a, b| b.date_last_played.cmp(&a.date_last_played));

        out
    }

    pub fn get_by_class_ref(&self, class_type: CharacterClass) -> Option<&CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        for c in &self.characters {
            if c.class_type == class_type {
                return Some(c);
            }
        }

        return None;
    }

    pub fn get_last_active_ref(&self) -> Option<&CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        return Some(&self.characters[0]);
    }
}
