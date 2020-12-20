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
        
        CREATE TABLE IF NOT EXISTS "main"."character_activity_stats" (
            "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
            "character"	INTEGER NOT NULL,
            "activity"	INTEGER NOT NULL,
            UNIQUE(activity, character),
        
            FOREIGN KEY (activity)
               REFERENCES activity (id)
               ON DELETE CASCADE,
        
            FOREIGN KEY (character)
               REFERENCES character (id)
               ON DELETE CASCADE
        );
        COMMIT;