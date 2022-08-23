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

use std::fmt;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};

use crate::{
    error::Error,
    utils::{
        get_destiny2_launch_date, get_last_daily_reset, get_last_friday_reset,
        get_last_weekly_reset,
    },
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

    Launch,
    CurseOfOsiris,
    Warmind,
    SeasonOfTheOutlaw,
    SeasonOfTheForge,
    SeasonOfTheDrifter,
    SeasonOfOpulence,
    SeasonOfTheUndying,
    SeasonOfDawn,
    SeasonOfTheWorthy,
    SeasonOfArrivals,
    SeasonOfTheHunt,
    SeasonOfTheChosen,
    SeasonOfTheSplicer,
    SeasonOfTheLost,
    SeasonOfTheRisen,
    WitchQueen,
    SeasonOfTheHaunted,
    SeasonOfThePlunder,
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

            Moment::Launch => Utc.ymd(2017, 9, 6).and_hms(0, 0, 1),
            Moment::CurseOfOsiris => Utc.ymd(2017, 12, 5).and_hms(17, 0, 0),
            Moment::Warmind => Utc.ymd(2018, 5, 8).and_hms(17, 0, 0),
            Moment::SeasonOfTheOutlaw => Utc.ymd(2018, 9, 4).and_hms(17, 0, 0),
            Moment::SeasonOfTheForge => Utc.ymd(2018, 12, 4).and_hms(17, 0, 0),
            Moment::SeasonOfTheDrifter => Utc.ymd(2019, 3, 5).and_hms(17, 0, 0),
            Moment::SeasonOfOpulence => Utc.ymd(2019, 6, 4).and_hms(17, 0, 0),
            Moment::SeasonOfTheUndying => {
                Utc.ymd(2019, 10, 1).and_hms(17, 0, 0)
            }
            Moment::SeasonOfDawn => Utc.ymd(2019, 12, 10).and_hms(17, 0, 0),
            Moment::SeasonOfTheWorthy => Utc.ymd(2020, 3, 10).and_hms(17, 0, 0),
            Moment::SeasonOfArrivals => Utc.ymd(2020, 6, 9).and_hms(17, 0, 0),
            Moment::SeasonOfTheHunt => Utc.ymd(2020, 11, 10).and_hms(17, 0, 0),
            Moment::SeasonOfTheChosen => Utc.ymd(2021, 2, 9).and_hms(17, 0, 0),
            Moment::SeasonOfTheSplicer => {
                Utc.ymd(2021, 5, 11).and_hms(17, 0, 0)
            }
            Moment::SeasonOfTheLost => Utc.ymd(2021, 8, 24).and_hms(17, 0, 0),
            Moment::SeasonOfTheRisen => Utc.ymd(2022, 2, 22).and_hms(17, 0, 0),
            Moment::WitchQueen => Utc.ymd(2022, 2, 22).and_hms(17, 0, 0),
            Moment::SeasonOfTheHaunted => {
                Utc.ymd(2022, 5, 24).and_hms(17, 0, 0)
            }

            Moment::SeasonOfThePlunder => {
                Utc.ymd(2022, 8, 23).and_hms(17, 0, 0)
            }
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

            "launch" => Ok(Moment::Launch),
            "curse_of_osiris" => Ok(Moment::CurseOfOsiris),
            "warmind" => Ok(Moment::Warmind),
            "season_of_the_outlaw" => Ok(Moment::SeasonOfTheOutlaw),
            "season_of_the_forge" => Ok(Moment::SeasonOfTheForge),
            "season_of_the_drifter" => Ok(Moment::SeasonOfTheDrifter),
            "season_of_opulence" => Ok(Moment::SeasonOfOpulence),
            "season_of_the_undying" => Ok(Moment::SeasonOfTheUndying),
            "season_of_dawn" => Ok(Moment::SeasonOfDawn),
            "season_of_the_worthy" => Ok(Moment::SeasonOfTheWorthy),
            "season_of_arrivals" => Ok(Moment::SeasonOfArrivals),
            "season_of_the_hunt" => Ok(Moment::SeasonOfTheHunt),
            "season_of_the_chosen" => Ok(Moment::SeasonOfTheChosen),
            "season_of_the_splicer" => Ok(Moment::SeasonOfTheSplicer),
            "season_of_the_lost" => Ok(Moment::SeasonOfTheLost),
            "season_of_the_risen" => Ok(Moment::SeasonOfTheRisen),
            "witch_queen" => Ok(Moment::WitchQueen),
            "season_of_the_haunted" => Ok(Moment::SeasonOfTheHaunted),
            "season_of_the_plunder" => Ok(Moment::SeasonOfThePlunder),

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

            Moment::Custom => "custom",

            Moment::Launch => "launch",
            Moment::CurseOfOsiris => "Curse of Osiris",
            Moment::Warmind => "Warmind",
            Moment::SeasonOfTheOutlaw => "Season of the Outlaw",
            Moment::SeasonOfTheForge => "Season of the Forge",
            Moment::SeasonOfTheDrifter => "Season of the Drifter",
            Moment::SeasonOfOpulence => "Season of Opulence",
            Moment::SeasonOfTheUndying => "Season of the Undying",
            Moment::SeasonOfDawn => "Season of Dawn",
            Moment::SeasonOfTheWorthy => "Season of the Worthy",
            Moment::SeasonOfArrivals => "Season of Arrivals",
            Moment::SeasonOfTheHunt => "Season of the Hunt",
            Moment::SeasonOfTheChosen => "Season of the Chosen",
            Moment::SeasonOfTheSplicer => "Season of the Splicer",
            Moment::SeasonOfTheLost => "Season of the Lost",
            Moment::SeasonOfTheRisen => "Season of the Risen",
            Moment::WitchQueen => "The Witch Queen",
            Moment::SeasonOfTheHaunted => "Season of the Haunted",
            Moment::SeasonOfThePlunder => "Season of the Plunder",
        };

        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct DateTimePeriod {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl DateTimePeriod {
    pub fn get_start(&self) -> DateTime<Utc> {
        self.start
    }

    pub fn get_end(&self) -> DateTime<Utc> {
        self.end
    }

    pub fn with_start_time(
        start: DateTime<Utc>,
    ) -> Result<DateTimePeriod, Error> {
        let end = Utc::now();

        if start > end {
            return Err(Error::DateTimePeriodOrder);
        }

        Ok(DateTimePeriod { start, end })
    }

    pub fn with_start_end_time(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<DateTimePeriod, Error> {
        if start > end {
            return Err(Error::DateTimePeriodOrder);
        }

        Ok(DateTimePeriod { start, end })
    }
}
