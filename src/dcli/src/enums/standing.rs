/*
* Copyright 2021 Mike Chambers
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

use crate::enums::mode::Mode;

pub const STANDING_UNKNOWN_MAGIC_NUMBER: u32 = 2325;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u32)]
pub enum Standing {
    Victory = 0,
    Defeat = 1,
    Unknown = STANDING_UNKNOWN_MAGIC_NUMBER,
}

impl Default for Standing {
    fn default() -> Self {
        Standing::Unknown
    }
}

impl Standing {
    //todo: any value except 1 is probably defeat
    pub fn from_value(value: u32) -> Standing {
        if value == 0 {
            Standing::Victory
        } else if value == 1 {
            Standing::Defeat
        } else {
            Standing::Unknown
        }
    }

    pub fn from_mode(value: u32, mode: &Mode) -> Standing {
        if value == 0 {
            return Standing::Victory;
        } else if value == STANDING_UNKNOWN_MAGIC_NUMBER {
            return Standing::Unknown;
        }

        if value > 0 {
            if mode == &Mode::Rumble && value > 2 {
                Standing::Defeat
            } else {
                Standing::Victory
            }
        } else {
            Standing::Victory
        }
    }
}

impl fmt::Display for Standing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Standing::Victory => "Win",
            Standing::Defeat => "Loss",
            Standing::Unknown => "Unknown",
        };

        write!(f, "{}", out)
    }
}
