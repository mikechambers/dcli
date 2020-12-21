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

use crate::mode::Mode;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum Standing {
    Victory = 0,
    Defeat = 1,
    Unknown = -1,
}

impl Default for Standing {
    fn default() -> Self {
        Standing::Unknown
    }
}

impl Standing {
    pub fn from_mode(value: i32, mode: &Mode) -> Standing {
        if value == 0 {
            return Standing::Victory;
        } else if value == -1 {
            return Standing::Unknown;
        }

        if value > 0 {
            if mode == &Mode::Rumble && value > 2 {
                return Standing::Defeat;
            } else {
                return Standing::Victory;
            }
        } else {
            return Standing::Victory;
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
