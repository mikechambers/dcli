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
use std::str::FromStr;

use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

use crate::utils::{
    get_destiny2_launch_date, get_last_daily_reset, get_last_friday_reset,
    get_last_weekly_reset, Period,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Moment {
    Now,
    Daily,
    NextDaily,
    Weekend,
    NextWeekend,
    Weekly,
    NextWeekly,
    Day,
    NextDay,
    Week,
    NextWeek,
    Month,
    NextMonth,
    AllTime,
    Custom,
}

impl Moment {
    pub fn get_date_time(&self) -> DateTime<Utc> {
        match self {
            Moment::Now => Utc::now(),
            Moment::Daily => get_last_daily_reset(),
            Moment::NextDaily => get_last_daily_reset() + Duration::days(1),
            Moment::Weekend => get_last_friday_reset(),
            Moment::NextWeekend => get_last_friday_reset() + Duration::days(1),
            Moment::Weekly => get_last_weekly_reset(),
            Moment::NextWeekly => get_last_weekly_reset() + Duration::days(1),
            Moment::Day => Utc::now() - Duration::days(1),
            Moment::NextDay => Utc::now() + Duration::days(1),
            Moment::Week => Utc::now() - Duration::weeks(1),
            Moment::NextWeek => Utc::now() + Duration::weeks(1),
            Moment::Month => Utc::now() - Duration::days(30),
            Moment::NextMonth => Utc::now() + Duration::days(30),
            Moment::AllTime => get_destiny2_launch_date(),
            Moment::Custom => Utc.ymd(0, 0, 0).and_hms(0, 0, 0),
        }
    }
}

impl FromStr for Moment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "now" => Ok(Moment::Now),
            "daily" => Ok(Moment::Daily),
            "next_daily" => Ok(Moment::NextDaily),
            "weekend" => Ok(Moment::Weekend),
            "next_weekend" => Ok(Moment::NextWeekend),
            "weekly" => Ok(Moment::Weekly),
            "next_weekly" => Ok(Moment::NextWeekly),
            "day" => Ok(Moment::Day),
            "next_day" => Ok(Moment::NextDay),
            "week" => Ok(Moment::Week),
            "next_week" => Ok(Moment::NextWeek),
            "month" => Ok(Moment::Month),
            "next_month" => Ok(Moment::NextMonth),
            "all_time" => Ok(Moment::AllTime),
            "custom" => Ok(Moment::Custom),

            _ => Err("Unknown Moment type"),
        }
    }
}

impl fmt::Display for Moment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Moment::Now => "now",
            Moment::Daily => "last daily reset",
            Moment::NextDaily => "next daily reset",
            Moment::Weekend => "last weekend reset",
            Moment::NextWeekend => "next weekend reset",
            Moment::Weekly => "last weekly reset",
            Moment::NextWeekly => "next weekly reset",
            Moment::Day => "last day",
            Moment::NextDay => "next day",
            Moment::Week => "last week",
            Moment::NextWeek => "next week",
            Moment::Month => "last month",
            Moment::NextMonth => "next month",
            Moment::AllTime => "all time",

            //TODO: is there a way to store the date / time with custom?
            //now that we are parsing ourselves?
            //can it default to none? Probably not
            Moment::Custom => "custom",
        };

        write!(f, "{}", out)
    }
}

pub struct MomentPeriod {
    pub moment: Moment,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Period for MomentPeriod {
    fn get_start(&self) -> DateTime<Utc> {
        self.start
    }

    fn get_end(&self) -> DateTime<Utc> {
        self.end
    }
}

impl MomentPeriod {
    ///Takes a Moment and creates MomentPeriod relative to now.
    ///If Moment is in the future, it will be from now to moment,
    ///If Moment is in the past, it will be from moment to now.
    pub fn from_moment(moment: Moment) -> MomentPeriod {
        let mut start = moment.get_date_time();
        let mut end = Utc::now();

        if start > end {
            std::mem::swap(&mut start, &mut end);
        }

        MomentPeriod { moment, start, end }
    }
}
