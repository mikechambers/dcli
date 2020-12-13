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

use crate::error::Error;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};
use std::env;
use std::ffi::OsStr;
use std::path::Path;

use crossterm::{execute, terminal};
use std::io::{stdout, Write};

//use chrono::prelude::*;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub const WEEK_IN_SECONDS: i64 = 604800;
pub const DAY_IN_SECONDS: i64 = 86400;

pub const TSV_EOL: &str = "\n";
pub const TSV_DELIM: &str = "\t";

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn f32_are_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

pub trait Period {
    fn get_start(&self) -> DateTime<Utc>;
    fn get_end(&self) -> DateTime<Utc>;
}

pub fn print_verbose(msg: &str, verbose: bool) {
    if !verbose {
        return;
    }

    eprintln!("{}", msg);
}

pub fn print_error(msg: &str, error: Error) {
    let app_name = env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .unwrap_or_else(|| "".to_string());

    eprintln!("{} : v{}", app_name, VERSION);

    eprintln!("{}", msg);
    eprintln!("{}", error);

    match error {
        Error::InvalidParameters => {
            eprintln!("This can occur if --platform is set incorrectly.");
        }
        Error::ParameterParseFailure => {
            eprintln!("This can occur if --member-id or --character-id were entered incorrectly.");
        }
        _ => {}
    }

    eprintln!();
    eprintln!("If you think you have hit a bug and would like to report it (or would just like some help):");
    eprintln!("    1. Run command with '--verbose' flag.");
    eprintln!("    2. Copy output, and log a bug at: ");
    eprintln!("       https://github.com/mikechambers/dcli/issues");
}

pub fn calculate_per_activity_average(value: f32, total_activities: f32) -> f32 {
    if total_activities == 0.0 {
        return 0.0;
    }

    value / total_activities
}

pub fn calculate_efficiency(kills: f32, deaths: f32, assists: f32) -> f32 {
    let t = kills + assists;
    if deaths > 0.0 {
        t / deaths
    } else {
        t
    }
}

pub fn calculate_kills_deaths_ratio(kills: f32, deaths: f32) -> f32 {
    if deaths > 0.0 {
        kills / deaths
    } else {
        kills
    }
}

pub fn calculate_kills_deaths_assists(kills: f32, deaths: f32, assists: f32) -> f32 {
    let t = kills + (assists / 2.0);
    if deaths > 0.0 {
        t / deaths
    } else {
        t
    }
}

pub fn format_f32(val: f32, precision: usize) -> String {
    format!("{:.p$}", val, p = precision)
}

pub fn repeat_str(s: &str, count: usize) -> String {
    std::iter::repeat(s).take(count).collect::<String>()
}

/// Clears screen. Works across platforms
pub fn clear_scr() {
    let mut stdout = stdout();
    //just silently fail if something goes wrong
    //note execute flushes queue immediately
    let _ = execute!(stdout, terminal::Clear(terminal::ClearType::All));
}

pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

//https://stackoverflow.com/a/38406885/10232
pub fn uppercase_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

//this could use some more work and polish. Add "and" before the last item.
pub fn human_duration(seconds: f32) -> String {
    let s = seconds as i64;

    let dt = Utc.ymd(0, 1, 1).and_hms(0, 0, 0) + Duration::seconds(s);

    let y = build_time_str(dt.year(), "year");
    let mon = build_time_str(dt.month() as i32 - 1, "month");
    let d = build_time_str(dt.day() as i32 - 1, "day");
    let h = build_time_str(dt.hour() as i32, "hour");
    let min = build_time_str(dt.minute() as i32, "minute");
    let s = build_time_str(dt.second() as i32, "second");

    (&format!(
        "{y} {mon} {d} {h} {min} {s}",
        y = y,
        mon = mon,
        d = d,
        h = h,
        min = min,
        s = s
    ))
        .trim()
        .to_string()
}

pub fn build_time_str(t: i32, label: &str) -> String {
    let mut out: String = "".to_string();
    if t > 0 {
        out.push_str(&format!("{} {}", t, label));

        if t > 1 {
            out.push('s');
        }
    }

    out
}

pub fn build_tsv(name_values: Vec<(&str, String)>) -> String {
    name_values
        .iter()
        .map(|x| format!("{}{}{}{}", x.0, TSV_DELIM, x.1, TSV_EOL))
        .collect()
}

pub fn get_destiny2_launch_date() -> DateTime<Utc> {
    Utc.ymd(2017, 9, 6).and_hms(17, 0, 0)
}

pub fn get_last_weekly_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every tuesday)
    let past_reset: DateTime<Utc> = Utc.ymd(2020, 11, 10).and_hms(17, 0, 0);
    find_previous_moment(past_reset, WEEK_IN_SECONDS)
}

pub fn get_last_friday_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every friday)
    let past_reset: DateTime<Utc> = Utc.ymd(2020, 12, 4).and_hms(18, 0, 0);
    find_previous_moment(past_reset, WEEK_IN_SECONDS)
}

pub fn get_last_daily_reset() -> DateTime<Utc> {
    //get a hardcoded past daily date / time (17:00 UTC every tuesday)
    let past_reset: DateTime<Utc> = Utc.ymd(2020, 11, 10).and_hms(18, 0, 0);

    find_previous_moment(past_reset, DAY_IN_SECONDS)
}

fn find_previous_moment(past_reset: DateTime<Utc>, interval: i64) -> DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();

    //get total seconds between now and the past reset
    //take the mod of that divided by a week in seconds
    //subtract that amount from current date / time to find previous reset
    now - Duration::seconds((now - past_reset).num_seconds() % interval)
}
