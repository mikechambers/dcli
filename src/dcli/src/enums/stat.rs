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

use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Stat {
    KD,
    Efficiency,
    KDA,
    Kills,
    OpponentsDefeated,
    Deaths,
    Assists,
    KillsAvg,
    OpponentsDefeatedAvg,
    DeathsAvg,
    AssistsAvg,

    KDMax,
    EfficiencyMax,
    KDAMax,
    KillsMax,
    OpponentsDefeatedMax,
    DeathsMax,
    AssistsMax,

    Games,
    Wins,
    Losses,
    Mercies,
}

impl FromStr for Stat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //wrap in String so we can convert to lower case
        let s = String::from(s).to_lowercase();

        //get a slice to get a &str for the match
        match &s[..] {
            "kd" => Ok(Stat::KD),
            "efficiency" => Ok(Stat::Efficiency),
            "kda" => Ok(Stat::KDA),
            "kills" => Ok(Stat::Kills),
            "opponents_defeated" => Ok(Stat::OpponentsDefeated),
            "deaths" => Ok(Stat::Deaths),
            "assists" => Ok(Stat::Assists),
            "kills_avg" => Ok(Stat::KillsAvg),
            "opponents_defeated_avg" => Ok(Stat::OpponentsDefeatedAvg),
            "deaths_avg" => Ok(Stat::DeathsAvg),
            "assists_avg" => Ok(Stat::AssistsAvg),

            "kd_max" => Ok(Stat::KDMax),
            "efficiency_max" => Ok(Stat::EfficiencyMax),
            "kda_max" => Ok(Stat::KDAMax),
            "kills_max" => Ok(Stat::KillsMax),
            "deaths_max" => Ok(Stat::DeathsMax),
            "assists_max" => Ok(Stat::AssistsMax),
            "opponents_defeated_max" => Ok(Stat::OpponentsDefeatedMax),

            "games" => Ok(Stat::Games),
            "wins" => Ok(Stat::Wins),
            "losses" => Ok(Stat::Losses),
            "mercies" => Ok(Stat::Mercies),

            _ => Err("Unknown Stat type"),
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Stat::KD => "Kills Deaths Ratio",
            Stat::Efficiency => "Efficiency",
            Stat::KDA => "Kills Deaths Assists Ratio",
            Stat::Kills => "Kills",
            Stat::OpponentsDefeated => "Opponents defeated",
            Stat::Deaths => "Deaths",
            Stat::Assists => "Assists",
            Stat::KillsAvg => "Kills per game",
            Stat::DeathsAvg => "Deaths per game",
            Stat::OpponentsDefeatedAvg => "Opponents defeated per game",
            Stat::AssistsAvg => "Assists per game",

            Stat::KDMax => "Highest game kills deaths ration",
            Stat::EfficiencyMax => "Highest game efficiency",
            Stat::KDAMax => "Highest game kills deaths assists ratio",
            Stat::KillsMax => "Highest kills in a game",
            Stat::DeathsMax => "Highest deaths in a game",
            Stat::AssistsMax => "Highest assists in a game",
            Stat::OpponentsDefeatedMax => {
                "Highest opponents deafeated in a game"
            }

            Stat::Games => "games",
            Stat::Wins => "wins",
            Stat::Losses => "losses",
            Stat::Mercies => "games ending in mercy",
        };

        write!(f, "{}", out)
    }
}
