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

use crate::response::character::CharacterData;
use crate::{enums::character::CharacterClass, response::pgcr::UserInfoCard};

pub struct PlayerInfo {
    pub characters: Characters,
    pub user_info: UserInfoCard,
}

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

    pub fn get_by_class(
        &self,
        class_type: CharacterClass,
    ) -> Option<CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        for c in &self.characters {
            if c.class_type == class_type {
                return Some(c.clone());
            }
        }

        None
    }

    pub fn get_last_active(&self) -> Option<CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        Some(self.characters[0].clone())
    }

    pub fn get_by_class_ref(
        &self,
        class_type: CharacterClass,
    ) -> Option<&CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        for c in &self.characters {
            if c.class_type == class_type {
                return Some(c);
            }
        }

        None
    }

    pub fn get_last_active_ref(&self) -> Option<&CharacterData> {
        if self.characters.is_empty() {
            return None;
        }

        Some(&self.characters[0])
    }
}
