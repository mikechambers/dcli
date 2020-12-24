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

use crate::crucible::{
    ActivityDetail, CruciblePlayerPerformance, CrucibleStats, ExtendedCrucibleStats, Item, Medal,
    MedalStat, Player, PlayerCruciblePerformances, WeaponStat,
};
use crate::enums::medaltier::MedalTier;
use crate::enums::mode::Mode;
use crate::enums::platform::Platform;
use crate::{apiinterface::ApiInterface, manifestinterface::ManifestInterface};
use crate::{
    error::Error,
    response::pgcr::{DestinyHistoricalStatsValue, DestinyPostGameCarnageReportData},
    utils::{calculate_efficiency, calculate_kills_deaths_assists, calculate_kills_deaths_ratio},
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
        let mut ids: Vec<i64> = Vec::new();

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
                let activity_id: i64 = row.try_get("activity_id")?;
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
                f.push(api.retrieve_post_game_carnage_report(*c));
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
                                    Ok(_e) => (),
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
                        eprintln!();
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
        let max_id: i64 = self.get_max_activity_id(character_row_id).await?;

        let api = ApiInterface::new(self.verbose)?;

        eprintln!("Checking for new activities.");
        eprintln!("This may take a moment depending on the number of activities.");
        let result = api
            .retrieve_activities_since_id(member_id, character_id, platform, &Mode::AllPvP, max_id)
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

    fn get_medal_hash_value(
        &self,
        property: &str,
        medal_hash: &mut HashMap<String, DestinyHistoricalStatsValue>,
    ) -> u32 {
        match medal_hash.remove(property) {
            Some(e) => e.basic.value as u32,
            None => 0,
        }
    }

    async fn _insert_character_activity_stats(
        &mut self,
        data: &DestinyPostGameCarnageReportData,
        character_row_id: i32,
        character_id: &str,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO "main"."activity"
                ("activity_id","period","mode","platform","director_activity_hash", "reference_id") 
            VALUES (?,?,?,?,?, ?)
        "#,
        )
        .bind(data.activity_details.instance_id) //activity_id
        .bind(format!("{}", data.period.to_rfc3339())) //period
        .bind(format!("{}", data.activity_details.mode.to_id())) //mode
        .bind(format!("{}", data.activity_details.membership_type.to_id())) //platform
        .bind(format!("{}", data.activity_details.director_activity_hash)) //director_activity_hash
        .bind(format!("{}", data.activity_details.reference_id)) //reference_id
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
            .bind(data.activity_details.instance_id)
            .execute(&mut self.db)
            .await?;
        }

        let char_data = data
            .get_entry_for_character(&character_id)
            .ok_or(Error::CharacterDataNotFound)?;

        let mut medal_hash: HashMap<String, DestinyHistoricalStatsValue> =
            char_data.extended.values;

        let precision_kills: u32 = self.get_medal_hash_value("precisionKills", &mut medal_hash);
        let weapon_kills_ability: u32 =
            self.get_medal_hash_value("weaponKillsAbility", &mut medal_hash);
        let weapon_kills_grenade: u32 =
            self.get_medal_hash_value("weaponKillsGrenade", &mut medal_hash);
        let weapon_kills_melee: u32 =
            self.get_medal_hash_value("weaponKillsMelee", &mut medal_hash);
        let weapon_kills_super: u32 =
            self.get_medal_hash_value("weaponKillsSuper", &mut medal_hash);
        let all_medals_earned: u32 = self.get_medal_hash_value("allMedalsEarned", &mut medal_hash);

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
        //we for through format, as otherwise we have to cast to i32, and while
        //shouldnt be an issue, there is a chance we could lose precision when
        //converting some of the IDS. so we just do this to be consistent.
        //TODO: should think about losing data when pulling out of DB
        .bind(format!("{}", character_row_id as u32)) //character
        .bind(format!("{}", char_data.values.assists as u32)) //assists
        .bind(format!("{}", char_data.values.score as u32)) //score
        .bind(format!("{}", char_data.values.kills as u32)) //kiis
        .bind(format!("{}", char_data.values.deaths as u32)) //deaths
        .bind(format!("{}", char_data.values.average_score_per_kill)) //average_score_per_kill
        .bind(format!("{}", char_data.values.average_score_per_life)) //average_score_per_life
        .bind(format!("{}", char_data.values.completed as u32)) //completed
        .bind(format!("{}", char_data.values.opponents_defeated as u32)) //opponents_defeated
        .bind(format!(
            "{}",
            char_data.values.activity_duration_seconds as u32
        )) //activity_duration_seconds
        .bind(format!("{}", char_data.values.standing as u32)) //standing
        .bind(format!("{}", char_data.values.team as u32)) //team
        .bind(format!("{}", char_data.values.completion_reason as u32)) //completion_reason
        .bind(format!("{}", char_data.values.start_seconds as u32)) //start_seconds
        .bind(format!("{}", char_data.values.time_played_seconds as u32)) //time_played_seconds
        .bind(format!("{}", char_data.values.player_count as u32)) //player_count
        .bind(format!("{}", char_data.values.team_score as u32)) //team_score
        .bind(format!("{}", precision_kills)) //precision_kills
        .bind(format!("{}", weapon_kills_ability)) //weapon_kills_ability
        .bind(format!("{}", weapon_kills_grenade)) //weapon_kills_grenade
        .bind(format!("{}", weapon_kills_melee)) //weapon_kills_melee
        .bind(format!("{}", weapon_kills_super)) //weapon_kills_super
        .bind(format!("{}", all_medals_earned)) //weapon_kills_super
        .bind(data.activity_details.instance_id) //activity
        .execute(&mut self.db)
        .await?;

        for (key, value) in medal_hash {
            sqlx::query(
                r#"
                INSERT INTO "main"."medal_result"
                (
                    "reference_id", "count", "character_activity_stats"
                )
                select ?, ?, id from character_activity_stats where rowid = 
                (SELECT max(rowid) from character_activity_stats);
                "#,
            )
            .bind(key) //reference_id
            .bind(format!("{}", value.basic.value as u32)) //unique_weapon_kills
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
                        "reference_id", "kills", "precision_kills", "kills_precision_kills_ratio", "character_activity_stats"
                    )
                    select ?, ?, ?, ?, id from character_activity_stats where rowid = 
                    (SELECT max(rowid) from character_activity_stats);
                    "#,
                )
                .bind(format!("{}", w.reference_id)) //reference_id
                .bind(format!("{}", w.values.unique_weapon_kills as u32)) //unique_weapon_kills
                .bind(format!("{}", w.values.unique_weapon_precision_kills as u32)) //unique_weapon_precision_kills
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
        .bind(data.activity_details.instance_id)
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

    async fn get_max_activity_id(&mut self, character_row_id: i32) -> Result<i64, Error> {
        let row = sqlx::query(
            r#"
            SELECT
                MAX(activity.activity_id) as max_activity_id
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
        Ok(activity_id)
    }

    pub async fn retrieve_activities_since(
        &mut self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        start_time: &DateTime<Utc>,
        manifest: &mut ManifestInterface,
    ) -> Result<Option<PlayerCruciblePerformances>, Error> {
        let character_index = self
            .get_character_row_id(member_id, character_id, platform)
            .await?;

        let activity_rows = sqlx::query(
            r#"
            SELECT
                *,
                activity.id as activity_index, 
                activity.mode as activity_mode,
                character_activity_stats.id as character_activity_stats_index
            FROM 
                activity, character_activity_stats, modes 
            WHERE 
                activity.period > ? and 
                character_activity_stats.character = ? and 
                character_activity_stats.activity = activity.id and 
                modes.activity = activity.id and 
                modes.mode = ?
        "#,
        )
        .bind(start_time.to_string())
        .bind(character_index.to_string())
        .bind(mode.to_id().to_string())
        .fetch_all(&mut self.db)
        .await?;

        //get_activity_definition

        let mut performances: Vec<CruciblePlayerPerformance> = Vec::new();
        for activity_row in &activity_rows {
            //let activity_index: i32 = activity_row.try_get("activity_index")?;
            let activity_id: i64 = activity_row.try_get("activity_id")?;

            let mode_id: i32 = activity_row.try_get("activity_mode")?;
            let platform_id: i32 = activity_row.try_get("platform")?;

            let period: String = activity_row.try_get("period")?;
            let period = DateTime::parse_from_rfc3339(&period)?;
            let period = period.with_timezone(&Utc);

            let director_activity_hash: i64 = activity_row.try_get("director_activity_hash")?;
            let director_activity_hash: u32 = director_activity_hash as u32;

            let reference_id: i64 = activity_row.try_get("reference_id")?;
            let reference_id: u32 = reference_id as u32;

            let activity_definition = manifest.get_activity_definition(reference_id).await?;

            let activity_detail = ActivityDetail {
                id: activity_id,
                period,
                map_name: activity_definition.display_properties.name,
                mode: Mode::from_id(mode_id as u32)?,
                platform: Platform::from_id(platform_id as u32),
                director_activity_hash,
                reference_id,
            };

            let assists: i32 = activity_row.try_get("assists")?;
            let assists: u32 = assists as u32;

            let score: i32 = activity_row.try_get("score")?;
            let score: u32 = score as u32;

            let kills: i32 = activity_row.try_get("kills")?;
            let kills: u32 = kills as u32;

            let deaths: i32 = activity_row.try_get("deaths")?;
            let deaths: u32 = deaths as u32;

            let average_score_per_kill: f32 = activity_row.try_get("average_score_per_kill")?;
            let average_score_per_life: f32 = activity_row.try_get("average_score_per_life")?;
            let completed: i32 = activity_row.try_get("completed")?;
            let completed: u32 = completed as u32;

            let opponents_defeated: i32 = activity_row.try_get("opponents_defeated")?;
            let opponents_defeated: u32 = opponents_defeated as u32;

            let activity_duration_seconds: i32 =
                activity_row.try_get("activity_duration_seconds")?;
            let activity_duration_seconds: u32 = activity_duration_seconds as u32;

            let standing: i32 = activity_row.try_get("standing")?;
            let standing: u32 = standing as u32;

            let team: i32 = activity_row.try_get("team")?;
            let team: u32 = team as u32;

            let completion_reason: i32 = activity_row.try_get("completion_reason")?;
            let completion_reason: u32 = completion_reason as u32;

            let start_seconds: i32 = activity_row.try_get("start_seconds")?;
            let start_seconds: u32 = start_seconds as u32;

            let time_played_seconds: i32 = activity_row.try_get("time_played_seconds")?;
            let time_played_seconds: u32 = time_played_seconds as u32;

            let player_count: i32 = activity_row.try_get("player_count")?;
            let player_count: u32 = player_count as u32;

            let team_score: i32 = activity_row.try_get("team_score")?;
            let team_score: u32 = team_score as u32;

            let precision_kills: i32 = activity_row.try_get("precision_kills")?;
            let precision_kills: u32 = precision_kills as u32;

            let weapon_kills_ability: i32 = activity_row.try_get("weapon_kills_ability")?;
            let weapon_kills_ability: u32 = weapon_kills_ability as u32;

            let weapon_kills_grenade: i32 = activity_row.try_get("weapon_kills_grenade")?;
            let weapon_kills_grenade: u32 = weapon_kills_grenade as u32;

            let weapon_kills_melee: i32 = activity_row.try_get("weapon_kills_melee")?;
            let weapon_kills_melee: u32 = weapon_kills_melee as u32;

            let weapon_kills_super: i32 = activity_row.try_get("weapon_kills_super")?;
            let weapon_kills_super: u32 = weapon_kills_super as u32;

            let all_medals_earned: i32 = activity_row.try_get("all_medals_earned")?;
            let all_medals_earned: u32 = all_medals_earned as u32;

            let character_activity_stats_index: i64 =
                activity_row.try_get("character_activity_stats_index")?;

            let weapon_rows = sqlx::query(
                r#"
                select * from weapon_result where character_activity_stats = ?
            "#,
            )
            .bind(character_activity_stats_index)
            .fetch_all(&mut self.db)
            .await?;

            let mut weapon_stats: Vec<WeaponStat> = Vec::new();
            for weapon_row in &weapon_rows {
                let reference_id: i64 = weapon_row.try_get("reference_id")?;
                let reference_id = reference_id as u32;

                let kills: i32 = weapon_row.try_get("kills")?;
                let precision_kills: i32 = weapon_row.try_get("precision_kills")?;
                let precision_kills_percent: f32 =
                    weapon_row.try_get("kills_precision_kills_ratio")?;

                let item_definition = manifest.get_iventory_item_definition(reference_id).await?;

                let name: String = item_definition
                    .display_properties
                    .description
                    .unwrap_or("".to_string());

                let item: Item = Item {
                    id: reference_id,
                    name: item_definition.display_properties.name,
                    description: name,
                    item_type: item_definition.item_type,
                    item_sub_type: item_definition.item_sub_type,
                };

                let ws = WeaponStat {
                    weapon: item,
                    kills: kills as u32,
                    precision_kills: precision_kills as u32,
                    precision_kills_percent,
                };

                weapon_stats.push(ws);
            }

            let medal_rows = sqlx::query(
                r#"
                select * from medal_result where character_activity_stats = ?
            "#,
            )
            .bind(character_activity_stats_index)
            .fetch_all(&mut self.db)
            .await?;

            let mut medal_stats: Vec<MedalStat> = Vec::new();
            for medal_row in &medal_rows {
                let reference_id: String = medal_row.try_get("reference_id")?;

                let count: i32 = medal_row.try_get("count")?;
                let count: u32 = count as u32;

                let medal_definition = manifest
                    .get_historical_stats_definition(&reference_id)
                    .await?;

                let medal = Medal {
                    id: medal_definition.id,
                    icon_image_path: medal_definition.icon_image_path,
                    tier: medal_definition.medal_tier.unwrap_or(MedalTier::Unknown),
                    name: medal_definition.name,
                    description: medal_definition.description,
                };

                let medal_stat = MedalStat { medal, count };
                medal_stats.push(medal_stat);
            }

            let player = Player {
                member_id: member_id.to_string().clone(),
                character_id: character_id.to_string().clone(),
                platform: *platform,
            };

            let extended = ExtendedCrucibleStats {
                precision_kills,
                weapon_kills_ability,
                weapon_kills_grenade,
                weapon_kills_melee,
                weapon_kills_super,
                all_medals_earned,

                weapons: weapon_stats,
                medals: medal_stats,
            };

            let stats = CrucibleStats {
                assists,
                score,
                kills,
                deaths,
                average_score_per_kill,
                average_score_per_life,
                completed,
                opponents_defeated,
                efficiency: calculate_efficiency(kills, deaths, assists),
                kills_deaths_ratio: calculate_kills_deaths_ratio(kills, deaths),
                kills_deaths_assists: calculate_kills_deaths_assists(kills, deaths, assists),
                activity_duration_seconds,
                standing,
                team,
                completion_reason,
                start_seconds,
                time_played_seconds,
                player_count,
                team_score,
                extended: Some(extended),
            };

            let player_performance = CruciblePlayerPerformance {
                player,
                activity_detail,
                stats,
            };

            performances.push(player_performance);
        }

        println!("{:#?}", performances[0]);

        println!(
            "retrieve_activities_since results returned : {}",
            activity_rows.len()
        );

        Ok(None)
    }
}
