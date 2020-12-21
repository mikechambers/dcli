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

use crate::{error::Error, response::pgcr::{DestinyPostGameCarnageReportData, DestinyHistoricalStatsValue}};

use crate::apiinterface::ApiInterface;
use crate::mode::Mode;
use crate::platform::Platform;
use futures::TryStreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Row;
use sqlx::{ConnectOptions, SqliteConnection};
use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashMap;

const ACTIVITY_STORE_SCHEMA: &str = include_str!("../actitvity_store_schema.sql");
const PCGR_REQUEST_CHUNK_AMOUNT: usize = 25;

pub struct ActivityStoreInterface {
    verbose: bool,
    db: SqliteConnection,
}

impl ActivityStoreInterface {
    pub async fn init_with_path(
        store_path: &PathBuf,
        verbose: bool,
    ) -> Result<ActivityStoreInterface, Error> {
        let path: String = format!("{}", store_path.display());
        let read_only = false;
        let connection_string: &str = &path;

        let mut db = SqliteConnectOptions::from_str(&connection_string)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(read_only)
            .connect()
            .await?;

        sqlx::query(ACTIVITY_STORE_SCHEMA).execute(&mut db).await?;

        Ok(ActivityStoreInterface {
            db: db,
            verbose: verbose,
        })
    }

    /// retrieves and stores activity details for ids in activity queue
    pub async fn sync(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        self.update_activity_queue(member_id, character_id, platform)
            .await?;
        self.sync_activities(member_id, character_id, platform)
            .await?;

        Ok(())
    }

    /// download results from ids in queue, and return number of items synced
    async fn sync_activities(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        let mut ids: Vec<String> = Vec::new();

        //This is to scope rows, so the mutable borrow of self goes out of scope
        {
            let character_rowid: i32 = match self
                .get_character_row_id(member_id, character_id, platform)
                .await
            {
                Ok(e) => e,
                Err(_e) => {
                    return Ok(());
                }
            };

            let mut rows =
                sqlx::query("SELECT activity_id from activity_queue where character = ?")
                    .bind(format!("{}", character_rowid))
                    .fetch(&mut self.db);

            while let Some(row) = rows.try_next().await? {
                let activity_id: String = row.try_get("activity_id")?;
                ids.push(activity_id);
            }
        };

        let mut count = 0;

        let api = ApiInterface::new(self.verbose)?;
        for id_chunks in ids.chunks(PCGR_REQUEST_CHUNK_AMOUNT) {
            let mut f = Vec::new();

            for c in id_chunks {
                f.push(api.retrieve_post_game_carnage_report(c));
            }

            count += f.len();
            println!("{} of {}", count, ids.len());

            //TODO: look into using threading for this
            let results = futures::future::join_all(f).await;

            //loop through. if we get results. grab those, otherwise, we ignore
            //any errors, as that will keep the IDs in the queue to try next time
            for r in results {
                match r {
                    Ok(e) => {
                        if e.is_some() {
                            match self
                                .insert_character_activity_stats(
                                    &e.unwrap(),
                                    member_id,
                                    character_id,
                                    platform,
                                )
                                .await
                            {
                                Ok(_e) => {}
                                Err(e) => {
                                    //TODO: debugging
                                    println!("Error inserting stats");
                                    println!("{}", e);
                                }
                            }

                            //extended.push(e.unwrap());
                        }
                        //TODO: what do we do if it returns None?
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }

        Ok(())
    }

    //updates activity id queue with ids which have not been synced
    async fn update_activity_queue(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        self.sync_activities(member_id, character_id, platform)
            .await?;

        let max_id: String = self
            .get_max_activity_id(member_id, character_id, platform)
            .await?;

        let api = ApiInterface::new(self.verbose)?;

        let result = api
            .retrieve_activities_since_id(member_id, character_id, platform, &Mode::AllPvP, &max_id)
            .await?;

        if result.is_none() {
            return Ok(());
        }

        let mut activities = result.unwrap();
        println!("{} new activities found", activities.len());

        let member_row_id = self.insert_member_id(&member_id, &platform).await?;

        let character_row_id = self
            .insert_character_id(&character_id, member_row_id)
            .await?;

        activities.reverse();

        // TODO: think through this
        // Right now, we do all inserts in one transation. This gives a significant performance
        // increse when inserting large number of activities at one time (i.e. on first sync).
        // however, it means if something goes wrong, nothing will be inserted, and if we
        // come across some data that causes a bug inserting, then nothing would ever be inserted
        // (until we fixed the bug). Probably shouldnt be an issue, since any weird stuff with
        // api data should be caught by the json deserializer in apiinterface
        sqlx::query("BEGIN TRANSACTION;")
            .execute(&mut self.db)
            .await?;
        for activity in activities {
            let instance_id = activity.details.instance_id;

            sqlx::query("INSERT into activity_queue ('activity_id', 'character') VALUES (?, ?)")
                .bind(instance_id)
                .bind(character_row_id)
                .execute(&mut self.db)
                .await?;
        }
        sqlx::query("COMMIT;").execute(&mut self.db).await?;

        Ok(())
    }

    async fn insert_character_activity_stats(
        &mut self,
        data: &DestinyPostGameCarnageReportData,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        sqlx::query("BEGIN TRANSACTION;")
            .execute(&mut self.db)
            .await?;

        match self
            ._insert_character_activity_stats(data, member_id, character_id, platform)
            .await
        {
            Ok(e) => {
                sqlx::query("COMMIT;").execute(&mut self.db).await?;
                Ok(e)
            }
            Err(e) => {
                sqlx::query("ROLLBACK;").execute(&mut self.db).await?;
                Err(e)
            }
        }
    }

    async fn _insert_character_activity_stats(
        &mut self,
        data: &DestinyPostGameCarnageReportData,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO 'main'.'activity'('activity_id','period','mode','platform','director_activity_hash') VALUES (?,?,?,?,?)")
        .bind(format!("{}", data.activity_details.instance_id)) //activity_id
        .bind(format!("{}", data.period)) //period
        .bind(format!("{}", data.activity_details.mode.to_id())) //mode
        .bind(format!("{}", data.activity_details.membership_type.to_id())) //platform
        .bind(format!("{}", data.activity_details.director_activity_hash)) //director_activity_hash
        .execute(&mut self.db)
        .await?;

        let character_rowid: i32 = self
            .get_character_row_id(member_id, character_id, platform)
            .await?;

        //TODO: need to handle not finding character
        let char_data = data.get_entry_for_character(&character_id).unwrap();

        let mut medal_hash:HashMap<String, DestinyHistoricalStatsValue> = char_data.extended.values;

        let precision_kills:f32 = match medal_hash.remove("precisionKills") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        let weapon_kills_ability:f32 = match medal_hash.remove("weaponKillsAbility") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        let weapon_kills_grenade:f32 = match medal_hash.remove("weaponKillsGrenade") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        let weapon_kills_melee:f32 = match medal_hash.remove("weaponKillsMelee") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        let weapon_kills_super:f32 = match medal_hash.remove("weaponKillsSuper") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        let all_medals_earned:f32 = match medal_hash.remove("allMedalsEarned") {
            Some(e) => e.basic.value,
            None => -1.0,
        };

        sqlx::query(
            r#"
            INSERT INTO 'main'.'character_activity_stats'
            (
                'character', 'assists', 'score', 'kills', 'deaths', 
                'average_score_per_kill', 'average_score_per_life', 'completed', 
                'opponents_defeated', 'activity_duration_seconds', 'standing', 
                'team', 'completion_reason', 'start_seconds', 'time_played_seconds', 
                'player_count', 'team_score', 'precision_kills', 'weapon_kills_ability', 
                'weapon_kills_grenade', 'weapon_kills_melee', 'weapon_kills_super', 
                'all_medals_earned', 'activity'
            )
            SELECT
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                id from activity where activity_id = ?
            "#,
        )
        .bind(format!("{}", character_rowid)) //character
        .bind(format!("{}", char_data.values.assists)) //assists
        .bind(format!("{}", char_data.values.score)) //score
        .bind(format!("{}", char_data.values.kills)) //kiis
        .bind(format!("{}", char_data.values.deaths)) //deaths
        .bind(format!("{}", char_data.values.average_score_per_kill)) //average_score_per_kill
        .bind(format!("{}", char_data.values.average_score_per_life)) //average_score_per_life
        .bind(format!("{}", char_data.values.completed)) //completed
        .bind(format!("{}", char_data.values.opponents_defeated)) //opponents_defeated
        .bind(format!("{}", char_data.values.activity_duration_seconds)) //activity_duration_seconds
        .bind(format!("{}", char_data.values.standing)) //standing
        .bind(format!("{}", char_data.values.team)) //team
        .bind(format!("{}", char_data.values.completion_reason)) //completion_reason
        .bind(format!("{}", char_data.values.start_seconds)) //start_seconds
        .bind(format!("{}", char_data.values.time_played_seconds)) //time_played_seconds
        .bind(format!("{}", char_data.values.player_count)) //player_count
        .bind(format!("{}", char_data.values.team_score)) //team_score
        .bind(format!("{}", precision_kills)) //precision_kills
        .bind(format!("{}", weapon_kills_ability)) //weapon_kills_ability
        .bind(format!("{}", weapon_kills_grenade)) //weapon_kills_grenade
        .bind(format!("{}", weapon_kills_melee)) //weapon_kills_melee
        .bind(format!("{}", weapon_kills_super)) //weapon_kills_super
        .bind(format!("{}", all_medals_earned)) //weapon_kills_super
        .bind(format!("{}", data.activity_details.instance_id)) //activity
        .execute(&mut self.db)
        .await?;

        if char_data.extended.weapons.is_some() {
            let weapons = char_data.extended.weapons.unwrap();
            for w in weapons {
                sqlx::query(
                    r#"
                    INSERT INTO "main"."weapon_result"
                    (
                        "reference_id", "unique_weapon_kills", "unique_weapon_precision_kills", "unique_weapon_kills_precision_kills", "character_activity_stats"
                    )
                    select ?, ?, ?, ?, id from character_activity_stats where rowid = 
                    (SELECT max(rowid) from character_activity_stats);
                    "#,
                )
                .bind(format!("{}", w.reference_id)) //reference_id
                .bind(format!("{}", w.values.unique_weapon_kills)) //unique_weapon_kills
                .bind(format!("{}", w.values.unique_weapon_precision_kills)) //unique_weapon_precision_kills
                .bind(format!("{}", w.values.unique_weapon_kills_precision_kills)) //unique_weapon_kills_precision_kills
                .execute(&mut self.db)
                .await?;
            }
        }

        sqlx::query(
            r#"
            DELETE FROM "main"."activity_queue" WHERE character = ? and activity_id = ?
        "#,
        )
        .bind(format!("{}", character_rowid))
        .bind(format!("{}", data.activity_details.instance_id))
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    async fn get_character_row_id(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<i32, Error> {
        let row = sqlx::query(
            r#"
                select character.id as id from character, member where character_id = ? and 
                character.member = member.id and member.member_id = ? and member.platform_id = ?
        "#,
        )
        .bind(format!("{}", character_id))
        .bind(format!("{}", member_id))
        .bind(format!("{}", platform.to_id()))
        .fetch_one(&mut self.db)
        .await?;

        let character_rowid: i32 = row.try_get("id")?;

        Ok(character_rowid)
    }

    async fn insert_member_id(
        &mut self,
        member_id: &str,
        platform: &Platform,
    ) -> Result<i32, Error> {
        sqlx::query("INSERT OR IGNORE into member ('member_id', 'platform_id') VALUES ($1, $2)")
            .bind(format!("{}", member_id))
            .bind(format!("{}", platform.to_id()))
            .execute(&mut self.db)
            .await?;

        let row = sqlx::query("SELECT id from member where member_id=? and platform_id=?")
            .bind(format!("{}", member_id))
            .bind(format!("{}", platform.to_id()))
            .fetch_one(&mut self.db)
            .await?;

        let rowid: i32 = row.try_get("id")?;

        Ok(rowid)
    }

    async fn insert_character_id(
        &mut self,
        character_id: &str,
        member_rowid: i32,
    ) -> Result<i32, Error> {
        sqlx::query("INSERT OR IGNORE into character ('character_id', 'member') VALUES ($1, $2)")
            .bind(format!("{}", character_id))
            .bind(member_rowid)
            .execute(&mut self.db)
            .await?;

        let row = sqlx::query("SELECT id from character where character_id=? and member=?")
            .bind(format!("{}", character_id))
            .bind(format!("{}", member_rowid))
            .fetch_one(&mut self.db)
            .await?;

        let rowid: i32 = row.try_get("id")?;

        Ok(rowid)
    }

    async fn get_max_activity_id(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<String, Error> {
        let row = sqlx::query(
            r#"
            SELECT
                MAX(CAST(activity.activity_id as INTEGER)) as max_activity_id
            FROM
                activity, character_activity_stats, character, member
            WHERE
                character_activity_stats.activity = activity.id AND 
                character.character_id = ? AND 
                character_activity_stats.character = character.id AND
                member.member_id = ? AND
                character.member = member.id AND
                member.platform_id = ?
            "#,
        )
        .bind(format!("{}", character_id))
        .bind(format!("{}", member_id))
        .bind(format!("{}", platform.to_id()))
        .fetch_one(&mut self.db)
        .await?;

        let activity_id: i64 = row.try_get("max_activity_id")?;
        Ok(activity_id.to_string())
    }
}
