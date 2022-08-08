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

use std::str::FromStr;
use std::{collections::HashMap, path::Path};

use chrono::{DateTime, Utc};

use crate::response::character::CharacterData;
use crate::response::gmd::UserMembershipData;
use crate::utils::calculate_percent;
use crate::{
    crucible::{CrucibleActivity, Member, PlayerName, Team},
    enums::{
        completionreason::CompletionReason,
        itemtype::{ItemSubType, ItemType},
        moment::DateTimePeriod,
        standing::Standing,
    },
    response::pgcr::DestinyPostGameCarnageReportEntry,
};
use futures::TryStreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Row;
use sqlx::{ConnectOptions, SqliteConnection};

use crate::crucible::{
    ActivityDetail, CruciblePlayerActivityPerformance,
    CruciblePlayerPerformance, CrucibleStats, ExtendedCrucibleStats, Item,
    Medal, MedalStat, Player, WeaponStat,
};
use crate::enums::character::{CharacterClass, CharacterClassSelection};
use crate::enums::medaltier::MedalTier;
use crate::enums::mode::Mode;
use crate::enums::platform::Platform;
use crate::{apiinterface::ApiInterface, manifestinterface::ManifestInterface};
use crate::{
    error::Error,
    response::pgcr::{
        DestinyHistoricalStatsValue, DestinyPostGameCarnageReportData,
    },
    utils::{
        calculate_efficiency, calculate_kills_deaths_assists,
        calculate_kills_deaths_ratio,
    },
};

const STORE_FILE_NAME: &str = "dcli.sqlite3";
const STORE_DB_SCHEMA: &str = include_str!("../actitvity_store_schema.sql");

//numer of simultaneous requests we make to server when retrieving activity history
const PGCR_REQUEST_CHUNK_AMOUNT: usize = 24;

const DB_SCHEMA_VERSION: i32 = 9;
const NO_TEAMS_INDEX: i32 = 253;

pub struct ActivityStoreInterface {
    verbose: bool,
    db: SqliteConnection,
    path: String,
    api_interface: ApiInterface,
}

impl ActivityStoreInterface {
    pub fn get_storage_path(&self) -> String {
        self.path.clone()
    }

    pub async fn init_with_path(
        store_dir: &Path,
        verbose: bool,
        key: Option<String>,
    ) -> Result<ActivityStoreInterface, Error> {
        let path = store_dir.join(STORE_FILE_NAME).display().to_string();

        let read_only = false;
        let connection_string: &str = &path;

        //TODO: Is this still the correct / best journal mode for us?
        let mut db = SqliteConnectOptions::from_str(&connection_string)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(read_only)
            .connect()
            .await?;

        //is this an existing db, or a completly new one / first time?

        let should_update_schema = match sqlx::query(
            r#"
            SELECT max(version) as max_version FROM version
        "#,
        )
        .fetch_one(&mut db)
        .await
        {
            Ok(e) => {
                let version: i32 = e.try_get("max_version").unwrap_or(-1);
                version != DB_SCHEMA_VERSION
            }
            Err(_e) => true,
        };

        if should_update_schema {
            eprintln!("Data store needs to be updated.");
            sqlx::query(STORE_DB_SCHEMA).execute(&mut db).await?;
        }

        let api_interface = match key {
            Some(e) => ApiInterface::new_with_key(verbose, &e)?,
            None => ApiInterface::new(verbose)?,
        };

        Ok(ActivityStoreInterface {
            db,
            verbose,
            path: path.to_string(),
            api_interface,
        })
    }

    //todo: this should take a PlayerName
    async fn retrieve_member_by_name(
        &mut self,
        player_name: &PlayerName,
    ) -> Result<Option<Member>, Error> {
        //TODO: check name fields are not null / None

        let row_option = sqlx::query(
            r#"
            SELECT "member_id", "platform_id", "display_name", "bungie_display_name", "bungie_display_name_code" from "member" where bungie_display_name= ? and bungie_display_name_code = ?
        "#,
        )
        .bind(player_name.bungie_display_name.as_ref().unwrap())
        .bind(player_name.bungie_display_name_code.as_ref().unwrap())
        .fetch_optional(&mut self.db)
        .await?;

        //TODO: may want to use this in other areas
        let row = match row_option {
            Some(e) => e,
            None => return Ok(None),
        };

        let member = self.parse_member_row(&row)?;

        Ok(Some(member))
    }

    fn parse_member_row(
        &mut self,
        member_row: &sqlx::sqlite::SqliteRow,
    ) -> Result<Member, Error> {
        let member_id: String = member_row.try_get("member_id")?;
        let platform_id: u32 = member_row.try_get("platform_id")?;
        let display_name: Option<String> =
            member_row.try_get("display_name")?;
        let bungie_display_name: Option<String> =
            member_row.try_get("bungie_display_name")?;
        let bungie_display_name_code: Option<String> =
            member_row.try_get("bungie_display_name_code")?;

        let name: PlayerName = PlayerName {
            display_name,
            bungie_display_name,
            bungie_display_name_code,
        };

        let member: Member = Member {
            name,
            platform: Platform::from_id(platform_id),
            id: member_id,
        };

        Ok(member)
    }

    pub async fn update_sync_entry(
        &mut self,
        member_row_id: i32,
    ) -> Result<i32, Error> {
        let timestamp: String = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT into "sync" ("member", "last_sync") VALUES (?, ?)
            ON CONFLICT(member) DO UPDATE
            set last_sync = ?
        "#,
        )
        .bind(member_row_id)
        .bind(&timestamp)
        .bind(&timestamp)
        .execute(&mut self.db)
        .await?;

        //TODO: what to do if exists? (on conflict)

        let row = sqlx::query(
            r#"
            SELECT id from "sync" where member=?
        "#,
        )
        .bind(member_row_id)
        .fetch_one(&mut self.db)
        .await?;

        let rowid: i32 = row.try_get("id")?;

        Ok(rowid)
    }

    pub async fn find_member(
        &mut self,
        name: &PlayerName,
        store: bool,
    ) -> Result<Member, Error> {
        let member: Option<Member> = self.retrieve_member_by_name(name).await?;

        let out = match member {
            Some(e) => e,
            None => {
                //let api = ApiInterface::new(self.verbose)?;
                let user_info =
                    self.api_interface.search_destiny_player(name).await?;

                let m: Member = user_info.to_member();

                if store {
                    self.insert_member(&m).await?;
                }

                m
            }
        };

        Ok(out)
    }

    pub async fn sync_players(
        &mut self,
        players: &[PlayerName],
    ) -> Result<(), Error> {
        for player in players.iter() {
            match self.sync_player(&player).await {
                Ok(_) => {}
                Err(e) => {
                    if e == Error::BungieNameNotFound {
                        println!(
                            "Name not found: {}. Skipping.",
                            player.get_bungie_name()
                        );
                    } else {
                        return Err(e);
                    }
                }
            };
        }

        Ok(())
    }

    //todo: rename sync to sync member
    pub async fn sync_player(
        &mut self,
        player: &PlayerName,
    ) -> Result<SyncResult, Error> {
        let member = self.find_member(&player, false).await?;

        let out = self.sync_member(&member).await?;
        Ok(out)
    }

    //select all members where sync member = memberid
    pub async fn sync_all(&mut self) -> Result<(), Error> {
        let members: Vec<Member> = self.get_sync_members().await?;

        for member in members.iter() {
            self.sync_member(&member).await?;
        }

        Ok(())
    }

    pub async fn get_sync_members(&mut self) -> Result<Vec<Member>, Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                member.id, member_id, platform_id, display_name, 
                bungie_display_name, bungie_display_name_code
            FROM
                member
            INNER JOIN
                sync on member.id = sync.member
        "#,
        )
        .fetch_all(&mut self.db)
        .await?;

        let mut out: Vec<Member> = Vec::new();
        for row in rows.iter() {
            let member = self.parse_member_row(&row)?;
            out.push(member);
        }

        Ok(out)
    }

    /*
        select member.id, member_id, platform_id, display_name, bungie_display_name, bungie_display_name_code from member
    INNER JOIN
    sync on member.id = sync.member
     */

    pub async fn remove_player_from_sync(
        &mut self,
        player: &PlayerName,
    ) -> Result<(), Error> {
        let row_option = sqlx::query(
            r#"
            SELECT "id" from "member" 
            where bungie_display_name = ? and bungie_display_name_code = ?
        "#,
        )
        .bind(player.bungie_display_name.as_ref().unwrap())
        .bind(player.bungie_display_name_code.as_ref().unwrap())
        .fetch_optional(&mut self.db)
        .await?;

        let id: u32 = match row_option {
            Some(e) => {
                let id: u32 = e.try_get("id")?;
                id
            }
            None => {
                return Ok(());
            }
        };

        sqlx::query(
            r#"
            delete from "sync" where member = ?
        "#,
        )
        .bind(id)
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    pub async fn add_member_to_sync(
        &mut self,
        member: &Member,
    ) -> Result<(), Error> {
        self.begin_transaction().await?;
        let row_id = match self.insert_member(member).await {
            Ok(e) => e,
            Err(e) => {
                self.rollback_transaction().await?;
                return Err(e);
            }
        };

        match self.update_sync_entry(row_id).await {
            Ok(e) => e,
            Err(e) => {
                self.rollback_transaction().await?;
                return Err(e);
            }
        };

        self.commit_transaction().await?;

        Ok(())
    }

    pub async fn add_player_to_sync(
        &mut self,
        player: &PlayerName,
    ) -> Result<(), Error> {
        let member = match self.find_member(player, false).await {
            Ok(e) => e,
            Err(e) => {
                return Err(e);
            }
        };

        self.add_member_to_sync(&member).await?;

        Ok(())
    }

    /// TODO currently no way to sync old / delete characters. would be easy to
    /// add by just moving the character sync into its own api sync_character(id, class_type)
    /// but not going to worry about it unless someone requests it
    /// retrieves and stores activity details for ids in activity queue
    pub async fn sync_member(
        &mut self,
        member: &Member,
    ) -> Result<SyncResult, Error> {
        //let api = ApiInterface::new(self.verbose)?;

        //Note, we need this call in case the user deletes and creates a new character
        //https://www.bungie.net/Platform/Destiny2/1/Profile/4611686018429783292/?components=100,200
        let player_info = self
            .api_interface
            .get_player_info(&member.id, &member.platform)
            .await?;

        let characters = player_info.characters;

        let member_row_id = self
            .insert_member(&player_info.user_info.to_member())
            .await?;

        let mut total_synced = 0;
        let mut total_in_queue = 0;

        eprintln!();

        eprintln!(
            "CHECKING FOR NEW ACTIVITIES FOR {} (PUBLIC AND PRIVATE)",
            member.name.get_bungie_name()
        );
        eprintln!("This may take a few minutes depending on the number of activities.");
        for c in characters.characters {
            let character_id = &c.id;
            let character_row_id = self
                .insert_character_id(&c.id, &c.class_type, member_row_id)
                .await?;
            eprintln!("{}", format!("{}", c.class_type).to_uppercase());

            //these calls could be a little more general purpose by taking api ids and not db ids.
            //however, passing the db ids, lets us optimize a lot of the sql, and avoid
            //some extra calls to the DB

            let a = self.sync_activities(character_row_id).await?;

            let _b = self
                .update_activity_queue(
                    character_row_id,
                    &member.id,
                    character_id,
                    &member.platform,
                )
                .await?;

            let c = self.sync_activities(character_row_id).await?;

            total_synced += a.total_synced + c.total_synced;
            total_in_queue += (a.total_available + c.total_available)
                - (a.total_synced + c.total_synced);
        }

        self.update_sync_entry(member_row_id).await?;

        Ok(SyncResult {
            total_synced,
            total_available: total_in_queue,
        })
    }

    /// download results from ids in queue, and return number of items synced
    async fn sync_activities(
        &mut self,
        character_row_id: i32,
    ) -> Result<SyncResult, Error> {
        let mut ids: Vec<i64> = Vec::new();

        //This is to scope rows, so the mutable borrow of self goes out of scope
        {
            let mut rows = sqlx::query(
                r#"
                    SELECT
                        "activity_id"
                    FROM
                        "activity_queue"
                    WHERE
                        character = ? AND synced = 0
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
            return Ok(SyncResult {
                total_available: 0,
                total_synced: 0,
            });
        }

        let mut filtered_ids = Vec::new();

        //remove ids to sync for activities we have already synced
        for id in ids {
            match self.get_activity_row_id(id).await {
                Ok(_) => {
                    let _ = self
                        .remove_from_activity_queue(&character_row_id, &id)
                        .await?;
                    continue;
                }
                Err(_) => filtered_ids.push(id),
            }
        }
        ids = filtered_ids;

        let total_available = ids.len() as u32;
        let mut total_synced = 0;

        let s = if ids.len() == 1 { "y" } else { "ies" };
        eprintln!(
            "{}",
            format!("Retrieving details for {} activit{}", ids.len(), s)
        );

        eprintln!(
            "Each dot represents {} activities",
            PGCR_REQUEST_CHUNK_AMOUNT
        );
        eprint!("[");
        for id_chunks in ids.chunks(PGCR_REQUEST_CHUNK_AMOUNT) {
            let mut f = Vec::new();

            for c in id_chunks {
                //this is saving the future, call hasnt been made yet
                f.push(
                    self.api_interface.retrieve_post_game_carnage_report(*c),
                );
            }

            eprint!(".");

            //TODO: look into using threading for this
            let results = futures::future::join_all(f).await;

            //loop through. if we get results. grab those, otherwise, we ignore
            //any errors, as that will keep the IDs in the queue to try next time
            //TODO: this is a mess. can we simpify and not nest so deeply?
            for r in results {
                match r {
                    Ok(e) => {
                        match e {
                            Some(e) => match self
                                .insert_activity(&e, character_row_id)
                                .await
                            {
                                Ok(_e) => {
                                    total_synced += 1;
                                }
                                Err(e) => {
                                    eprintln!();
                                    eprintln!(
                                        "Error inserting data into character activity stats table. Skipping. : {}",
                                        e,
                                    );
                                }
                            },
                            None => {
                                eprintln!();
                                eprintln!(
                                    "PGCR returned empty response. Ignoring."
                                );
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
                        eprintln!(
                            "Error retrieving activity details from api. Skipping : {}",
                            e
                        );
                    }
                }
            }
        }

        if !ids.is_empty() {
            sqlx::query("PRAGMA OPTIMIZE;")
                .execute(&mut self.db)
                .await?;
        }

        eprintln!("]");
        eprintln!(
            "{} of {} synced ({}%)",
            total_synced,
            total_available,
            calculate_percent(total_synced, total_available).floor()
        );

        Ok(SyncResult {
            total_available,
            total_synced,
        })
    }

    async fn update_activity_queue(
        &mut self,
        character_row_id: i32,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
    ) -> Result<SyncResult, Error> {
        //TODO catch errors so we can continue?
        let prv_result = self
            ._update_activity_queue(
                character_row_id,
                member_id,
                character_id,
                platform,
                &Mode::PrivateMatchesAll,
            )
            .await?;

        let pub_result = self
            ._update_activity_queue(
                character_row_id,
                member_id,
                character_id,
                platform,
                &Mode::AllPvP,
            )
            .await?;

        Ok(pub_result + prv_result)
    }

    async fn begin_transaction(&mut self) -> Result<(), Error> {
        sqlx::query("BEGIN TRANSACTION;")
            .execute(&mut self.db)
            .await?;

        Ok(())
    }

    async fn commit_transaction(&mut self) -> Result<(), Error> {
        sqlx::query("COMMIT;").execute(&mut self.db).await?;

        Ok(())
    }

    async fn rollback_transaction(&mut self) -> Result<(), Error> {
        sqlx::query("ROLLBACK;").execute(&mut self.db).await?;

        Ok(())
    }

    //updates activity id queue with ids which have not been synced
    async fn _update_activity_queue(
        &mut self,
        character_row_id: i32,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
    ) -> Result<SyncResult, Error> {
        let max_id: i64 =
            self.get_max_activity_id(character_row_id, mode).await?;

        let result = self
            .api_interface
            .retrieve_activities_since_id(
                member_id,
                character_id,
                platform,
                mode,
                max_id,
            )
            .await?;

        if result.is_none() {
            return Ok(SyncResult {
                total_available: 0,
                total_synced: 0,
            });
        }

        let mut activities = result.unwrap();
        eprintln!("{} new activities found", activities.len());

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

        let mut total = 0;

        for activity in activities {
            let director_activity_hash =
                activity.details.director_activity_hash;

            //these are DestinyActivityDefinition manifest hashes for gambit private
            //matches
            //TODO: can rewrite this to short circuit when first result found
            //if !(director_activity_hash != 2526740498 && director_activity_hash != 248695599)

            #[allow(clippy::nonminimal_bool)]
            if director_activity_hash == 2526740498
                || director_activity_hash == 248695599
                || director_activity_hash == 248695599
            {
                //gambit private matches. ignoring

                continue;
            }

            total += 1;

            let instance_id = activity.details.instance_id;

            //TODO: its possible we could insert the activity, and its already been
            //synced for another character. we could potentially detect this, and when we
            //insert it we flag it as already synced. Need to check if this would impact
            //update logic for detecting new activities (seeing which was the latest synced)
            match sqlx::query(
                "INSERT into activity_queue ('activity_id', 'character') VALUES (?, ?)",
            )
            .bind(instance_id)
            .bind(character_row_id)
            .execute(&mut self.db)
            .await
            {
                Ok(_e) => (),
                Err(e) => {
                    sqlx::query("ROLLBACK;").execute(&mut self.db).await?;
                    return Err(Error::from(e));
                }
            };
        }
        sqlx::query("COMMIT;").execute(&mut self.db).await?;

        Ok(SyncResult {
            total_available: total,
            total_synced: total,
        })
    }

    //todo: this doesnt need to be an instance fn, not sure if it matters
    fn get_medal_hash_value(
        &self,
        property: &str,
        medal_hash: &HashMap<String, DestinyHistoricalStatsValue>,
    ) -> u32 {
        match medal_hash.get(property) {
            Some(e) => e.basic.value as u32,
            None => 0,
        }
    }

    async fn insert_activity(
        &mut self,
        data: &DestinyPostGameCarnageReportData,
        character_row_id: i32,
    ) -> Result<(), Error> {
        sqlx::query("BEGIN TRANSACTION;")
            .execute(&mut self.db)
            .await?;

        match self._insert_activity(data, character_row_id).await {
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

    async fn _insert_activity(
        &mut self,
        data: &DestinyPostGameCarnageReportData,
        character_row_id: i32,
    ) -> Result<(), Error> {
        //NOTE: We shouldnt need the check below, as we check for this before
        //we retrieve activites. But right now, keeping it here just as extra safety
        //so data doesn't get in some weird state.
        //this means we have an extra DB call (get_activity_row) for each insert
        /*
        match self
            .get_activity_row_id(data.activity_details.instance_id)
            .await
        {
            Ok(_e) => {
                //If we have already synced it, remove it from queue
                self.remove_from_activity_queue(
                    &character_row_id,
                    &data.activity_details.instance_id,
                )
                .await?;

                return Ok(());
            }
            Err(_e) => (),
        };
        */

        //throw an error if we try to insert and it already exists. That should never
        //happen since we check for that above.
        //NOTE we added OR IGNORE to call calls before. should never need it, since
        //we should never try to insert duplicates
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO "main"."activity"
                ("activity_id","period","mode","platform","director_activity_hash", "reference_id") 
            VALUES (?,?,?,?,?, ?)
        "#,
        )
        .bind(data.activity_details.instance_id) //activity_id
        .bind(data.period.to_rfc3339()) //period
        .bind(data.activity_details.mode.as_id().to_string()) //mode
        .bind(data.activity_details.membership_type.as_id().to_string()) //platform
        .bind(data.activity_details.director_activity_hash.to_string()) //director_activity_hash
        .bind(data.activity_details.reference_id.to_string()) //reference_id
        .execute(&mut self.db)
        .await?;

        let activity_row_id = self
            .get_activity_row_id(data.activity_details.instance_id)
            .await?;

        for team in &data.teams {
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO "main"."team_result"
                (
                    "team_id", "score", "standing", "activity"
                )
                VALUES(?,?, ?, ?)
                "#,
            )
            .bind(team.team)
            .bind(team.score as i32)
            .bind(team.standing as i32)
            .bind(activity_row_id)
            .execute(&mut self.db)
            .await?;
        }

        //TODO: Rumble will have no teams. Need to create one

        for mode in &data.activity_details.modes {
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO "main"."modes"
                (
                    "mode", "activity"
                )
                VALUES(?,?)
                "#,
            )
            .bind(mode.as_id().to_string())
            .bind(activity_row_id)
            .execute(&mut self.db)
            .await?;
        }

        for entry in &data.entries {
            //todo: not sure if we should use membership type of crosssave orveride
            let member_row_id = self
                .insert_member(&entry.player.user_info.to_member())
                .await?;

            let class_type = CharacterClass::from_hash(entry.player.class_hash);

            let character_row_id = self
                .insert_character_id(
                    &entry.character_id,
                    &class_type,
                    member_row_id,
                )
                .await?;

            self._insert_character_activity_stats(
                &entry,
                character_row_id,
                activity_row_id,
            )
            .await?;
        }

        self.remove_from_activity_queue(
            &character_row_id,
            &data.activity_details.instance_id,
        )
        .await?;

        Ok(())
    }

    async fn _insert_character_activity_stats(
        &mut self,
        entry: &DestinyPostGameCarnageReportEntry,
        character_row_id: i32,
        activity_row_id: i32,
    ) -> Result<(), Error> {
        let char_data = entry;

        //There are rare cases where no extended data will be included in a report
        //when that happens we just enter 0 for everything
        let mut precision_kills: u32 = 0;
        let mut weapon_kills_ability: u32 = 0;
        let mut weapon_kills_grenade: u32 = 0;
        let mut weapon_kills_melee: u32 = 0;
        let mut weapon_kills_super: u32 = 0;
        let mut all_medals_earned: u32 = 0;

        let mut medal_hash: &HashMap<String, DestinyHistoricalStatsValue> =
            &HashMap::new();

        if entry.extended.is_some() {
            medal_hash = &entry.extended.as_ref().unwrap().values;

            precision_kills =
                self.get_medal_hash_value("precisionKills", medal_hash);
            weapon_kills_ability =
                self.get_medal_hash_value("weaponKillsAbility", medal_hash);
            weapon_kills_grenade =
                self.get_medal_hash_value("weaponKillsGrenade", medal_hash);
            weapon_kills_melee =
                self.get_medal_hash_value("weaponKillsMelee", medal_hash);
            weapon_kills_super =
                self.get_medal_hash_value("weaponKillsSuper", medal_hash);
            all_medals_earned =
                self.get_medal_hash_value("allMedalsEarned", medal_hash);
        }

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
                "all_medals_earned", "light_level", "activity", "fireteam_id", "emblem_hash"
            )
            VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ? )
            "#,
        )
        //we for through format, as otherwise we have to cast to i32, and while
        //shouldnt be an issue, there is a chance we could lose precision when
        //converting some of the IDS. so we just do this to be consistent.
        //TODO: should think about losing data when pulling out of DB
        .bind(character_row_id as i32) //character
        .bind(char_data.values.assists as i32) //assists
        .bind(char_data.values.score as i32) //score
        .bind(char_data.values.kills as i32) //kiis
        .bind(char_data.values.deaths as i32) //deaths
        .bind(char_data.values.average_score_per_kill) //average_score_per_kill
        .bind(char_data.values.average_score_per_life) //average_score_per_life
        .bind(char_data.values.completed as i32) //completed
        .bind(char_data.values.opponents_defeated as i32) //opponents_defeated
        .bind(format!(
            "{}",
            char_data.values.activity_duration_seconds as u32
        )) //activity_duration_seconds
        .bind(char_data.values.standing as i32) //standing
        .bind(char_data.values.team as i32) //team
        .bind(char_data.values.completion_reason as i32) //completion_reason
        .bind(char_data.values.start_seconds as i32) //start_seconds
        .bind(char_data.values.time_played_seconds as i32) //time_played_seconds
        .bind(char_data.values.player_count as i32) //player_count
        .bind(char_data.values.team_score as i32) //team_score
        .bind(precision_kills as i32) //precision_kills
        .bind(weapon_kills_ability as i32) //weapon_kills_ability
        .bind(weapon_kills_grenade as i32) //weapon_kills_grenade
        .bind(weapon_kills_melee as i32) //weapon_kills_melee
        .bind(weapon_kills_super as i32) //weapon_kills_super
        .bind(all_medals_earned as i32) //weapon_kills_super
        .bind(char_data.player.light_level) //activity
        .bind(activity_row_id)
        .bind(char_data.values.fireteam_id.to_string())
        .bind(char_data.player.emblem_hash) //activity
        .execute(&mut self.db)
        .await?;

        //character_activity_stats

        let row = sqlx::query(
            r#"
            SELECT "id" FROM "character_activity_stats" WHERE activity = ? and character = ?
        "#,
        )
        .bind(activity_row_id)
        .bind(character_row_id)
        .fetch_one(&mut self.db)
        .await?;

        let character_activity_stats_id: i32 = row.try_get("id")?;

        for (key, value) in medal_hash {
            sqlx::query(
                r#"
                INSERT INTO "main"."medal_result"
                (
                    "reference_id", "count", "character_activity_stats"
                )
                VALUES  (
                    ?,?,?
                )
                "#,
            )
            .bind(key) //reference_id
            .bind(format!("{}", value.basic.value as u32)) //unique_weapon_kills
            .bind(character_activity_stats_id)
            .execute(&mut self.db)
            .await?;
        }

        //ran into a case once where weapons was missing, so have to check here
        if entry.extended.is_some() {
            let extended = char_data.extended.as_ref().unwrap();
            if extended.weapons.is_some() {
                let weapons = extended.weapons.as_ref().unwrap();
                for w in weapons {
                    sqlx::query(
                        r#"
                        INSERT INTO "main"."weapon_result"
                        (
                            "reference_id", "kills", "precision_kills", "kills_precision_kills_ratio", "character_activity_stats"
                        )
                        VALUES (?, ?, ?, ?, ?)
                        "#,
                    )
                    .bind(format!("{}", w.reference_id)) //reference_id
                    .bind(format!("{}", w.values.unique_weapon_kills as u32)) //unique_weapon_kills
                    .bind(format!("{}", w.values.unique_weapon_precision_kills as u32)) //unique_weapon_precision_kills
                    .bind(format!("{}", w.values.unique_weapon_kills_precision_kills)) //unique_weapon_kills_precision_kills
                    .bind(character_activity_stats_id)
                    .execute(&mut self.db)
                    .await?;
                }
            }
        }

        Ok(())
    }

    async fn remove_from_activity_queue(
        &mut self,
        character_row_id: &i32,
        instance_id: &i64,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE "main"."activity_queue"
            SET synced = 1
            WHERE character = ? and activity_id = ?
        "#,
        )
        .bind(character_row_id.to_string())
        .bind(instance_id)
        .execute(&mut self.db)
        .await?;

        Ok(())
    }

    async fn get_activity_row_id(
        &mut self,
        instance_id: i64,
    ) -> Result<i32, Error> {
        let row = sqlx::query(
            r#"
            SELECT "id" FROM "activity" WHERE activity_id = ?
        "#,
        )
        .bind(instance_id.to_string())
        .fetch_one(&mut self.db)
        .await?;

        let id: i32 = row.try_get("id")?;

        Ok(id)
    }

    async fn insert_member(&mut self, mem: &Member) -> Result<i32, Error> {
        let mut member = mem;
        let mut m = None;
        //check and see if data is missing (happens a lot with the API)
        if member.name.display_name.is_none()
            || member.name.bungie_display_name_code.is_none()
            || member.platform == Platform::Unknown
        {
            //if some data is missing, the see if we already have info on
            //this member stored.

            let row_id: i32 = match self.get_member_row_id(member).await {
                Ok(e) => e,
                Err(_e) => -1,
            };

            //if we do have something stored, just return that entry, otherwise,
            //add it to the DB, even though the name info in missing
            if row_id != -1 {
                return Ok(row_id);
            }

            //need to catch and ignore error here
            let card_result = self
                .api_interface
                .retrieve_player_info_by_id(&member.id)
                .await;

            if card_result.is_ok() {
                let c = card_result.unwrap();
                let cards = c.destiny_memberships;

                for card in cards.iter() {
                    if card.cross_save_override == card.membership_type {
                        m = Some(card.to_member());
                        break;
                    }
                }

                //if we get here, then we need to go to the server to try and get the
                //data
                if m.is_some() {
                    member = m.as_ref().unwrap();
                }
            }
        }

        sqlx::query(
            r#"
            INSERT into "member" ("member_id", "platform_id", "display_name", "bungie_display_name", "bungie_display_name_code") VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(member_id) DO UPDATE
            set display_name = ?, bungie_display_name = ?, bungie_display_name_code = ?
        "#,
        )
        .bind(member.id.to_string())
        .bind(member.platform.as_id().to_string())
        .bind(&member.name.display_name)
        .bind(&member.name.bungie_display_name)
        .bind(&member.name.bungie_display_name_code)
        .bind(&member.name.display_name)
        .bind(&member.name.bungie_display_name)
        .bind(&member.name.bungie_display_name_code)
        .execute(&mut self.db)
        .await?;

        let rowid: i32 = self.get_member_row_id(member).await?;

        Ok(rowid)
    }

    async fn get_member_row_id(
        &mut self,
        member: &Member,
    ) -> Result<i32, Error> {
        let row = sqlx::query(
            r#"
            SELECT id from "member" where member_id=?
        "#,
        )
        .bind(member.id.to_string())
        .fetch_one(&mut self.db)
        .await?;

        let rowid: i32 = row.try_get("id")?;

        Ok(rowid)
    }

    async fn insert_character_id(
        &mut self,
        character_id: &str,
        class_type: &CharacterClass,
        member_rowid: i32,
    ) -> Result<i32, Error> {
        sqlx::query(
            r#"
            INSERT OR IGNORE into "character" ("character_id", "member", "class") VALUES (?, ?, ?)
        "#,
        )
        .bind(character_id.to_string())
        .bind(member_rowid)
        .bind(class_type.as_id().to_string())
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

    async fn get_max_activity_id(
        &mut self,
        character_row_id: i32,
        mode: &Mode,
    ) -> Result<i64, Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                activity_queue.activity_id as max_activity_id
            FROM
                "activity_queue"
            INNER JOIN
                activity ON activity_queue.activity_id = activity.activity_id,
                character_activity_stats ON character_activity_stats.activity = activity.id,
                character on character_activity_stats.character = character.id,
                modes ON modes.activity = activity.id and modes.mode in (select mode from modes where mode = ?)
            WHERE
                character_activity_stats.character = ? AND
                activity_queue.character = ?
            ORDER BY activity.period DESC LIMIT 1
        "#,
        )
        .bind(mode.as_id().to_string())
        .bind(character_row_id.to_string())
        .bind(character_row_id.to_string())
        .fetch_all(&mut self.db)
        .await?;

        if rows.is_empty() {
            return Ok(0);
        }

        let row = &rows[0];
        let activity_id: i64 = row.try_get("max_activity_id")?;
        Ok(activity_id)
    }

    pub async fn retrieve_activity_by_index(
        &mut self,
        activity_index: u32,
        manifest: &mut ManifestInterface,
    ) -> Result<CrucibleActivity, Error> {
        let activity_row = match sqlx::query(
            r#"
            SELECT
                activity.id as activity_index_id,
                activity.activity_id,
                activity.period,
                activity.mode as activity_mode,
                activity.director_activity_hash,
                activity.reference_id,
                activity.platform
            FROM
                activity
            INNER JOIN
                character_activity_stats on character_activity_stats.activity = activity.id,
                character on character_activity_stats.character = character.id,
                member on character.member = member.id
            WHERE
                activity.id = ?
            ORDER BY
                period DESC LIMIT 1
            "#,
        )
        .bind(activity_index.to_string())
        .fetch_one(&mut self.db)
        .await
        {
            Ok(e) => e,
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    return Err(Error::ActivityNotFound);
                }
                _ => {
                    return Err(Error::from(e));
                }
            },
        };

        let crucible_activity =
            self.populate_activity_data(&activity_row, manifest).await?;
        Ok(crucible_activity)
    }

    pub async fn retrieve_last_activity(
        &mut self,
        member: &Member,
        character_selection: &CharacterClassSelection,
        mode: &Mode,
        manifest: &mut ManifestInterface,
    ) -> Result<CrucibleActivity, Error> {
        let member_id = &member.id;

        let activity_row = if character_selection
            == &CharacterClassSelection::All
        {
            match sqlx::query(
                r#"
                SELECT
                    activity.id as activity_index_id,
                    activity.activity_id,
                    activity.period,
                    activity.mode as activity_mode,
                    activity.director_activity_hash,
                    activity.reference_id,
                    activity.platform
                FROM
                    activity
                INNER JOIN
                    character_activity_stats on character_activity_stats.activity = activity.id,
                    character on character_activity_stats.character = character.id,
                    member on character.member = member.id AND member.member_id = ?
                WHERE
                    exists (select 1 from modes where activity = activity.id and mode = ?)
                ORDER BY
                    period DESC LIMIT 1
                "#,
            )
            .bind(member_id.to_string())
            .bind(mode.as_id().to_string())
            .fetch_one(&mut self.db)
            .await
            {
                Ok(e) => e,
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        return Err(Error::ActivityNotFound);
                    }
                    _ => {
                        return Err(Error::from(e));
                    }
                },
            }
        } else {
            let character_data = self
                .retrieve_character_selection_data(member, character_selection)
                .await?;

            match sqlx::query(
                    r#"
                    SELECT
                        activity.id as activity_index_id,
                        activity.activity_id,
                        activity.period,
                        activity.mode as activity_mode,
                        activity.director_activity_hash,
                        activity.reference_id,
                        activity.platform
                    FROM
                        activity
                    INNER JOIN
                        character_activity_stats on character_activity_stats.activity = activity.id,
                        character on character_activity_stats.character = character.id AND character.character_id = ?
                    WHERE
                        exists (select 1 from modes where activity = activity.id and mode = ?)
                    ORDER BY
                        period DESC LIMIT 1
                    "#
                ).bind(character_data.id.to_string())
                .bind(mode.as_id().to_string())
                .fetch_one(&mut self.db)
                .await
                {
                    Ok(e) => e,
                    Err(e) => match e {
                        sqlx::Error::RowNotFound => {
                            return Err(Error::ActivityNotFound);
                        }
                        _ => {
                            return Err(Error::from(e));
                        }
                    },
                }
        };

        let crucible_activity =
            self.populate_activity_data(&activity_row, manifest).await?;
        Ok(crucible_activity)
    }

    async fn populate_activity_data(
        &mut self,
        activity_row: &sqlx::sqlite::SqliteRow,
        manifest: &mut ManifestInterface,
    ) -> Result<CrucibleActivity, Error> {
        let activity_row_id: i32 = activity_row.try_get("activity_index_id")?;

        let team_rows = sqlx::query(
            r#"
            SELECT
                *
            FROM
                team_result
            WHERE
                activity = ?
            "#,
        )
        .bind(activity_row_id)
        .fetch_all(&mut self.db)
        .await?;

        let mut teams: HashMap<i32, Team> = HashMap::new();

        let mut team_names = vec![
            "Alpha".to_string(),
            "Bravo".to_string(),
            "Charlie".to_string(),
            "Delta".to_string(),
            "Echo".to_string(),
            "Foxtrot".to_string(),
        ];
        team_names.reverse();

        for t in team_rows {
            let standing: i32 = t.try_get("standing")?;
            let standing = Standing::from_value(standing as u32);

            let id: i32 = t.try_get("team_id")?;
            let score: u32 = t.try_get("score")?;

            let player_performances: Vec<CruciblePlayerPerformance> =
                Vec::new();

            let display_name =
                team_names.pop().unwrap_or_else(|| "".to_string());

            let team = Team {
                id,
                standing,
                score,
                player_performances,
                display_name,
            };

            teams.insert(id, team);
        }

        //Rumble wont have any teams, so we put all items in one team
        //this also covered any bugs where no teams are specified
        let mut no_teams = false;
        if teams.is_empty() {
            let display_name =
                team_names.pop().unwrap_or_else(|| "".to_string());

            let team = Team {
                standing: Standing::Unknown,
                id: NO_TEAMS_INDEX,
                score: 0,
                player_performances: Vec::new(),
                display_name,
            };

            teams.insert(NO_TEAMS_INDEX, team);
            no_teams = true;
        }

        //TODO: need to account for character and member, need to join both
        let character_rows = sqlx::query(
            r#"
            SELECT
                *,
                character_activity_stats.id as character_activity_stats_index
            FROM
                character_activity_stats
            INNER JOIN
                character on character_activity_stats.character = character.id,
                member on character.member = member.id
            WHERE
                activity = ?
            "#,
        )
        .bind(activity_row_id)
        .fetch_all(&mut self.db)
        .await?;

        for c_row in character_rows {
            let stats = self.parse_crucible_stats(manifest, &c_row).await?;

            let player = self.parse_player(&c_row).await?;

            let cpp = CruciblePlayerPerformance { player, stats };

            let index = if no_teams {
                NO_TEAMS_INDEX
            } else {
                cpp.stats.team
            };

            match teams.get_mut(&index) {
                Some(e) => e.player_performances.push(cpp),
                None => eprintln!("Invalid Team ID ({}) : Skipping", &index),
            }
        }

        let details = self.parse_activity(manifest, &activity_row).await?;

        Ok(CrucibleActivity { details, teams })
    }

    //returns character_id for specified character class selection
    //returns member_id if selection is ALL
    async fn retrieve_character_selection_data(
        &self,
        member: &Member,
        character_selection: &CharacterClassSelection,
    ) -> Result<CharacterData, Error> {
        let member_id = &member.id;
        let platform = &member.platform;

        let api = ApiInterface::new(self.verbose)?;
        //first, lets get all of the current characters for the member
        let characters = api
            .retrieve_characters(member_id, platform)
            .await?
            .ok_or(Error::NoCharacters)?;

        let out = match character_selection {
            CharacterClassSelection::All => {
                panic!("CharacterClassSelection::All pass to retrieve_character_selection_data which is not supported.");
            }
            CharacterClassSelection::Hunter => {
                match characters.get_by_class(CharacterClass::Hunter) {
                    Some(e) => e,
                    None => return Err(Error::CharacterDoesNotExist),
                }
            }
            CharacterClassSelection::Titan => {
                match characters.get_by_class(CharacterClass::Titan) {
                    Some(e) => e,
                    None => return Err(Error::CharacterDoesNotExist),
                }
            }
            CharacterClassSelection::Warlock => {
                match characters.get_by_class(CharacterClass::Warlock) {
                    Some(e) => e,
                    None => return Err(Error::CharacterDoesNotExist),
                }
            }
            CharacterClassSelection::LastActive => {
                match characters.get_last_active() {
                    Some(e) => e,
                    None => return Err(Error::CharacterDoesNotExist),
                }
            }
        };

        Ok(out)
    }

    pub async fn retrieve_activities_since(
        &mut self,
        member: &Member,
        character_selection: &CharacterClassSelection,
        mode: &Mode,
        time_period: &DateTimePeriod,
        manifest: &mut ManifestInterface,
    ) -> Result<Option<Vec<CruciblePlayerActivityPerformance>>, Error> {
        //if mode if private, we dont restrict results
        let restrict_mode_id = if mode.is_private() {
            -1
        } else {
            //if not private, then we dont include any results that are private
            Mode::PrivateMatchesAll.as_id() as i32
        };

        //this is running about 550ms
        //TODO: this currently works because the bungie api for private only returns 32
        //and does not contain submodes. so we only get private results if we explicitly
        //search for private all (32), and dont get no private results. however,
        //if bungie fixes this and starts include additional mode data (i.e. private control)
        //then this will start to mix private and all when searching for control.
        //need to see if its a private or non-private and then exclude others.
        let activity_rows = if character_selection
            == &CharacterClassSelection::All
        {
            sqlx::query(
            r#"
            SELECT
                *,
                activity.mode as activity_mode,
                activity.id as activity_index_id,
                character_activity_stats.id as character_activity_stats_index  
            FROM
                character_activity_stats
            INNER JOIN
                activity ON character_activity_stats.activity = activity.id,
                character on character_activity_stats.character = character.id,
                member on member.id = character.member
            WHERE
                member.id = (select id from member where member_id = ?) AND
                period > ? AND
                period < ? AND
                exists (select 1 from modes where activity = activity.id and mode = ?) AND
                not exists (select 1 from modes where activity = activity.id and mode = ?)
            ORDER BY
                activity.period DESC
            "#,
        )
        .bind(member.id.to_string())
        .bind(time_period.get_start().to_rfc3339())
        .bind(time_period.get_end().to_rfc3339())
        .bind(mode.as_id().to_string())
        .bind(restrict_mode_id.to_string())
        .fetch_all(&mut self.db)
        .await?
        } else {
            let character_data = self
                .retrieve_character_selection_data(&member, character_selection)
                .await?;

            //todo: note, this might break if user has multiple characters of the same
            //class. need to test
            sqlx::query(
            r#"
            SELECT
                *,
                activity.mode as activity_mode,
                activity.id as activity_index_id,
                character_activity_stats.id as character_activity_stats_index  
            FROM
                character_activity_stats
            INNER JOIN
                activity ON character_activity_stats.activity = activity.id,
                character on character_activity_stats.character = character.id,
                member on member.id = character.member
            WHERE
                member.id = (select id from member where member_id = ?) AND
                activity.period > ? AND
                activity.period < ? AND
                exists (select 1 from modes where activity = activity.id and mode = ?) AND
                not exists (select 1 from modes where activity = activity.id and mode = ?) AND
                character_activity_stats.character = character.id AND
                character.class = ?
            ORDER BY
                activity.period DESC
            "#,
        )
        .bind(member.id.to_string())
        .bind(time_period.get_start().to_rfc3339())
        .bind(time_period.get_end().to_rfc3339())
        .bind(mode.as_id().to_string())
        .bind(restrict_mode_id.to_string())
        .bind(character_data.class_type.as_id())
        .fetch_all(&mut self.db)
        .await?
        };

        if activity_rows.is_empty() {
            return Ok(None);
        }

        let p = self
            .parse_individual_performance_rows(manifest, &activity_rows)
            .await?;

        Ok(Some(p))
    }

    async fn parse_individual_performance_rows(
        &mut self,
        manifest: &mut ManifestInterface,
        activity_rows: &[sqlx::sqlite::SqliteRow],
    ) -> Result<Vec<CruciblePlayerActivityPerformance>, Error> {
        let mut performances: Vec<CruciblePlayerActivityPerformance> =
            Vec::with_capacity(activity_rows.len());

        for activity_row in activity_rows {
            let player_performance = self
                .parse_individual_performance_row(manifest, &activity_row)
                .await?;

            performances.push(player_performance);
        }
        //performances.sort_by(|a, b| a.activity_detail.period.cmp(&b.activity_detail.period));
        //let p = AggregateCruciblePerformances::with_performances(performances);

        Ok(performances)
    }

    async fn parse_activity(
        &mut self,
        manifest: &mut ManifestInterface,
        activity_row: &sqlx::sqlite::SqliteRow,
    ) -> Result<ActivityDetail, Error> {
        let activity_id: i64 = activity_row.try_get("activity_id")?;

        let mode_id: u32 = activity_row.try_get_unchecked("activity_mode")?;
        let platform_id: u32 = activity_row.try_get_unchecked("platform")?;

        let period: String = activity_row.try_get_unchecked("period")?;
        let period = DateTime::parse_from_rfc3339(&period)?;
        let period = period.with_timezone(&Utc);

        let director_activity_hash: i64 =
            activity_row.try_get_unchecked("director_activity_hash")?;
        let director_activity_hash: u32 = director_activity_hash as u32;

        let reference_id: u32 =
            activity_row.try_get_unchecked("reference_id")?;

        let index_id: u32 =
            activity_row.try_get_unchecked("activity_index_id")?;
        let activity_definition =
            manifest.get_activity_definition(reference_id).await?;

        let map_name = match activity_definition {
            Some(e) => e.display_properties.name,
            None => "Unknown".to_string(),
        };

        let activity_detail = ActivityDetail {
            index_id,
            id: activity_id,
            period,
            map_name,
            mode: Mode::from_id(mode_id)?,
            platform: Platform::from_id(platform_id),
            director_activity_hash,
            reference_id,
        };

        Ok(activity_detail)
    }

    async fn parse_crucible_stats(
        &mut self,
        manifest: &mut ManifestInterface,
        activity_row: &sqlx::sqlite::SqliteRow,
    ) -> Result<CrucibleStats, Error> {
        let assists: u32 = activity_row.try_get_unchecked("assists")?;
        let score: u32 = activity_row.try_get_unchecked("score")?;
        let kills: u32 = activity_row.try_get_unchecked("kills")?;
        let deaths: u32 = activity_row.try_get_unchecked("deaths")?;

        let average_score_per_kill: f32 =
            activity_row.try_get_unchecked("average_score_per_kill")?;
        let average_score_per_life: f32 =
            activity_row.try_get_unchecked("average_score_per_life")?;
        let completed: i32 = activity_row.try_get_unchecked("completed")?;
        let completed: bool = completed == 1;

        let opponents_defeated: u32 =
            activity_row.try_get_unchecked("opponents_defeated")?;

        let activity_duration_seconds: u32 =
            activity_row.try_get_unchecked("activity_duration_seconds")?;

        let standing: u32 = activity_row.try_get_unchecked("standing")?;
        let standing: Standing = Standing::from_value(standing);

        let team: i32 = activity_row.try_get_unchecked("team")?;

        let completion_reason: u32 =
            activity_row.try_get_unchecked("completion_reason")?;
        let completion_reason = CompletionReason::from_id(completion_reason);

        let start_seconds: u32 =
            activity_row.try_get_unchecked("start_seconds")?;

        let time_played_seconds: u32 =
            activity_row.try_get_unchecked("time_played_seconds")?;

        let player_count: u32 =
            activity_row.try_get_unchecked("player_count")?;

        let team_score: u32 = activity_row.try_get_unchecked("team_score")?;

        let precision_kills: u32 =
            activity_row.try_get_unchecked("precision_kills")?;

        let weapon_kills_ability: u32 =
            activity_row.try_get_unchecked("weapon_kills_ability")?;

        let weapon_kills_grenade: u32 =
            activity_row.try_get_unchecked("weapon_kills_grenade")?;

        let weapon_kills_melee: u32 =
            activity_row.try_get_unchecked("weapon_kills_melee")?;

        let weapon_kills_super: u32 =
            activity_row.try_get_unchecked("weapon_kills_super")?;

        let all_medals_earned: u32 =
            activity_row.try_get_unchecked("all_medals_earned")?;

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

        let win_count: u32 = match standing {
            Standing::Victory => 1,
            _ => 0,
        };

        let mut weapon_stats: Vec<WeaponStat> =
            Vec::with_capacity(weapon_rows.len());
        for weapon_row in &weapon_rows {
            let reference_id: u32 =
                weapon_row.try_get_unchecked("reference_id")?;

            let kills: u32 = weapon_row.try_get_unchecked("kills")?;
            let precision_kills: u32 =
                weapon_row.try_get_unchecked("precision_kills")?;
            let precision_kills_percent: f32 =
                weapon_row.try_get("kills_precision_kills_ratio")?;

            let item_definition =
                manifest.get_iventory_item_definition(reference_id).await?;

            //TODO: catch error here if not found

            let description: String;
            let name: String;
            let item_type: ItemType;
            let item_sub_type: ItemSubType;

            match item_definition {
                Some(e) => {
                    description = e
                        .display_properties
                        .description
                        .unwrap_or_else(|| "".to_string());
                    name = e.display_properties.name;
                    item_type = e.item_type;
                    item_sub_type = e.item_sub_type;
                }
                None => {
                    name = "Unknown".to_string();
                    description = "".to_string();
                    item_type = ItemType::Unknown;
                    item_sub_type = ItemSubType::Unknown;
                }
            };

            let item: Item = Item {
                id: reference_id,
                name,
                description,
                item_type,
                item_sub_type,
            };

            let ws = WeaponStat {
                weapon: item,
                kills,
                precision_kills,
                precision_kills_percent,
                activity_count: 1,
                wins: win_count,
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

        let mut medal_stats: Vec<MedalStat> =
            Vec::with_capacity(medal_rows.len());
        for medal_row in &medal_rows {
            let reference_id: String =
                medal_row.try_get_unchecked("reference_id")?;

            let count: u32 = medal_row.try_get_unchecked("count")?;

            let medal_definition = manifest
                .get_historical_stats_definition(&reference_id)
                .await?;

            let id: String;
            let icon_image_path: Option<String>;
            let tier: MedalTier;
            let name: String;
            let description: String;

            match medal_definition {
                Some(e) => {
                    id = e.id;
                    icon_image_path = e.icon_image_path;
                    tier = e.medal_tier.unwrap_or(MedalTier::Unknown);
                    name = e.name;
                    description = e.description;
                }
                None => {
                    id = reference_id;
                    icon_image_path = None;
                    tier = MedalTier::Unknown;
                    name = "Unknown".to_string();
                    description = "".to_string();
                }
            };

            let medal = Medal {
                id,
                icon_image_path,
                tier,
                name,
                description,
            };

            let medal_stat = MedalStat { medal, count };
            medal_stats.push(medal_stat);
        }

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
            kills_deaths_assists: calculate_kills_deaths_assists(
                kills, deaths, assists,
            ),
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

        Ok(stats)
    }

    async fn parse_player(
        &mut self,
        activity_row: &sqlx::sqlite::SqliteRow,
    ) -> Result<Player, Error> {
        let member_id: String = activity_row.try_get_unchecked("member_id")?;
        let character_id = activity_row.try_get_unchecked("character_id")?;
        let platform_id: u32 = activity_row.try_get_unchecked("platform_id")?;
        let display_name: Option<String> =
            activity_row.try_get_unchecked("display_name")?;

        let bungie_display_name: Option<String> =
            activity_row.try_get_unchecked("bungie_display_name")?;

        let bungie_display_name_code: Option<String> =
            activity_row.try_get_unchecked("bungie_display_name_code")?;

        let light_level: i32 = activity_row.try_get_unchecked("light_level")?;
        let class_type: u32 = activity_row.try_get_unchecked("class")?;
        let class_type: CharacterClass = CharacterClass::from_id(class_type);

        let platform = Platform::from_id(platform_id);

        let name = PlayerName {
            display_name,
            bungie_display_name,
            bungie_display_name_code,
        };

        let player = Player {
            member_id,
            character_id,
            platform,
            name,
            light_level,
            class_type,
        };

        Ok(player)
    }

    async fn parse_individual_performance_row(
        &mut self,
        manifest: &mut ManifestInterface,
        activity_row: &sqlx::sqlite::SqliteRow,
    ) -> Result<CruciblePlayerActivityPerformance, Error> {
        let activity_detail =
            self.parse_activity(manifest, activity_row).await?;
        let stats = self.parse_crucible_stats(manifest, activity_row).await?;
        let player = self.parse_player(activity_row).await?;

        let performance = CruciblePlayerPerformance { player, stats };

        let player_performance = CruciblePlayerActivityPerformance {
            performance,
            activity_detail,
        };

        Ok(player_performance)
    }
}

#[derive(Debug)]
pub struct SyncResult {
    pub total_available: u32,
    pub total_synced: u32,
}

impl std::ops::Add<SyncResult> for SyncResult {
    type Output = SyncResult;

    fn add(self, sr: SyncResult) -> SyncResult {
        SyncResult {
            total_available: self.total_available + sr.total_available,
            total_synced: self.total_synced + sr.total_synced,
        }
    }
}
