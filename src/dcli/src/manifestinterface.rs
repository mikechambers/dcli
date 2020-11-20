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
use sqlx::Connection;
use sqlx::SqliteConnection;
use std::path::PathBuf;

use std::convert::TryFrom;

/// Takes a Destiny 2 API has and converts it to a Destiny 2 manifest db index value
pub fn convert_hash_to_id(hash: u32) -> i64 {
    let mut id: i64 = hash as i64;

    if (id & (1 << (32 - 1))) != 0 {
        id = id - (1 << 32);
    }

    id
}

pub struct ManifestInterface {
    manifest_db: SqliteConnection,
}

impl ManifestInterface {
    pub async fn new(manifest_path: PathBuf, cache: bool) -> Result<ManifestInterface, Error> {
        if !manifest_path.exists() {
            return Err(Error::IoFileDoesNotExist {
                description: format!(
                    "Manifest path points to non-existent file. {}",
                    manifest_path.display()
                ),
            });
        }

        let path: String = format!("{}", manifest_path.display());
        let connection_string: String = if cache {
            "sqlite:file::memory:".to_string()
        } else {
            format!("{}?mode=ro", path)
        };

        let mut db = SqliteConnection::connect(&connection_string).await?;

        if cache {
            match sqlx::query("ATTACH DATABASE '?' as 'tmpDb'")
                .bind(path)
                .execute(&mut db)
                .await
            {
                Ok(e) => e,
                Err(e) => {
                    db.close().await?;
                    return Err(Error::from(e));
                }
            };

            //TODO: Need to impliment this to dynamically pull table names
            //"SELECT name FROM sqlite_master WHERE type='table'"
            let table_name: String = "DestinyInventoryItemDefinition".to_string();
            //todo: do we need to pass table_name twice?
            match sqlx::query("CREATE TABLE ? AS SELECT * FROM tmpDb.?")
                .bind(table_name)
                .execute(&mut db)
                .await
            {
                Ok(e) => e,
                Err(e) => {
                    db.close().await?;
                    return Err(Error::from(e));
                }
            };

            match sqlx::query("DETACH DATABASE tmpDb").execute(&mut db).await {
                Ok(e) => e,
                Err(e) => {
                    db.close().await?;
                    return Err(Error::from(e));
                }
            };
        }

        Ok(ManifestInterface { manifest_db: db })
    }

    ///closes the database connection and takes ownership of self
    pub async fn close(self) -> Result<(), Error> {
        //can call ping to see if its still open? but that throws an error if it
        //isnt, so we can just try and close
        //TODO: should we bubble the error? or just silently fail?
        self.manifest_db.close().await?;
        Ok(())
    }

    /// Searches entire manifest for id, and returns associated data for it.
    /// returns an error if more that one result found.
    //TODO: should we return a vector in case there are multiple results?
    pub async fn find(&mut self, hash: u32) -> Result<Vec<String>, Error> {
        let id = convert_hash_to_id(hash);

        let tables: Vec<String> = self.get_tables_with_id_column().await?;
        let mut out: Vec<String> = Vec::new();

        for table in tables.iter() {
            //for some reason sqlx doesnt let you bind table names
            let q = format!("SELECT json FROM {} WHERE id=?", table);

            let mut rows = sqlx::query(&q).bind(id).fetch(&mut self.manifest_db);

            use futures::TryStreamExt;
            use sqlx::Row;

            while let Some(row) = rows.try_next().await? {
                // map the row into a user-defined domain type
                let json: &str = row.try_get("json")?;

                out.push(json.to_string());
            }
        }

        Ok(out)
    }

    pub async fn get_tables_with_id_column(&mut self) -> Result<Vec<String>, Error> {
        let mut tables: Vec<String> = Vec::new();

        //select all of the tables which have an id column
        let mut rows = sqlx::query("SELECT m.name as name, p.name as id FROM sqlite_master AS m JOIN pragma_table_info(m.name) AS p WHERE p.name = 'id'")
            .fetch(&mut self.manifest_db);

        use futures::TryStreamExt;
        use sqlx::Row;

        while let Some(row) = rows.try_next().await? {
            let name: &str = row.try_get("name")?;
            tables.push(name.to_string());
        }
        Ok(tables)
    }

    pub async fn get_tables(&mut self) -> Result<Vec<String>, Error> {
        let mut tables: Vec<String> = Vec::new();

        let mut rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch(&mut self.manifest_db);

        use futures::TryStreamExt;
        use sqlx::Row;
        
        while let Some(row) = rows.try_next().await? {
            
            let name: &str = row.try_get("name")?;
           
            tables.push(name.to_string());
        }
        Ok(tables)
    }

    /*
    let row: (i64,String) = match sqlx::query_as("select * from DestinyInventoryItemDefinition")
        .fetch_one(&mut db).await {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        };
        */
}

struct ManifestRow {
    name: String,
    id: i64,
}

const _MANIFEST_TABLES: &'static [&'static str] = &[
    "DestinyEnemyRaceDefinition",
    "DestinyPlaceDefinition",
    "DestinyActivityDefinition",
    "DestinyActivityTypeDefinition",
    "DestinyClassDefinition",
    "DestinyGenderDefinition",
    "DestinyInventoryBucketDefinition",
    "DestinyRaceDefinition",
    "DestinyTalentGridDefinition",
    "DestinyUnlockDefinition",
    "DestinyMaterialRequirementSetDefinition",
    "DestinySandboxPerkDefinition",
    "DestinyStatGroupDefinition",
    "DestinyFactionDefinition",
    "DestinyVendorGroupDefinition",
    "DestinyRewardSourceDefinition",
    "DestinyItemCategoryDefinition",
    "DestinyDamageTypeDefinition",
    "DestinyActivityModeDefinition",
    "DestinyMedalTierDefinition",
    "DestinyAchievementDefinition",
    "DestinyActivityGraphDefinition",
    "DestinyBondDefinition",
    "DestinyCollectibleDefinition",
    "DestinyDestinationDefinition",
    "DestinyEquipmentSlotDefinition",
    "DestinyStatDefinition",
    "DestinyInventoryItemDefinition",
    "DestinyItemTierTypeDefinition",
    "DestinyLocationDefinition",
    "DestinyLoreDefinition",
    "DestinyMetricDefinition",
    "DestinyObjectiveDefinition",
    "DestinyPlugSetDefinition",
    "DestinyPowerCapDefinition",
    "DestinyPresentationNodeDefinition",
    "DestinyProgressionDefinition",
    "DestinyProgressionLevelRequirementDefinition",
    "DestinyRecordDefinition",
    "DestinySackRewardItemListDefinition",
    "DestinySandboxPatternDefinition",
    "DestinySeasonDefinition",
    "DestinySeasonPassDefinition",
    "DestinySocketCategoryDefinition",
    "DestinySocketTypeDefinition",
    "DestinyTraitDefinition",
    "DestinyTraitCategoryDefinition",
    "DestinyVendorDefinition",
    "DestinyMilestoneDefinition",
    "DestinyActivityModifierDefinition",
    "DestinyReportReasonCategoryDefinition",
    "DestinyArtifactDefinition",
    "DestinyBreakerTypeDefinition",
    "DestinyChecklistDefinition",
    "DestinyEnergyTypeDefinition",
    "DestinyHistoricalStatsDefinition",
];
