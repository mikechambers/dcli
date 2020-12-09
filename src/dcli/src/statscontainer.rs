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

use crate::utils::{
    calculate_efficiency,
    calculate_kills_deaths_ratio,
    calculate_kills_deaths_assists,
    calculate_per_activity_average
};

use crate::response::activities::Activity;
use crate::standing::Standing;

#[derive(Default)]
pub struct ActivityStatsContainer {
    pub activities:Vec<Activity>,

    assists:f32,
    score:f32,
    kills:f32,
    deaths:f32,
    opponents_defeated:f32,
    efficiency:f32,
    kills_deaths_ratio:f32,
    kills_deaths_assists:f32,
    wins:f32,
    losses:f32,
    draws:f32,
    time_played_seconds:f32,

}

impl ActivityStatsContainer {
    pub fn with_activities(activities:Vec<Activity>) -> ActivityStatsContainer {
        let mut a = ActivityStatsContainer {
            activities,
            assists:0.0,
            score:0.0,
            kills:0.0,
            deaths:0.0,
            opponents_defeated:0.0,
            efficiency:0.0,
            kills_deaths_ratio:0.0,
            kills_deaths_assists:0.0,
            wins:0.0,
            losses:0.0,
            draws:0.0,
            time_played_seconds:0.0,
        };

        a.update();
        a
    }

    fn per_activity_average(&self, value:f32) -> f32 {
        calculate_per_activity_average(value, self.activities.len() as f32)
    }

    fn update(&mut self) {

        for a in self.activities.iter() {
            self.assists += a.values.assists;
            self.score += a.values.score;
            self.kills += a.values.kills;
            self.deaths += a.values.deaths;
            self.opponents_defeated += a.values.opponents_defeated;
            self.time_played_seconds += a.values.time_played_seconds;
            
            match a.values.standing {
                Standing::Victory => {
                    self.wins += 1.0;
                },
                Standing::Defeat => {
                    self.losses += 1.0;
                },
                Standing::Unknown => {
                    self.draws += 1.0;
                },
            }

        }

        self.kills_deaths_assists = calculate_kills_deaths_assists(self.kills, self.deaths, self.assists);
        self.kills_deaths_ratio = calculate_kills_deaths_ratio(self.kills, self.deaths);
        self.efficiency = calculate_efficiency(self.kills, self.deaths, self.assists);
    }

    fn assists(&self) -> f32 {
        self.assists
    }

    pub fn kills(&self) -> f32 {
        self.kills
    }

    pub fn deaths(&self) -> f32 {
        self.deaths
    }

    pub fn opponents_defeated(&self) -> f32 {
        self.opponents_defeated
    }

    pub fn efficiency(&self) -> f32 {
        self.efficiency
    }

    pub fn kills_deaths_ratio(&self) -> f32 {
        self.kills_deaths_ratio
    }

    pub fn kills_deaths_assists(&self) -> f32 {
        self.kills_deaths_assists
    }

    pub fn wins(&self) -> f32 {
        self.wins
    }

    pub fn losses(&self) -> f32 {
        self.losses
    }

    pub fn draws(&self) -> f32 {
        self.draws
    }

    pub fn total_activities(&self) -> f32 {
        self.activities.len() as f32
    }
}

//create a trait for the stats type that has getters for all shared stats