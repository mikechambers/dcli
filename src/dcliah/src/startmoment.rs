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


use std::fmt;
use std::str::FromStr;

//TODO: sync these with dclitime
#[derive(PartialEq, Debug)]
pub enum StartMoment {
    Daily,
    Weekend,
    Weekly,
    Day,
    Week,
    Month, 
    AllTime,
    Custom,
}


impl FromStr for StartMoment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "daily" => Ok(StartMoment::Daily),
            "friday" => Ok(StartMoment::Weekend),
            "weekly" => Ok(StartMoment::Weekly),
            "day" => Ok(StartMoment::Day),
            "week" => Ok(StartMoment::Week),
            "month" => Ok(StartMoment::Month),
            "alltime" => Ok(StartMoment::AllTime),
            "custom" => Ok(StartMoment::Custom),

            _ => Err("Unknown DateStart type"),
        }
    }
}

impl fmt::Display for StartMoment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            StartMoment::Daily => "since the last daily reset",
            StartMoment::Weekend => "since the last Friday reset",
            StartMoment::Weekly => "since the last weekly reset",
            StartMoment::Day => "last day",
            StartMoment::Week => "last week",
            StartMoment::Month => "last month", 
            StartMoment::AllTime => "all time",
            StartMoment::Custom => "custom date / time",
        };

        write!(f, "{}", out)
    }
}
