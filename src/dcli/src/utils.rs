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

use std::env;
use std::ffi::OsStr;
use std::io::stdout;
use std::path::Path;
use std::path::PathBuf;

use crate::enums::mode::Mode;
use chrono::{DateTime, Datelike, Duration, Local, TimeZone, Timelike, Utc};
use crossterm::{execute, terminal};
use std::str::FromStr;

use crate::error::Error;

//use chrono::prelude::*;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub const WEEK_IN_SECONDS: i64 = 604800;
pub const DAY_IN_SECONDS: i64 = 86400;

pub const TSV_EOL: &str = "\n";
pub const TSV_DELIM: &str = "\t";

pub const COMPETITIVE_PVP_ACTIVITY_HASH: u32 = 2754695317;
pub const FREELANCE_COMPETITIVE_PVP_ACTIVITY_HASH: u32 = 2607135461;

//pub const CHECKMATE_CONTROL_ACTIVITY_HASH: u32 = 3374318171;
//480175362let my_array: [i32; 2] = [1, 2];
pub const CHECKMATE_CONTROL_ACTIVITY_HASHES: [u32; 2] = [480175362, 2461220411];

pub const CHECKMATE_CLASH_ACTIVITY_HASH: u32 = 1251966208;
pub const CHECKMATE_COUNTDOWN_ACTIVITY_HASH: u32 = 2344293485;
pub const CHECKMATE_RUMBLE_ACTIVITY_HASH: u32 = 2461220411;

pub const IRON_BANNER_TRIBUTE_ACTIVITY_HASH: u32 = 2955009825;
pub const IRON_BANNER_FORTRESS_ACTIVITY_HASH: u32 = 2888503916;

pub const CHECKMATE_SURVIVAL_ACTIVITY_HASH: u32 = 3876264582;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn f32_are_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < f32::EPSILON
}

pub trait Period {
    fn get_start(&self) -> DateTime<Utc>;
    fn get_end(&self) -> DateTime<Utc>;
}

pub fn format_error(msg: &str, error: Error) -> String {
    let mut strings: Vec<String> = vec![];

    let app_name = env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .unwrap_or_else(|| "".to_string());

    strings.push(format!("{} : v{}", app_name, VERSION));
    strings.push(msg.to_string());
    strings.push(format!("{}", error));

    match error {
        Error::InvalidParameters => {
            strings.push(
                "This can occur if --platform is set incorrectly.".to_string(),
            );
        }
        Error::ParameterParseFailure => {
            strings.push("This can occur if --member-id or --character-id were entered incorrectly.".to_string());
        }
        _ => {}
    }

    strings.push("\nIf you think you have hit a bug and would like to report it (or would just like some help):".to_string());
    strings.push("    1. Run command with '--verbose' flag.".to_string());
    strings.push("    2a. Copy output, and share on Discord: ".to_string());
    strings.push("       https://discord.gg/2Y8bV2Mq3p".to_string());
    strings.push("    2b. Copy output, and log a bug at: ".to_string());
    strings
        .push("       https://github.com/mikechambers/dcli/issues".to_string());

    strings.join("\n")
}

pub fn calculate_average(value: u32, total_activities: u32) -> f32 {
    if total_activities == 0 {
        return 0.0;
    }

    value as f32 / total_activities as f32
}

pub fn calculate_efficiency(kills: u32, deaths: u32, assists: u32) -> f32 {
    let t = (kills + assists) as f32;
    if deaths > 0 {
        t / deaths as f32
    } else {
        t
    }
}

pub fn calculate_kills_deaths_ratio(kills: u32, deaths: u32) -> f32 {
    let kills = kills as f32;
    if deaths > 0 {
        kills / deaths as f32
    } else {
        kills
    }
}

pub fn calculate_kills_deaths_assists(
    kills: u32,
    deaths: u32,
    assists: u32,
) -> f32 {
    let kills = kills as f32;
    let assists = assists as f32;

    let t = kills + (assists / 2.0);
    if deaths > 0 {
        t / deaths as f32
    } else {
        t
    }
}

pub fn format_f32(val: f32, precision: usize) -> String {
    format!("{:.p$}", val, p = precision)
}

pub fn repeat_str(s: &str, count: usize) -> String {
    s.repeat(count)
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

pub fn human_date_format(start_time: &DateTime<Utc>) -> String {
    let local = start_time.with_timezone(&Local);
    let format_str = if Utc::now() - *start_time > Duration::days(6) {
        "%B %-d, %Y"
    } else if local.day() == Local::now().day() {
        "Today at %-I:%M %p"
    } else {
        "%A at %-I:%M %p"
    };

    format!("{}", local.format(format_str))
}

//this could use some more work and polish. Add "and" before the last item.
pub fn human_duration(seconds: u32) -> String {
    //let dt = Utc.with_ymd_and_hms(0, 1, 1,0, 0, 0) + Duration::seconds(seconds as i64);
    let dt = Utc.with_ymd_and_hms(0, 1, 1, 0, 0, 0).unwrap()
        + Duration::seconds(seconds as i64);
    let year = build_time_str(dt.year(), "year");
    let mon = build_time_str(dt.month() as i32 - 1, "month");
    let day = build_time_str(dt.day() as i32 - 1, "day");
    let hour = build_time_str(dt.hour() as i32, "hour");
    let min = build_time_str(dt.minute() as i32, "minute");
    let sec = build_time_str(dt.second() as i32, "second");
    //collect all items into a vector
    let t = vec![year, mon, day, hour, min, sec];

    //remove empty items
    let mut t = t
        .into_iter()
        .filter(|i| i.trim().chars().count() > 0)
        .collect::<Vec<String>>();

    //add an add before the last item
    if t.len() > 1 {
        t.insert(t.len() - 1, "and".to_string());
    }

    t.join(" ")
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
    Utc.with_ymd_and_hms(2017, 9, 6, 17, 0, 0).unwrap()
}

pub fn get_last_weekly_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every tuesday)
    let past_reset: DateTime<Utc> =
        Utc.with_ymd_and_hms(2020, 11, 10, 17, 0, 0).unwrap();
    find_previous_moment(past_reset, WEEK_IN_SECONDS)
}

pub fn get_last_friday_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every friday)
    let past_reset: DateTime<Utc> =
        Utc.with_ymd_and_hms(2020, 12, 4, 17, 0, 0).unwrap();
    find_previous_moment(past_reset, WEEK_IN_SECONDS)
}

pub fn get_last_daily_reset() -> DateTime<Utc> {
    //get a hardcoded past daily date / time (17:00 UTC every tuesday)
    let past_reset: DateTime<Utc> =
        Utc.with_ymd_and_hms(2020, 11, 10, 17, 0, 0).unwrap();

    find_previous_moment(past_reset, DAY_IN_SECONDS)
}

fn find_previous_moment(
    past_reset: DateTime<Utc>,
    interval: i64,
) -> DateTime<Utc> {
    let now: DateTime<Utc> = Utc::now();

    //get total seconds between now and the past reset
    //take the mod of that divided by a week in seconds
    //subtract that amount from current date / time to find previous reset
    now - Duration::seconds((now - past_reset).num_seconds() % interval)
}

pub fn determine_data_dir(dir: Option<PathBuf>) -> Result<PathBuf, Error> {
    let path = match dir {
        Some(e) => e,
        None => {
            let dld = dirs_next::data_local_dir()
                .ok_or(Error::SystemDirectoryNotFound)?;
            dld.join("dcli")
        }
    };

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn calculate_ratio(a: u32, b: u32) -> f32 {
    if b == 0 {
        return 0.0;
    }

    a as f32 / b as f32
}

pub fn calculate_avg(total: f32, count: u32) -> f32 {
    if count == 0 {
        return 0.0;
    }

    total / count as f32
}

pub fn calculate_percent(value: u32, total: u32) -> f32 {
    if total == 0 {
        return 0.0;
    }

    (value as f32 / total as f32) * 100.0
}

pub fn truncate_ascii_string(input: &str, max_len: usize) -> String {
    if input.chars().count() <= max_len {
        return input.to_string();
    }

    format!("{:.len$}...", input, len = max_len - 3)
}

pub fn parse_rfc3339(src: &str) -> Result<DateTime<Utc>, String> {
    let d =
        match DateTime::parse_from_rfc3339(src) {
            Ok(e) => e,
            Err(_e) => return Err(
                "Invalid RFC 3339 Date / Time String : Example : 2020-12-08T17:00:00.774187+00:00"
                    .to_string(),
            ),
        };

    let d = d.with_timezone(&Utc);

    if d > Utc::now() {
        return Err("start-date must be in the past.".to_string());
    }

    Ok(d)
}

pub fn parse_and_validate_crucible_mode(src: &str) -> Result<Mode, String> {
    let mode = Mode::from_str(src)?;

    if !mode.is_crucible() {
        return Err(format!("Unsupported mode specified : {}", src));
    }

    Ok(mode)
}
