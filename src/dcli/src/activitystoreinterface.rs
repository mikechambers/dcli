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

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};

use futures::TryStreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Row;
use sqlx::{ConnectOptions, SqliteConnection};

use crate::apiinterface::ApiInterface;
use crate::crucible::PlayerCruciblePerformances;
use crate::mode::Mode;
use crate::platform::Platform;
use crate::{
    error::Error,
    response::pgcr::{DestinyHistoricalStatsValue, DestinyPostGameCarnageReportData},
};

const STORE_FILE_NAME: &str = "dcli.sqlite3";
const ACTIVITY_STORE_SCHEMA: &str = include_str!("../actitvity_store_schema.sql");

//numer of simultaneous requests we make to server when retrieving activity history
const PGCR_REQUEST_CHUNK_AMOUNT: usize = 25;

pub struct ActivityStoreInterface {
    verbose: bool,
    db: SqliteConnection,
    path: String,
}

impl ActivityStoreInterface {
    pub fn get_storage_path(&self) -> String {
        self.path.clone()
    }

    pub async fn init_with_path(
        store_dir: &PathBuf,
        verbose: bool,
    ) -> Result<ActivityStoreInterface, Error> {
        let path = format!("{}", store_dir.join(STORE_FILE_NAME).display());

        let read_only = false;
        let connection_string: &str = &path;

        let mut db = SqliteConnectOptions::from_str(&connection_string)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(read_only)
            .connect()
            .await?;

        //is this an existing db, or a completly new one / first time?
        let rows = sqlx::query(
            r#"
            SELECT name FROM sqlite_master WHERE type='table' AND name='activity_queue'
        "#,
        )
        .fetch_all(&mut db)
        .await?;

        if rows.is_empty() {
            sqlx::query(ACTIVITY_STORE_SCHEMA).execute(&mut db).await?;
        }

        Ok(ActivityStoreInterface { db, verbose, path })
    }

    /// retrieves and stores activity details for ids in activity queue
    pub async fn sync(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        let member_row_id = self.insert_member_id(&member_id, &platform).await?;
        let character_row_id = self
            .insert_character_id(&character_id, member_row_id)
            .await?;

        //these calls could be a little more general purpose by taking api ids and not db ids.
        //however, passing the db ids, lets us optimize a lot of the sql, and avoid
        //some extra calls to the DB
        self.sync_activities(character_row_id, character_id).await?;
        self.update_activity_queue(character_row_id, member_id, character_id, platform)
            .await?;
        self.sync_activities(character_row_id, character_id).await?;

        Ok(())
    }

    /// download results from ids in queue, and return number of items synced
    async fn sync_activities(
        &mut self,
        character_row_id: i32,
        character_id: &str,
    ) -> Result<(), Error> {
        let mut ids: Vec<String> = Vec::new();

        //This is to scope rows, so the mutable borrow of self goes out of scope
        {
            let mut rows = sqlx::query(
                r#"
                    SELECT "activity_id" from "activity_queue" where character = ?
                "#,
            )
            .bind(format!("{}", character_row_id))
            .fetch(&mut self.db);

            while let Some(row) = rows.try_next().await? {
                let activity_id: String = row.try_get("activity_id")?;
                ids.push(activity_id);
            }
        };

        if ids.is_empty() {
            //eprintln!("No activity details need to be retrieved.");
            return Ok(());
        }

        //let mut count = 0;

        let api = ApiInterface::new(self.verbose)?;

        let s = if ids.len() == 1 { "y" } else { "ies" };
        eprintln!("Retrieving details for {} activit{}.", ids.len(), s);
        eprintln!("This may take a few minutes depending on the number of activities.");
        eprintln!(
            "Each dot represents {} activities",
            PGCR_REQUEST_CHUNK_AMOUNT
        );
        eprint!("[");
        for id_chunks in ids.chunks(PGCR_REQUEST_CHUNK_AMOUNT) {
            let mut f = Vec::new();

            for c in id_chunks {
                //this is saving the future, call hasnt been made yet
                f.push(api.retrieve_post_game_carnage_report(c));
            }

            //count += f.len();

            eprint!(".");
            /*
            eprintln!(
                "{} of {} ({}%)",
                count,
                ids.len(),
                ((count as f32 / ids.len() as f32) * 100.0).floor()
            );
            */

            //TODO: look into using threading for this
            let results = futures::future::join_all(f).await;

            //loop through. if we get results. grab those, otherwise, we ignore
            //any errors, as that will keep the IDs in the queue to try next time
            for r in results {
                match r {
                    Ok(e) => {
                        match e {
                            Some(e) => {
                                match self
                                    .insert_character_activity_stats(
                                        &e,
                                        character_row_id,
                                        character_id,
                                    )
                                    .await
                                {
                                    Ok(_e) => {}
                                    Err(e) => {
                                        eprintln!();
                                        eprintln!(
                                        "Error inserting data into character activity stats table : {}",
                                        e,
                                    );
                                    }
                                }
                            }
                            None => {
                                eprintln!();
                                eprintln!("PGCR returned empty response. Ignoring.");
                                //TODO: should not get here, as none means either an API error
                                //occured or there is no data associated with the ID (which is
                                //an api data error).
                                //we will just ignore it here, with the assumption that any error
                                //is temporary, and will be fixed next time we sync
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error retrieving activity details from api : {}", e);
                    }
                }
            }
        }

        eprintln!("]");

        Ok(())
    }

    //updates activity id queue with ids which have not been synced
    async fn update_activity_queue(
        &mut self,
        character_row_id: i32,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<(), Error> {
        let max_id: String = self.get_max_activity_id(character_row_id).await?;

        let api = ApiInterface::new(self.verbose)?;

        eprintln!("Checking for new activities.");
        eprintln!("This may take a moment depending on the number of activities.");
        let result = api
            .retrieve_activities_since_id(member_id, character_id, platform, &Mode::AllPvP, &max_id)
            .await?;

        if result.is_none() {
            return Ok(());
        }

        let mut activities = result.unwrap();
        eprintln!("{} new activities found.", activities.len());

        //reverse them so we add the oldest first
        activities.reverse();

        // TODO: think through this
        // Right now, we do all inserts in one transaction. This gives a significant performance
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
        character_row_id: i32,
        character_id: &str,
    ) -> Result<(), Error> {
        sqlx::query("BEGIN TRANSACTION;")
            .execute(&mut self.db)
            .await?;

        match self
            ._insert_character_activity_stats(data, character_row_id, character_id)
            .await
        {
            Ok(_e) => {
                sqlx::query("COMMIT;").execute(&mut self.db).await?;
                Ok(())
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
        character_row_id: i32,
        character_id: &str,
    ) -> Result<(), Error> {
        sqlx::query(r#"
            INSERT OR IGNORE INTO "main"."activity"("activity_id","period","mode","platform","director_activity_hash") VALUES (?,?,?,?,?)
        "#)
        .bind(data.activity_details.instance_id.clone()) //activity_id
        .bind(format!("{}", data.period)) //period
        .bind(format!("{}", data.activity_details.mode.to_id())) //mode
        .bind(format!("{}", data.activity_details.membership_type.to_id())) //platform
        .bind(format!("{}", data.activity_details.director_activity_hash)) //director_activity_hash
        .execute(&mut self.db)
        .await?;

        for mode in &data.activity_details.modes {
            sqlx::query(
                r#"
                INSERT INTO "main"."modes"
                (
                    "mode", "activity"
                )
                SELECT
                    ?,
                    id from activity where activity_id = ?
                "#,
            )
            .bind(mode.to_id().to_string())
            .bind(data.activity_details.instance_id.clone())
            .execute(&mut self.db)
            .await?;
        }

        let char_data = data
            .get_entry_for_character(&character_id)
            .ok_or(Error::CharacterDataNotFound)?;

        let mut medal_hash: HashMap<String, DestinyHistoricalStatsValue> =
            char_data.extended.values;

        let precision_kills: f32 = match medal_hash.remove("precisionKills") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        let weapon_kills_ability: f32 = match medal_hash.remove("weaponKillsAbility") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        let weapon_kills_grenade: f32 = match medal_hash.remove("weaponKillsGrenade") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        let weapon_kills_melee: f32 = match medal_hash.remove("weaponKillsMelee") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        let weapon_kills_super: f32 = match medal_hash.remove("weaponKillsSuper") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        let all_medals_earned: f32 = match medal_hash.remove("allMedalsEarned") {
            Some(e) => e.basic.value,
            None => 0.0,
        };

        sqlx::query(
            r#"
            INSERT INTO "main"."character_activity_stats"
            (
                "character", "assists", "score", "kills", "deaths", 
                "average_score_per_kill", "average_score_per_life", "completed", 
                "opponents_defeated", "activity_duration_seconds", "standing", 
                "team", "completion_reason", "start_seconds", "time_played_seconds", 
                "player_count", "team_score", "precision_kills", "weapon_kills_ability", 
                "weapon_kills_grenade", "weapon_kills_melee", "weapon_kills_super", 
                "all_medals_earned", "activity"
            )
            SELECT
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                id from activity where activity_id = ?
            "#,
        )
        .bind(format!("{}", character_row_id)) //character
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
        .bind(data.activity_details.instance_id.clone()) //activity
        .execute(&mut self.db)
        .await?;

        for (key, value) in medal_hash {
            sqlx::query(
                r#"
                INSERT INTO "main"."medal_result"
                (
                    "reference_id", "value", "character_activity_stats"
                )
                select ?, ?, id from character_activity_stats where rowid = 
                (SELECT max(rowid) from character_activity_stats);
                "#,
            )
            .bind(key) //reference_id
            .bind(value.basic.value) //unique_weapon_kills
            .execute(&mut self.db)
            .await?;
        }

        //ran into a case once where weapons was missing, so have to check here
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
        .bind(format!("{}", character_row_id))
        .bind(data.activity_details.instance_id.clone())
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
                select character.id as id from "character", "member" where character_id = ? and
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
        sqlx::query(
            r#"
            INSERT OR IGNORE into "member" ("member_id", "platform_id") VALUES (?, ?)
        "#,
        )
        .bind(member_id.to_string())
        .bind(format!("{}", platform.to_id()))
        .execute(&mut self.db)
        .await?;

        let row = sqlx::query(
            r#"
            SELECT id from "member" where member_id=? and platform_id=?
        "#,
        )
        .bind(member_id.to_string())
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
        sqlx::query(
            r#"
            INSERT OR IGNORE into "character" ("character_id", "member") VALUES (?, ?)
        "#,
        )
        .bind(character_id.to_string())
        .bind(member_rowid)
        .execute(&mut self.db)
        .await?;

        let row = sqlx::query(
            r#"
            SELECT id from "character" where character_id=? and member=?
        "#,
        )
        .bind(character_id.to_string())
        .bind(format!("{}", member_rowid))
        .fetch_one(&mut self.db)
        .await?;

        let rowid: i32 = row.try_get("id")?;

        Ok(rowid)
    }

    async fn get_max_activity_id(&mut self, character_row_id: i32) -> Result<String, Error> {
        let row = sqlx::query(
            r#"
            SELECT
                MAX(CAST(activity.activity_id as INTEGER)) as max_activity_id
            FROM
                "activity", "character_activity_stats", "character", "member"
            WHERE
                character_activity_stats.activity = activity.id AND 
                character_activity_stats.character = ? 
        "#,
        )
        .bind(format!("{}", character_row_id))
        .fetch_one(&mut self.db)
        .await?;

        let activity_id: i64 = row.try_get("max_activity_id")?;
        Ok(activity_id.to_string())
    }

    pub async fn retrieve_activities_since(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        start_time: &DateTime<Utc>,
    ) -> Result<Option<PlayerCruciblePerformances>, Error> {
        let character_index = self
            .get_character_row_id(member_id, character_id, platform)
            .await?;

        let query = sqlx::query(
            r#"
            select * from activity, character_activity_stats, modes where activity.period > ? and character_activity_stats.character = ? and character_activity_stats.activity = activity.id and modes.activity = activity.id and modes.mode = ?
        "#,
        )
        .bind(start_time.to_string())
        .bind(character_index.to_string())
        .bind(mode.to_id().to_string());

        let rows = query.fetch_all(&mut self.db).await?;

        println!(
            "retrieve_activities_since results returned : {}",
            rows.len()
        );

        Ok(None)
    }
}
