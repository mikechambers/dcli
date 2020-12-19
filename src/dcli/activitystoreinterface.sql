BEGIN TRANSACTION;

/* found activities we havent synced details from yet */
CREATE TABLE IF NOT EXISTS 'main'.'activity_queue' (
	'activity_id' TEXT NOT NULL,
	'character_rowid'	INTEGER NOT NULL,
	PRIMARY KEY('character_rowid', 'activity_id'),

	FOREIGN KEY (character_rowid)
       REFERENCES character (rowid)
	   ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS 'main'.'member' (
	'id'	TEXT NOT NULL,
	'platform_id'	INTEGER NOT NULL,
	PRIMARY KEY('id', 'platform_id')
);

/* character */
CREATE TABLE IF NOT EXISTS 'main'.'character' (
	'id'	TEXT NOT NULL,
	'member_rowid'	INTEGER NOT NULL,
	PRIMARY KEY('id', 'member_rowid'),
	FOREIGN KEY (member_rowid)
       REFERENCES member (rowid)
	   ON DELETE CASCADE
);

/* activity / match (doesnt have all fields yet */
CREATE TABLE IF NOT EXISTS 'main'.'activity' (
	'id'	INTEGER UNIQUE NOT NULL,
    'period' TEXT NOT NULL,
    'mode' INTEGER NOT NULL,
    'platform' INTEGER NOT NULL,
    'director_activity_hash' INTEGER NOT NULL
    
	PRIMARY KEY('id')
);

CREATE TABLE IF NOT EXISTS 'main'.'character_activity_stats' (
	'character_rowid'	INTEGER NOT NULL,

	/* we use id and not rowid since we shouldnt have dupes */
	'activity_id'	INTEGER NOT NULL,

	FOREIGN KEY (activity_id)
       REFERENCES activity (activity_id)
	   ON DELETE CASCADE,

	FOREIGN KEY (character_rowid)
       REFERENCES character (rowid)
	   ON DELETE CASCADE,

	PRIMARY KEY('character_rowid','activity_id')
);
COMMIT;