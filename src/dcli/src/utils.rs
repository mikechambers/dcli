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
use chrono::prelude::*;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub const WEEK_IN_SECONDS: i64 = 604800;

pub fn get_last_reset() -> DateTime<Utc> {
    //get a hardcoded past reset date / time (17:00 UTC every tuesday)
    let past_reset : DateTime<Utc> = Utc.ymd(2020, 11, 10).and_hms(17, 0, 0);
    let now: DateTime<Utc> = Utc::now();

    //get total seconds between now and the past reset
    //take the mod of that divided by a week in seconds
    //subtract that amount from current date / time to find previous reset
    now - Duration::seconds((now - past_reset).num_seconds() % WEEK_IN_SECONDS)
}

pub fn print_standard(out: &str, print: bool) {
    if !print {
        return;
    }

    println!("{}", out);
}

pub fn print_error(out: &str, print: bool) {
    if !print {
        return;
    }

    eprintln!("{}", out);
}
