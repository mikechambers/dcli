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

use crate::enums::standing::Standing;
use crate::response::activities::Activity;
use crate::utils::{
    calculate_efficiency, calculate_kills_deaths_assists, calculate_kills_deaths_ratio,
    calculate_per_activity_average,
};

#[derive(Default)]
pub struct ActivityStatsContainer {
    pub activities: Vec<Activity>,

    assists: f32,
    score: f32,
    kills: f32,
    deaths: f32,
    opponents_defeated: f32,
    efficiency: f32,
    kills_deaths_ratio: f32,
    kills_deaths_assists: f32,
    wins: f32,
    losses: f32,
    draws: f32,
    time_played_seconds: f32,

    highest_kills: f32,
    highest_assists: f32,
    highest_deaths: f32,
    highest_opponents_defeated: f32,
    highest_efficiency: f32,
    highest_kills_deaths_ratio: f32,
    highest_kills_deaths_assists: f32,

    longest_win_streak: f32,
    longest_loss_streak: f32,
}

impl ActivityStatsContainer {
    pub fn with_activities(activities: Vec<Activity>) -> ActivityStatsContainer {
        let mut a = ActivityStatsContainer {
            activities,
            assists: 0.0,
            score: 0.0,
            kills: 0.0,
            deaths: 0.0,
            opponents_defeated: 0.0,
            efficiency: 0.0,
            kills_deaths_ratio: 0.0,
            kills_deaths_assists: 0.0,
            wins: 0.0,
            losses: 0.0,
            draws: 0.0,
            time_played_seconds: 0.0,

            highest_kills: 0.0,
            highest_assists: 0.0,
            highest_deaths: 0.0,
            highest_opponents_defeated: 0.0,
            highest_efficiency: 0.0,
            highest_kills_deaths_ratio: 0.0,
            highest_kills_deaths_assists: 0.0,

            longest_win_streak: 0.0,
            longest_loss_streak: 0.0,
        };

        a.update();
        a
    }

    pub fn per_activity_average(&self, value: f32) -> f32 {
        calculate_per_activity_average(value as u32, self.activities.len() as u32)
    }

    fn update(&mut self) {
        let mut last_standing = Standing::Unknown;
        let mut streak = 0.0;
        for a in self.activities.iter() {
            self.assists += a.values.assists;
            self.score += a.values.score;
            self.kills += a.values.kills;

            //self.highest_kills = max(self.highest_kills as u32, a.values.kills as u32);
            self.highest_kills = self.highest_kills.max(a.values.kills);
            self.highest_assists = self.highest_assists.max(a.values.assists);
            self.highest_deaths = self.highest_deaths.max(a.values.deaths);
            self.highest_opponents_defeated = self
                .highest_opponents_defeated
                .max(a.values.opponents_defeated);

            self.highest_efficiency = self.highest_efficiency.max(a.values.efficiency);
            self.highest_kills_deaths_ratio = self
                .highest_kills_deaths_ratio
                .max(a.values.kills_deaths_ratio);
            self.highest_kills_deaths_assists = self
                .highest_kills_deaths_assists
                .max(a.values.kills_deaths_assists);

            self.deaths += a.values.deaths;
            self.opponents_defeated += a.values.opponents_defeated;
            self.time_played_seconds += a.values.time_played_seconds;

            let standing = Standing::from_mode(a.values.standing, &a.details.mode);
            match standing {
                Standing::Victory => {
                    self.wins += 1.0;
                }
                Standing::Defeat => {
                    self.losses += 1.0;
                }
                Standing::Unknown => {
                    self.draws += 1.0;
                }
            };

            if standing == last_standing {
                streak = match last_standing {
                    Standing::Unknown => 0.0,
                    Standing::Victory => streak + 1.0,
                    Standing::Defeat => streak - 1.0,
                };
            } else {
                last_standing = standing;
                streak = match last_standing {
                    Standing::Unknown => 0.0,
                    Standing::Victory => 1.0,
                    Standing::Defeat => -1.0,
                };
            }

            self.longest_loss_streak = self.longest_loss_streak.min(streak);
            self.longest_win_streak = self.longest_win_streak.max(streak);
        }

        self.kills_deaths_assists = calculate_kills_deaths_assists(
            self.kills as u32,
            self.deaths as u32,
            self.assists as u32,
        );
        self.kills_deaths_ratio =
            calculate_kills_deaths_ratio(self.kills as u32, self.deaths as u32);
        self.efficiency =
            calculate_efficiency(self.kills as u32, self.deaths as u32, self.assists as u32);
    }

    pub fn longest_win_streak(&self) -> f32 {
        self.longest_win_streak
    }

    pub fn longest_loss_streak(&self) -> f32 {
        self.longest_loss_streak.abs()
    }

    pub fn win_percentage(&self) -> f32 {
        let total = self.total_activities();

        if total == 0.0 {
            return 0.0;
        }

        self.wins / total * 100.0
    }

    pub fn highest_efficiency(&self) -> f32 {
        self.highest_efficiency
    }

    pub fn highest_kills_deaths_assists(&self) -> f32 {
        self.highest_kills_deaths_assists
    }

    pub fn highest_kills_deaths_ratio(&self) -> f32 {
        self.highest_kills_deaths_ratio
    }

    pub fn highest_kills(&self) -> f32 {
        self.highest_kills
    }

    pub fn highest_deaths(&self) -> f32 {
        self.highest_deaths
    }

    pub fn highest_assists(&self) -> f32 {
        self.highest_assists
    }

    pub fn highest_opponents_defeated(&self) -> f32 {
        self.highest_opponents_defeated
    }

    pub fn assists(&self) -> f32 {
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
