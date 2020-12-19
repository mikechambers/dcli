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


pub struct ActivityStoreInterface {
    db:SqliteConnection,
}

impl ActivityStoreInterface {

    pub async fn init_with_path(store_path:&PathBuf) -> Result<ActivityStoreInterface, Error> {

        let path: String = format!("{}", store_path.display());
        let read_only = false;
        let connection_string: &str = &path;

        let mut db = SqliteConnectOptions::from_str(&connection_string)?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .read_only(read_only)
            .connect()
            .await?;

        sqlx::query("
            BEGIN TRANSACTION;
            CREATE TABLE IF NOT EXISTS 'main'.'activity_id_queue' (
                'activity_id'	INTEGER NOT NULL,
                'member_id'	TEXT NOT NULL,
                'character_id'	TEXT NOT NULL,
                'platform_id'	INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS 'character_result' (
                'character_id'	INTEGER NOT NULL,
                'activity_id'	INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS 'character' (
                'character_id'	INTEGER NOT NULL, //primary
                'member_id'	INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS 'member' (
                'member_id'	INTEGER NOT NULL, //primary
                'platform_id'	INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS 'activity' (
                'activity_id'	INTEGER NOT NULL //PRIMARY KEY
            );
            COMMIT;
        ")
            .execute(&mut db)
            .await?;

        Ok(ActivityStoreInterface{db:db})
    }

    /// retrieves and stores activity details for ids in activity queue
    pub async fn sync(&self, member_id:&str, character_id:&str, platform:&Platform) -> Result<(), Error> {

        self.update_activity_queue(member_id, character_id, platform).await?;

        self.sync_activity_queue(member_id, character_id, platform).await?;

        //return total synced?

        Ok(())
    }

    /// download results from ids in queue, and return number of items synced
    async fn sync_activity_queue(&self, member_id:&str, character_id:&str, platform:&Platform) -> Result<i32, Error> {
        Ok(0)
    }

    //updates activity id queue with ids which have not been synced
    async fn update_activity_queue(&self, member_id:&str, character_id:&str, platform:&Platform) -> Result<(), Error> {

        self.sync_activity_queue(member_id, character_id, platform).await?;

        //select max id

        //retrieve ids until that id is found

        //write ids to queue

        //return number of new activities found? and total number in queue

        Ok(())
    }

}
