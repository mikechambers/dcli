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

use chrono::{DateTime, Utc, Duration, Datelike};
use chrono::prelude::*;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;


pub fn get_last_reset() -> DateTime<Utc> {

    let now: DateTime<Utc> = Utc::now();
    let n_date = Utc::now().date();
    let dt = Utc.ymd(n_date.year(), n_date.month(), n_date.day()).and_hms(18, 0, 0);

    let current_day = now.weekday().number_from_monday();
    let target_dt = if current_day == 2 {
        if dt > now {
            dt
        } else {
            dt - Duration::days(7)
        }
    }
    else if current_day > 2 {
        dt - Duration::days((current_day - 2) as i64)
    } else {
        dt - Duration::days(6)
    };

    target_dt
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
