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

#[derive(Eq, PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum MedalTier {
    Tier1 = 802673300,
    Tier2 = 802673303,
    Tier3 = 802673302,
    Tier4 = 802673297,
    Tier5 = 802673296,
    Tier6 = 802673299,
    Tier7 = 802673298,
    Unknown = 0,
}
impl MedalTier {
    pub fn get_order(&self) -> u32 {
        match self {
            MedalTier::Tier1 => 700,
            MedalTier::Tier2 => 600,
            MedalTier::Tier3 => 500,
            MedalTier::Tier4 => 400,
            MedalTier::Tier5 => 300,
            MedalTier::Tier6 => 200,
            MedalTier::Tier7 => 100,
            MedalTier::Unknown => 0,
        }
    }
}
