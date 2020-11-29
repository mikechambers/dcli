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

use chrono::{DateTime, Utc, Duration, TimeZone, Datelike, Timelike};
use crate::error::Error;

//use chrono::prelude::*;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub const WEEK_IN_SECONDS: i64 = 604800;

pub const TSV_EOL:&str = "\n";
pub const TSV_DELIM:&str = "\t";

pub fn get_last_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every tuesday)
    let past_reset : DateTime<Utc> = Utc.ymd(2020, 11, 10).and_hms(17, 0, 0);
    let now: DateTime<Utc> = Utc::now();

    //get total seconds between now and the past reset
    //take the mod of that divided by a week in seconds
    //subtract that amount from current date / time to find previous reset
    now - Duration::seconds((now - past_reset).num_seconds() % WEEK_IN_SECONDS)
}

pub fn print_error(msg: &str, error:Error) {
    eprintln!("{}", msg);
    eprintln!("{}", error);
    eprintln!();
    eprintln!("If you think you have hit a bug, please log it at:");
    eprintln!("https://github.com/mikechambers/dcli/issues");
}

pub fn calculate_efficiency(kills:f32, deaths:f32, assists:f32) -> f32 {
    let t = kills + assists;
    if deaths > 0.0 { t / deaths } else { t }
}

pub fn calculate_kills_deaths_ratio(kills:f32, deaths:f32) -> f32 {
    if deaths > 0.0 { kills / deaths } else { kills } 
}

pub fn calculate_kills_deaths_assists(kills:f32, deaths:f32, assists:f32) -> f32 {
    let t = kills + (assists / 2.0);
    if deaths > 0.0 { t / deaths } else { t }
}


pub fn format_f32(val: f32, precision: usize) -> String {
    format!("{:.p$}", val, p = precision)
}

pub fn repeat_str(s: &str, count: usize) -> String {
    std::iter::repeat(s).take(count).collect::<String>()
}

pub fn clear_scr() {
    print!("{}[2J", 27 as char);
}

//this could use some more work and polish. Add "and" before the last item.
pub fn human_duration(seconds:f32) -> String {

    let s = seconds as i64;

    let dt = Utc.ymd(0, 1, 1).and_hms(0, 0, 0) + Duration::seconds(s);

    let y = build_time_str(dt.year(), "year");
    let mon = build_time_str(dt.month() as i32 - 1, "month");
    let d = build_time_str(dt.day() as i32 - 1, "day");
    let h = build_time_str(dt.hour() as i32, "hour");
    let min = build_time_str(dt.minute() as i32, "minute");
    let s = build_time_str(dt.second() as i32, "second");

    (&format!("{y} {mon} {d} {h} {min} {s}", y=y, mon=mon, d=d, h=h, min=min, s=s)).trim().to_string()
}

pub fn build_time_str(t:i32, label:&str) -> String {
    let mut out:String = "".to_string();
    if t > 0 {

        out.push_str(&format!("{} {}", t, label));

        if t > 1 {
            out.push_str("s");
        }
    }

    out
}

pub fn build_tsv(name_values:Vec<(&str, String)>) -> String {
    name_values.iter().map(|x| format!("{}{}{}{}", x.0, TSV_DELIM, x.1, TSV_EOL) ).collect()
}
