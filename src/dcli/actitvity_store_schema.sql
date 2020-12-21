        BEGIN TRANSACTION;

        /* found activities we havent synced details from yet */
        CREATE TABLE IF NOT EXISTS "main"."activity_queue" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "activity_id" TEXT NOT NULL,
            "character"	INTEGER NOT NULL,
            UNIQUE(activity_id, character),
            FOREIGN KEY (character)
               REFERENCES character (id)
               ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS  "member" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "member_id"	TEXT NOT NULL,
            "platform_id"	INTEGER NOT NULL,
            UNIQUE(member_id, platform_id)
        );
        
        CREATE TABLE IF NOT EXISTS  "character" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "character_id"	TEXT NOT NULL,
            "member"	INTEGER NOT NULL,
            UNIQUE(character_id, member),
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
        
        CREATE TABLE IF NOT EXISTS "main"."weapon_result" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,

            "reference_id"                      INTEGER NOT NULL,
            "unique_weapon_kills"               REAL NOT NULL,
            "unique_weapon_precision_kills"     REAL NOT NULL,
            "unique_weapon_kills_precision_kills"   REAL NOT NULL,
            "character_activity_stats"          INTEGER NOT NULL,
            
            UNIQUE(character_activity_stats, reference_id),

            FOREIGN KEY (character_activity_stats)
               REFERENCES character_activity_stats (id)
               ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS "main"."character_activity_stats" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "character"                 INTEGER NOT NULL,
            "activity"	                INTEGER NOT NULL,

            "assists"                   REAL NOT NULL,
            "score"                     REAL NOT NULL,
            "kills"                     REAL NOT NULL,
            "deaths"                    REAL NOT NULL,
            "average_score_per_kill"    REAL NOT NULL,
            "average_score_per_life"    REAL NOT NULL,
            "completed"                 REAL NOT NULL,
            "opponents_defeated"        REAL NOT NULL,
            "activity_duration_seconds" REAL NOT NULL,
            "standing"                  INTEGER NOT NULL,
            "team"                      REAL NOT NULL,
            "completion_reason"         REAL NOT NULL,
            "start_seconds"             REAL NOT NULL,
            "time_played_seconds"       REAL NOT NULL,
            "player_count"              REAL NOT NULL,
            "team_score"                REAL NOT NULL,
            "precision_kills"           REAL NOT NULL,
            "weapon_kills_ability"      REAL NOT NULL,
            "weapon_kills_grenade"      REAL NOT NULL,
            "weapon_kills_melee"        REAL NOT NULL,
            "weapon_kills_super"        REAL NOT NULL,

            UNIQUE(activity, character),
        
            FOREIGN KEY (activity)
               REFERENCES activity (id)
               ON DELETE CASCADE,
        
            FOREIGN KEY (character)
               REFERENCES character (id)
               ON DELETE CASCADE
        );
        COMMIT;