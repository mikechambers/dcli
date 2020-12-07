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