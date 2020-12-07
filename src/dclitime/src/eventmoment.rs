use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc, Duration};

use dcli::utils::{
    get_last_weekly_reset,
    get_last_friday_reset,
    get_last_daily_reset
};

#[derive(PartialEq, Debug)]
pub enum EventMoment {
    Now,
    LastWeeklyReset,
    NextWeeklyReset,
    LastDailyReset,
    NextDailyReset,
    LastXurReset,
    NextXurReset,
    LastTrialsReset,
    NextTrialsReset,
}


impl EventMoment {
    pub fn get_date_time(&self) -> DateTime<Utc> {
        match self {
            EventMoment::Now => Utc::now(),
            EventMoment::LastWeeklyReset => get_last_weekly_reset(),
            EventMoment::NextWeeklyReset => get_last_weekly_reset() + Duration::weeks(1),
            EventMoment::LastDailyReset => Utc::now(),
            EventMoment::NextDailyReset => Utc::now(),
            EventMoment::LastXurReset => get_last_friday_reset(),
            EventMoment::NextXurReset => get_last_friday_reset() + Duration::weeks(1),
            EventMoment::LastTrialsReset => get_last_friday_reset(),
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
            "lastweeklyreset" => Ok(EventMoment::LastWeeklyReset),
            "nextweeklyreset" => Ok(EventMoment::NextWeeklyReset),
            "lastdailyreset" => Ok(EventMoment::LastDailyReset),
            "nextdailyreset" => Ok(EventMoment::NextDailyReset),
            "lastxureset" => Ok(EventMoment::LastXurReset),
            "Nextxurreset" => Ok(EventMoment::NextXurReset),
            "lasttrialsreset" => Ok(EventMoment::LastTrialsReset),
            "nexttrialsreset" => Ok(EventMoment::NextTrialsReset),

            _ => Err("Unknown EventMoment type"),
        }
    }
}

impl fmt::Display for EventMoment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            EventMoment::Now => "Now",
            EventMoment::LastWeeklyReset => " Last Weekly Reset",
            EventMoment::NextWeeklyReset => "Next Weekly Reset",
            EventMoment::LastDailyReset => "Last Daily Reset",
            EventMoment::NextDailyReset => "Next Daily Reset",
            EventMoment::LastXurReset => "Last Weekly Xur Reset",
            EventMoment::NextXurReset => "Next Weekly Xur Reset",
            EventMoment::LastTrialsReset => "Last Trials of Osiris Weekly Reset",
            EventMoment::NextTrialsReset => "Next Trials of Osiris Weekly Reset",
        };

        write!(f, "{}", out)
    }
}