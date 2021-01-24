BEGIN TRANSACTION;

DROP TABLE IF EXISTS "modes";
DROP TABLE IF EXISTS "team_result";
DROP TABLE IF EXISTS "weapon_result";
DROP TABLE IF EXISTS "medal_result";
DROP TABLE IF EXISTS "activity_queue";
DROP TABLE IF EXISTS "character_activity_stats";
DROP TABLE IF EXISTS "activity";
DROP TABLE IF EXISTS "version";
DROP TABLE IF EXISTS "character";
DROP TABLE IF EXISTS "member";

DROP INDEX IF EXISTS "modes_activity_index";
DROP INDEX IF EXISTS "character_activity_stats_char_index";
DROP INDEX IF EXISTS "activity_period_index";

CREATE TABLE IF NOT EXISTS "main"."version" (
    "version"   INTEGER NOT NULL UNIQUE
);

INSERT INTO "main"."version"("version") VALUES (5);

CREATE TABLE IF NOT EXISTS "main"."activity_queue" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "activity_id" INTEGER NOT NULL,
    "character"	INTEGER NOT NULL,
    UNIQUE("activity_id", "character"),
    FOREIGN KEY ("character")
        REFERENCES character ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  "member" (
    "id"            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "member_id"	    TEXT NOT NULL,
    "platform_id"	INTEGER NOT NULL,
    "display_name"  TEXT NOT NULL,
    UNIQUE("member_id")
);

CREATE TABLE IF NOT EXISTS  "character" (
    "id"	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "character_id"	TEXT NOT NULL,
    "member"	    INTEGER NOT NULL,
    "class"         INTEGER NOT NULL,

    UNIQUE("character_id", "member"),
    FOREIGN KEY ("member")
        REFERENCES "member" ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "main"."activity" (
    "id"	        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "activity_id"	INTEGER UNIQUE NOT NULL,
    "period"        TEXT NOT NULL,
    "mode"          INTEGER NOT NULL,
    "platform"      INTEGER NOT NULL,
    "director_activity_hash" INTEGER NOT NULL,
    "reference_id"  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS "main"."modes" (
    "id"	    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "mode"	    INTEGER NOT NULL,
    "activity"  INTEGER NOT NULL,
    UNIQUE("mode", "activity"),

    FOREIGN KEY ("activity")
        REFERENCES "activity" ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "main"."team_result" (
    "id"	    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "team_id"   INTEGER NOT NULL,
    "activity"  INTEGER NOT NULL,
    "score"     INTEGER NOT NULL,
    "standing"  INTEGER NOT NULL,

    UNIQUE("team_id", "activity"),

    FOREIGN KEY ("activity")
        REFERENCES "activity" ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "main"."weapon_result" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,

    "reference_id"                      INTEGER NOT NULL,
    "kills"               INTEGER NOT NULL,
    "precision_kills"     INTEGER NOT NULL,
    "kills_precision_kills_ratio"   REAL NOT NULL,
    "character_activity_stats"          INTEGER NOT NULL,
    
    UNIQUE("character_activity_stats", "reference_id"),

    FOREIGN KEY ("character_activity_stats")
        REFERENCES "character_activity_stats" ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "main"."medal_result" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "reference_id"                      INTEGER NOT NULL,
    "count"                             INTEGER NOT NULL,
    "character_activity_stats"          INTEGER NOT NULL,
    
    UNIQUE("character_activity_stats", "reference_id"),

    FOREIGN KEY ("character_activity_stats")
        REFERENCES "character_activity_stats" ("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "main"."character_activity_stats" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "character"                 INTEGER NOT NULL,
    "activity"	                INTEGER NOT NULL,

    "assists"                   INTEGER NOT NULL,
    "score"                     INTEGER NOT NULL,
    "kills"                     INTEGER NOT NULL,
    "deaths"                    INTEGER NOT NULL,
    "average_score_per_kill"    REAL NOT NULL,
    "average_score_per_life"    REAL NOT NULL,
    "completed"                 INTEGER NOT NULL,
    "opponents_defeated"        INTEGER NOT NULL,
    "activity_duration_seconds" INTEGER NOT NULL,
    "standing"                  INTEGER NOT NULL,
    "team"                      INTEGER NOT NULL,
    "completion_reason"         INTEGER NOT NULL,
    "start_seconds"             INTEGER NOT NULL,
    "time_played_seconds"       INTEGER NOT NULL,
    "player_count"              INTEGER NOT NULL,
    "team_score"                INTEGER NOT NULL,
    "precision_kills"           INTEGER NOT NULL,
    "weapon_kills_ability"      INTEGER NOT NULL,
    "weapon_kills_grenade"      INTEGER NOT NULL,
    "weapon_kills_melee"        INTEGER NOT NULL,
    "weapon_kills_super"        INTEGER NOT NULL,
    "all_medals_earned"         INTEGER NOT NULL,
    "light_level"               INTEGER NOT NULL,

    UNIQUE("activity", "character"),

    FOREIGN KEY ("activity")
        REFERENCES "activity" ("id")
        ON DELETE CASCADE,

    FOREIGN KEY ("character")
        REFERENCES "character" ("id")
        ON DELETE CASCADE
);

CREATE INDEX modes_activity_index ON modes (activity);
CREATE INDEX character_activity_stats_char_index ON character_activity_stats (character);
CREATE INDEX activity_period_index ON activity (period);

COMMIT;