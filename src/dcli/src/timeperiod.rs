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

use crate::utils::get_last_reset;
use chrono::{DateTime, Duration, Utc};
use std::fmt;
use std::str::FromStr;

pub struct DateTimePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(PartialEq, Debug)]
pub enum StatsTimePeriod {
    Yesterday,
    CurrentReset,
    LastReset,
    LastWeek,
    LastMonth,
    AllTime,
}

impl StatsTimePeriod {
    pub fn get_period(&self) -> DateTimePeriod {
        match self {
            //TODO: Right now, this period doesnt seem to ever return any data
            // https://github.com/mikechambers/dcli/issues/6
            StatsTimePeriod::Yesterday => {
                let n = Utc::now();

                DateTimePeriod {
                    start: n - Duration::hours(48),
                    end: Utc::now(),
                }
            }
            StatsTimePeriod::CurrentReset => DateTimePeriod {
                start: get_last_reset(),
                end: Utc::now(),
            },
            StatsTimePeriod::LastReset => {
                let reset = get_last_reset();
                DateTimePeriod {
                    start: reset - Duration::days(7),
                    end: reset,
                }
            }
            StatsTimePeriod::LastWeek => {
                let n = Utc::now();
                DateTimePeriod {
                    start: n - Duration::weeks(1),
                    end: n,
                }
            }
            StatsTimePeriod::LastMonth => {
                let n = Utc::now();
                DateTimePeriod {
                    start: n - Duration::days(30),
                    end: n,
                }
            }
            StatsTimePeriod::AllTime => {
                let n = Utc::now();
                DateTimePeriod {
                    start: n - Duration::weeks(7 * 52),
                    end: n,
                }
            }
        }
    }
}

impl FromStr for StatsTimePeriod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "yesterday" => Ok(StatsTimePeriod::Yesterday),
            "currentreset" => Ok(StatsTimePeriod::CurrentReset),
            "lastreset" => Ok(StatsTimePeriod::LastReset),
            "lastweek" => Ok(StatsTimePeriod::LastWeek),
            "lastmonth" => Ok(StatsTimePeriod::LastMonth),
            "alltime" => Ok(StatsTimePeriod::AllTime),
            _ => Err("Unknown StatsTimePeriod type"),
        }
    }
}

impl fmt::Display for StatsTimePeriod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            StatsTimePeriod::Yesterday => "yesterday",
            StatsTimePeriod::CurrentReset => "currentreset",
            StatsTimePeriod::LastReset => "lastreset",
            StatsTimePeriod::LastWeek => "lastweek",
            StatsTimePeriod::LastMonth => "lastmonth",
            StatsTimePeriod::AllTime => "alltime",
        };

        write!(f, "{}", out)
    }
}
