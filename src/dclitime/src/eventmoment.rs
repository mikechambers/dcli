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

use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};

use dcli::utils::{get_last_daily_reset, get_last_friday_reset, get_last_weekly_reset};

#[derive(PartialEq, Debug)]
pub enum EventMoment {
    Now,
    CurrentWeeklyReset,
    NextWeeklyReset,
    CurrentDailyReset,
    NextDailyReset,
    CurrentXurReset,
    NextXurReset,
    CurrentTrialsReset,
    NextTrialsReset,
}

impl EventMoment {
    pub fn get_date_time(&self) -> DateTime<Utc> {
        match self {
            EventMoment::Now => Utc::now(),
            EventMoment::CurrentWeeklyReset => get_last_weekly_reset(),
            EventMoment::NextWeeklyReset => get_last_weekly_reset() + Duration::weeks(1),
            EventMoment::CurrentDailyReset => get_last_daily_reset(),
            EventMoment::NextDailyReset => get_last_daily_reset() + Duration::days(1),
            EventMoment::CurrentXurReset => get_last_friday_reset(),
            EventMoment::NextXurReset => get_last_friday_reset() + Duration::weeks(1),
            EventMoment::CurrentTrialsReset => get_last_friday_reset(),
            EventMoment::NextTrialsReset => get_last_friday_reset() + Duration::weeks(1),
        }
    }
}

impl FromStr for EventMoment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "now" => Ok(EventMoment::Now),
            "current_weekly" => Ok(EventMoment::CurrentWeeklyReset),
            "next_weekly" => Ok(EventMoment::NextWeeklyReset),
            "current_daily" => Ok(EventMoment::CurrentDailyReset),
            "next_daily" => Ok(EventMoment::NextDailyReset),
            "current_xur" => Ok(EventMoment::CurrentXurReset),
            "next_xur" => Ok(EventMoment::NextXurReset),
            "current_trials" => Ok(EventMoment::CurrentTrialsReset),
            "next_trials" => Ok(EventMoment::NextTrialsReset),

            _ => Err("Unknown EventMoment type"),
        }
    }
}

impl fmt::Display for EventMoment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            EventMoment::Now => "Now",
            EventMoment::CurrentWeeklyReset => " Current Weekly Reset",
            EventMoment::NextWeeklyReset => "Next Weekly Reset",
            EventMoment::CurrentDailyReset => "Current Daily Reset",
            EventMoment::NextDailyReset => "Next Daily Reset",
            EventMoment::CurrentXurReset => "Current Weekly Xur Reset",
            EventMoment::NextXurReset => "Next Weekly Xur Reset",
            EventMoment::CurrentTrialsReset => "Current Trials of Osiris Weekly Reset",
            EventMoment::NextTrialsReset => "Next Trials of Osiris Weekly Reset",
        };

        write!(f, "{}", out)
    }
}
