/*
* Copyright 2023 Mike Chambers
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

#[derive(sqlx::FromRow, Debug)]
pub struct PlayerActivitiesSummary {
    pub total_activities: u32,
    pub time_played_seconds: u32,
    pub wins: u32,
    pub completion_reason_mercy: u32,
    pub completed: u32,
    pub assists: u32,
    pub kills: u32,
    pub deaths: u32,
    pub opponents_defeated: u32,

    pub grenade_kills: u32,
    pub melee_kills: u32,
    pub super_kills: u32,
    pub ability_kills: u32,
    pub precision: u32,
    pub highest_assists: u32,
    pub highest_kills: u32,
    pub highest_deaths: u32,
    pub highest_opponents_defeated: u32,
    pub highest_grenade_kills: u32,
    pub highest_melee_kills: u32,
    pub highest_super_kills: u32,
    pub highest_ability_kills: u32,

    pub highest_kills_deaths_assists_ratio: f32,
    pub highest_kills_deaths_ratio: f32,
    pub highest_efficiency: f32,
}
