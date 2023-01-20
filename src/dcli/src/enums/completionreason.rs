/*
* Copyright 2023 Mike Chambers
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

///Destiny 2 Platforms
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(i32)]
pub enum CompletionReason {
    ObjectiveComplete = 0,
    Failed = 2,
    TimerFinished = 1,
    NoOpponents = 3,
    Mercy = 4,
    Unknown = 255,
}

impl CompletionReason {
    pub fn as_id(&self) -> u32 {
        *self as u32
    }

    pub fn from_id(id: u32) -> CompletionReason {
        match id {
            0 => CompletionReason::ObjectiveComplete,
            1 => CompletionReason::TimerFinished,
            2 => CompletionReason::Failed,
            3 => CompletionReason::NoOpponents,
            4 => CompletionReason::Mercy,
            255 => CompletionReason::Unknown,
            _ => CompletionReason::Unknown,
        }
    }
}

impl fmt::Display for CompletionReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            CompletionReason::ObjectiveComplete => "Objective Complete",
            CompletionReason::TimerFinished => "Timer Finished",
            CompletionReason::Failed => "Failed",
            CompletionReason::NoOpponents => "No Opponents",
            CompletionReason::Mercy => "Mercy",
            CompletionReason::Unknown => "Unknown",
        };

        write!(f, "{}", out)
    }
}
