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

use serde::Deserialize;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserializer;

//TODO:: might not be best place. maybe in a consts mod?
pub const RESOURCE_BASE_URL: &str = "https://www.bungie.net";

//2020-10-05T18:49:25Z
pub const API_DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn prepend_base_url<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    String::deserialize(deserializer).map(|a| {
        let mut s = String::from(RESOURCE_BASE_URL);
        s.push_str(&a);
        s
    })
}

pub fn prepend_base_url_option<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
    .map(|o: Option<String>| {
        match o {
            Some(e) => {
                let mut s = String::from(RESOURCE_BASE_URL);
                s.push_str(&e);
                Some(s)
            },
            None => None,
        }
    })
}


//str_to_datetime
pub fn str_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let n = match NaiveDateTime::parse_from_str(&s, API_DATE_TIME_FORMAT) {
        Ok(e) => e,
        Err(e) => {
            return Err(serde::de::Error::custom(&format!(
                "Could not parse date-time : {}",
                e
            )))
        }
    };

    let dt = DateTime::<Utc>::from_utc(n, Utc);

    Ok(dt)
}