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

use crate::error::Error;

use futures::TryStreamExt;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Row;
use sqlx::{ConnectOptions, Connection, SqliteConnection};
use std::str::FromStr;
use std::path::PathBuf;
use crate::platform::Platform;
use crate::apiinterface::ApiInterface;
use crate::mode::Mode;


pub struct ActivityStoreInterface {
    verbose:bool,
    db:SqliteConnection,
}

impl ActivityStoreInterface {

    pub async fn init_with_path(store_path:&PathBuf, verbose:bool) -> Result<ActivityStoreInterface, Error> {

        let path: String = format!("{}", store_path.display());
        let read_only = false;
        let connection_string: &str = &path;

        let mut db = SqliteConnectOptions::from_str(&connection_string)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(read_only)
            .connect()
            .await?;

        sqlx::query(r#"
        BEGIN TRANSACTION;

        /* found activities we havent synced details from yet */
        CREATE TABLE IF NOT EXISTS "main"."activity_queue" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "activity_id" TEXT NOT NULL,
            "character"	INTEGER NOT NULL,
            FOREIGN KEY (character)
               REFERENCES character (id)
               ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS  "member" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "member_id"	TEXT NOT NULL,
            "platform_id"	INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS  "character" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "character_id"	TEXT NOT NULL,
            "member"	INTEGER NOT NULL,
            FOREIGN KEY ("member")
               REFERENCES member ("id")
               ON DELETE CASCADE
        );
        
        
        CREATE TABLE IF NOT EXISTS "main"."activity" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "activity_id"	INTEGER UNIQUE NOT NULL,
            "period" TEXT NOT NULL,
            "mode" INTEGER NOT NULL,
            "platform" INTEGER NOT NULL,
            "director_activity_hash" INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS "main"."character_activity_stats" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "character"	INTEGER NOT NULL,
        
            /* we use id and not rowid since we shouldnt have dupes */
            "activity"	INTEGER NOT NULL,
        
            FOREIGN KEY (activity)
               REFERENCES activity (id)
               ON DELETE CASCADE,
        
            FOREIGN KEY (character)
               REFERENCES character (id)
               ON DELETE CASCADE
        );
        COMMIT;
            
            "#)
            .execute(&mut db)
            .await?;

        Ok(ActivityStoreInterface{db:db, verbose:verbose})
    }

    /// retrieves and stores activity details for ids in activity queue
    pub async fn sync(&mut self, member_id:&str, character_id:&str, platform:&Platform) -> Result<(), Error> {

        self.update_activity_queue(member_id, character_id, platform).await?;

        //self.sync_activities(member_id, character_id, platform).await?;

        //return total synced?

        Ok(())
    }

    /// download results from ids in queue, and return number of items synced
    async fn sync_activities(&mut self, member_id:&str, character_id:&str, platform:&Platform) -> Result<i32, Error> {
        
        
        Ok(0)
    }

    //updates activity id queue with ids which have not been synced
    async fn update_activity_queue(&mut self, member_id:&str, character_id:&str, platform:&Platform) -> Result<(), Error> {

        self.sync_activities(member_id, character_id, platform).await?;

        let max_id:String = "7588684064".to_string();

        let api = ApiInterface::new(self.verbose);

        let activities = api.retrieve_activities_since_id(
            member_id, character_id, platform, &Mode::AllPvP, &max_id).await?;

        if activities.is_none() {
            println!("No new activities found");
            return Ok(());
        }

        let activities = activities.unwrap();

                //INSERT INTO "main"."platform"("platform_id","name") VALUES (1,'Xbox');

        //sqlx::query("DELETE FROM table").execute(&mut conn).await?;
 

        println!("Activities found: {}", activities.len());

        let member_row_id = self.insert_member_id(&member_id, &platform).await?;

println!("member_row_id : {}", member_row_id);

        let character_row_id = self.insert_character_id(&character_id, member_row_id).await?;
println!("character_row_id : {}", character_row_id);
        //select max(id) from activities where character = character  

        //retrieve ids until that id is found

        //insert or ignore into member member_id
        //select rowid from member where member_id = member_id
        //insert or ignore into character character_id, member_id_row
        //select character id where character_id = character_id member_id = member_id_row
        //insert into activity queue activity id, character id

        Ok(())
    }

    async fn insert_member_id(&mut self, member_id:&str, platform:&Platform) -> Result<i32, Error> {
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

        let rowid:i32 = row.try_get("id")?;

        Ok(rowid)
    }

    async fn insert_character_id(&mut self, character_id:&str, member_rowid:i32)  -> Result<i32, Error> {
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

        let rowid:i32 = row.try_get("id")?;

        Ok(rowid)
    }

}
