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

use serde_derive::Serialize;
use std::fmt;

#[derive(Serialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Standing {
    Victory = 0,
    Defeat = 1,
    Unknown = 2,
}

impl Default for Standing {
    fn default() -> Self {
        Standing::Unknown
    }
}

impl Standing {
    pub fn from_f32(value: f32) -> Standing {
        if value == 1.0 {
            Standing::Defeat
        } else if value == 0.0 {
            Standing::Victory
        } else {
            Standing::Unknown
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