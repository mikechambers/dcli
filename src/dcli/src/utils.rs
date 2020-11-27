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


/// Get the DateTime for the last reset time (18:00 UTC on the previous Tuesday)
pub fn get_last_reset() -> DateTime<Utc> {

    //There has to be a better way to figure this out

    let now: DateTime<Utc> = Utc::now();
    //use this to test specific dates / times
    //let now : DateTime<Utc> = Utc.ymd(2020, 11, 24).and_hms(18, 0, 1);

    let n_date = now.date();

    let dt = Utc.ymd(n_date.year(), n_date.month(), n_date.day()).and_hms(18, 0, 0);

    let w_day = n_date.weekday();

    //see if we are on reset day (Tue)
    let target_dt = if w_day == Weekday::Tue {

        if now > dt {
            //after reset, so use today's reset time
            dt
        } else {

            //before reset, go back to previous tuesday, reset
            dt - Duration::days(7)
        }
    } else {

        //figure out how many days we are away from the previous tuesday
        let c:i64 = ((w_day.num_days_from_sunday() + 4) % 7 + 1) as i64;
        dt - Duration::days(c)
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
