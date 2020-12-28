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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
//use serde_derive::Deserialize;

use crate::apiutils::RESOURCE_BASE_URL;
use crate::enums::standing::STANDING_UNKNOWN_MAGIC_NUMBER;

use std::fmt::Display;
use std::str::FromStr;

//2020-10-05T18:49:25Z
pub const API_DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn property_to_i32_value<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Outer {
        pub basic: Inner,
    }

    #[derive(Deserialize)]
    struct Inner {
        pub value: f32,
    }

    let helper = <Outer>::deserialize(deserializer)?;
    Ok(helper.basic.value as i32)
}

pub fn property_to_u32_value<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Outer {
        pub basic: Inner,
    }

    #[derive(Deserialize)]
    struct Inner {
        pub value: f32,
    }

    let helper = <Outer>::deserialize(deserializer)?;
    Ok(helper.basic.value as u32)
}

pub fn property_to_value<'de, D, T: serde::de::Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Outer<T> {
        pub basic: Inner<T>,
    }

    #[derive(Deserialize)]
    struct Inner<T> {
        pub value: T,
    }

    let helper = <Outer<T>>::deserialize(deserializer)?;
    Ok(helper.basic.value)
}

/*
pub fn property_to_standing<'de, D>(deserializer: D) -> Result<Standing, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Outer {
        pub basic: Inner,
    }

    #[derive(Deserialize)]
    struct Inner {
        pub value: f32,
    }

    let helper = Outer::deserialize(deserializer)?;
    Ok(Standing::from_f32(helper.basic.value))
}
*/

//BUG: this doesnt get called if the property is not include in the JSON
//https://github.com/serde-rs/json/issues/734
pub fn property_to_option_float<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    struct Outer {
        pub basic: Inner,
    }

    #[derive(Deserialize, Debug)]
    struct Inner {
        pub value: f32,
    }

    Option::<Outer>::deserialize(deserializer).map(|o: Option<Outer>| match o {
        Some(e) => Some(e.basic.value),
        None => None,
    })
}

/*
Option::<String>::deserialize(deserializer).map(|o: Option<String>| match o {
    Some(e) => {
        let mut s = String::from(RESOURCE_BASE_URL);
        s.push_str(&e);
        Some(s)
    }
    None => None,
})
*/

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
    Option::<String>::deserialize(deserializer).map(|o: Option<String>| match o {
        Some(e) => {
            let mut s = String::from(RESOURCE_BASE_URL);
            s.push_str(&e);
            Some(s)
        }
        None => None,
    })
}

pub fn string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let out = match s.parse::<i64>() {
        Ok(e) => e,
        Err(e) => {
            return Err(serde::de::Error::custom(&format!(
                "Could not parse string to i64 : {}",
                e
            )))
        }
    };

    Ok(out)
}

//str_to_datetime
pub fn str_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
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

pub fn standing_default() -> u32 {
    STANDING_UNKNOWN_MAGIC_NUMBER
}

pub fn str_to_int<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}
