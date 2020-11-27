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

use chrono::{DateTime, Utc, Duration};
use crate::utils::get_last_reset;
use std::str::FromStr;

#[derive(PartialEq)]
pub enum TimePeriod {
    Day,
    Reset,
    Week,
    Month,
    Alltime,
}

impl TimePeriod {
    pub fn get_date_time(&self) -> DateTime<Utc> {

        match self {
            TimePeriod::Day => Utc::now() - Duration::hours(24),
            TimePeriod::Reset => {
                get_last_reset()
                // /let tomorrow_midnight = (now + Duration::days(1)).date().and_hms(0, 0, 0);
            },
            TimePeriod::Week => Utc::now() - Duration::days(7),
            TimePeriod::Month => Utc::now() - Duration::days(30),
            TimePeriod::Alltime => Utc::now() - Duration::weeks(7 * 52),
        }
    }
}

impl FromStr for TimePeriod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "day" => Ok(TimePeriod::Day),
            "reset" => Ok(TimePeriod::Reset),
            "week" => Ok(TimePeriod::Week),
            "month" => Ok(TimePeriod::Month),
            "alltime" => Ok(TimePeriod::Alltime),
            _ => Err("Unknown TimePeriod type"),
        }
    }
}
