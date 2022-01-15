/*
* Copyright 2022 Mike Chambers
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

//use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum WeaponSort {
    Name,
    Kills,
    Games,
    KillsPerGameKills,
    PrecisionTotal,
    PrecisionPercent,
    WinPercent,
    Type,
}

impl FromStr for WeaponSort {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "name" => Ok(WeaponSort::Name),
            "kills" => Ok(WeaponSort::Kills),
            "games" => Ok(WeaponSort::Games),
            "kills_per_game_kills" => Ok(WeaponSort::KillsPerGameKills),
            "precision_total" => Ok(WeaponSort::PrecisionTotal),
            "precision_percent" => Ok(WeaponSort::PrecisionPercent),
            "wins_percent" => Ok(WeaponSort::WinPercent),
            "type" => Ok(WeaponSort::Type),

            _ => Err("Unknown WeaponSort type"),
        }
    }
}
