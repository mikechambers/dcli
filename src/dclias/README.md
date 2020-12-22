# dclias

Command line tool for downloading and syncing Destiny 2 Crucible activity history.

On initial run, the tool will download all Crucible activity history and data for the specified character, and store it in a [sqlite3](https://www.sqlite.org/index.html) database file. On subseqent runs, it will download any new activities since the last sync.

This provides a local sqlite3 database that contains all crucible matches and individual stats for the specified player. It can be used with other dcli apps, or you can make custom queries against the database.

The app has support for storing data across multiple players and characters.

The app syncs in 3 stages:

1. First, check if there are any local unsynced activities from previous syncs. If so, download their details (step 3).
2. Call the Destiny API, and get a list of all new activities since the last sync. If it is the first time the app been synced, then retrieve all Crucible activity ids for all time for the specified character. Store the activity ids.
3. Loop through all of the activity ids that have been found, and download all of the data on each activity and store it in the database.

If an error occurs when downloading the list of activities (step 2), then the app will abort. Just rerun.

If any errors occur while downloading activity details (step 3), then that specific activity will be skipped, and saved to retry the next time there is a sync.

Depending on the number of activities, the initial sync can take a couple of minutes. Subsequent synces should be much faster.

The tool stores match data for the specified character. It does not store match results for the other players in the match.



## USAGE
```
USAGE:
    dclias [FLAGS] [OPTIONS] --character-id <character-id> --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -c, --character-id <character-id>    
            Destiny 2 API character id
            
            Destiny 2 API character id for the character to retrieve activities for.
    -D, --data-dir <data-dir>            
            Directory where activity sqlite3 database will be stored. (optional)
            
            By default data will be loaded from and stored in the appropriate system local storage directory. Data will
            be stored in a sqlite3 database file named dcli.sqlite3
    -m, --member-id <member-id>          
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -O, --output-format <output>         
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -p, --platform <platform>            
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
```


| ARGUMENT | OPTIONS |
|---|---|
| --platform | xbox, playstation, stadia, steam |



member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   
character-id can be retrieved with [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic).   


### Examples

#### Download and store all Crucible activity history

```
$ dclias --character-id 2305843009264966985 -m --member-id 4611686018429783292 --platform xbox
```

Outputs:

```
Checking for new activities.
This may take a moment depending on the number of activities.
[............................]
6778 new activities found.
Retrieving details for 6778 activities.
This may take a few minutes depending on the number of activities.
Each dot represents 50 activities
[........................................................................................................................................]
Sync complete. Database stored at:
/home/mesh/.local/share/dcli/dcli.sqlite3
```

#### Query the database for most kills in a single game

```
$ sqlite3 '/home/mesh/.local/share/dcli/dcli.sqlite3' 'select max(kills) as kills from character_activity_stats'
```

Outputs:

```
33.0
```

This assumes sqlite3 is installed on the system.

## Questions, Feature Requests, Feedback

If you have any questions, feature requests, need help, are running into issues, or just want to chat, join the [dcli Discord server](https://discord.gg/2Y8bV2Mq3p).

You can also log bugs and features requests on the [issues page](https://github.com/mikechambers/dcli/issues).


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the `src/` directory and run:

```
$ cargo build --release
```

which will place the compiled tools in *src/target/release*
