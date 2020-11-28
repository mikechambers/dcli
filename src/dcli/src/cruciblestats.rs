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

use std::ops;
use crate::utils::{calculate_efficiency, calculate_kills_deaths_assists, calculate_kills_deaths_ratio};

#[derive(Default, Debug)]
pub struct CrucibleStats {
    pub activities_entered:f32,
    pub activities_won:f32,
    pub assists:f32,
    pub kills:f32,
    pub average_kill_distance:f32,
    pub total_kill_distance:f32,
    pub seconds_played:f32,
    pub deaths:f32,
    pub average_lifespan:f32,
    pub best_single_game_kills:Option<f32>,
    pub opponents_defeated:f32,
    pub efficiency:f32,
    pub kills_deaths_ratio:f32,
    pub kills_deaths_assists:f32,
    pub suicides:f32,
}

impl ops::Add<CrucibleStats> for CrucibleStats {
    type Output = CrucibleStats;

    fn add(self, _cs: CrucibleStats) -> CrucibleStats {
        
        let best_single_game_kills:Option<f32>;

        if _cs.best_single_game_kills.is_none() && self.best_single_game_kills.is_none() {
            best_single_game_kills = None;
        } else if _cs.best_single_game_kills.is_none() || self.best_single_game_kills.is_none() {
            if _cs.best_single_game_kills.is_none() {
                best_single_game_kills = self.best_single_game_kills;
            } else {
                best_single_game_kills = _cs.best_single_game_kills;
            }
        } else {

            let a = _cs.best_single_game_kills.unwrap();
            let b = self.best_single_game_kills.unwrap();
            let c = if a > b {a} else {b};
            best_single_game_kills = Some(c);
        }

        let kills = self.kills + _cs.kills;
        let total_kill_distance = self.total_kill_distance + _cs.total_kill_distance;
        let assists = self.assists + _cs.assists;
        let deaths = self.deaths + _cs.deaths;

        //todo : add total activities?
        CrucibleStats {
            activities_entered : self.activities_entered + _cs.activities_entered,
            activities_won : self.activities_won + _cs.activities_won,
            assists : assists,
            kills : kills,
            average_kill_distance : total_kill_distance / kills,
            total_kill_distance : total_kill_distance,
            seconds_played : self.seconds_played + _cs.seconds_played,
            deaths : deaths,
            average_lifespan : self.average_lifespan + _cs.average_lifespan, //TODO: think about this in agg
            opponents_defeated : self.opponents_defeated + _cs.opponents_defeated,
            efficiency : calculate_efficiency(kills, deaths, assists),
            kills_deaths_ratio : calculate_kills_deaths_ratio(kills, deaths),
            kills_deaths_assists : calculate_kills_deaths_assists(kills, deaths, assists),
            suicides : self.suicides + _cs.suicides,
            best_single_game_kills: best_single_game_kills,
        }
    }
}