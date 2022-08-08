# dclisync

Command line tool for downloading and syncing Destiny 2 Crucible activity
history to a sqlite3 database file.

Users may be added and removed via the --add and --remove flags, and clans can be imported via the --import-group flag.

If multiple flags are specified, they will be run in the following order:
import, add, remove, sync, list

Some options require that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment variable. You can obtain a key from [https://www.bungie.net/en/Application](https://www.bungie.net/en/Application)

On initial sync, the tool will download all Crucible activity history and data for the specified players(s), and store it in a [sqlite3](https://www.sqlite.org/index.html) database file. On subseqent runs, it will download any new activities since the last sync.

This provides a local sqlite3 database that contains all crucible matches and individual stats for the specified player. It can be used with other dcli apps, or you can make custom queries against the database.

Depending on the number of activities, the initial sync may take a couple of minutes to run for each player. Subsequent syncs should be much faster.

The app will download and store all public and privsate PVP activities for all currently active characters.

The app syncs in 3 stages:

1. First, check if there are any local unsynced activities from previous syncs. If so, download their details (step 3).
2. Call the Destiny API, and get a list of all new activities since the last sync. If it is the first time the app has been synced, then retrieve all Crucible activity ids for all time for the specified character. Store the activity ids.
3. Loop through all of the activity ids that have been found, and download all of the data on each activity and store it in the database.

If an error occurs when downloading the list of activities (step 2), then the app will abort. Just rerun.

If any errors occur while downloading activity details (step 3), then that specific activity will be skipped, and saved to retry the next time there is a sync.

## USAGE

```
USAGE:
    dclisync [FLAGS] [OPTIONS] --api-key <api-key> --sync <sync>...

FLAGS:
    -h, --help
            Prints help information

    -l, --list
            List all Bungie names which are flagged to be synced

    -V, --version
            Prints version information

    -v, --verbose
            Print out additional information

            Output is printed to stderr.

OPTIONS:
    -A, --add <add>...
            Add specified player(s) to have their activities synced the next time the database is synced.

            Name(s) must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings

            Requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment
            variable.

            You can obtain a key from https://www.bungie.net/en/Application
    -k, --api-key <api-key>
            API key from Bungie required for some actions.

            You can obtain a key from https://www.bungie.net/en/Application [env:
            DESTINY_API_KEY=8eacb6527ea648fbbd8106990231c21c]
    -D, --data-dir <data-dir>
            Directory where activity sqlite3 database will be stored. (optional)

            By default data will be loaded from and stored in the appropriate system local storage directory. Data will
            be stored in a sqlite3 database file named dcli.sqlite3
    -i, --import-group <import-group>
            Import all players for specified Destiny 2 Group / clan.

            You can get your groupid for your clan from the Bungie clan page: https://www.bungie.net/en/ClanV2/MyClans
            Click your clan, then copy the group id from the URL.

            Requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment
            variable.

            You can obtain a key from https://www.bungie.net/en/Application
    -r, --remove <remove>...
            Remove specified player(s) from having their acitivities synced.

            Note, player data will still be contained in the database, but no new activities will be synced for the
            player(s)

            Name(s) must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -s, --sync <sync>...
            Sync player activities.

            If no arguments are provided, all players will be synced. Optionally, you can pass in one or more space
            seperated Bungie names and codes. If a name has a space in it, you must the entire name in quotes.

            Name(s) must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings

            Requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment
            variable.

            You can obtain an api key from https://www.bungie.net/en/Application
```

### Examples

#### Add players to sync

```
$ dclisync --add mesh#3230 BUNGIENAME#3450 --api-key YOUR_DESTINY_API_KEY
```

API key may also be spcified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

#### Remove players from syncing

```
$ dclisync --remove mesh#3230 BUNGIENAME#3450
```

#### Import Clan / Group members

```
$ dclisync --import-group 4571679 --api-key YOUR_DESTINY_API_KEY
```

You can get your groupid for your clan from the Bungie clan page: https://www.bungie.net/en/ClanV2/MyClans . Click your clan, then copy the group id from the URL.

API key may also be spcified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

#### Sync activities for all players

```
$ dclisync --sync --api-key YOUR_DESTINY_API_KEY
```

API key may also be spcified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

#### Sync activities for specific players

```
$ dclisync --sync mesh#3230 --api-key YOUR_DESTINY_API_KEY
```

API key may also be spcified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

### Environment Variables

If the `DCLI_FIX_DATA` environment variable is set to `TRUE` then when corrupt or missing data is returned from the Bungie API, and there is not a valid local version, DCLI will attempt to retrieve updated, non-corrupt data from Bungie. (This sometimes happens if a lot of people leave a game, and no player names will be returned from the server).

Setting this to true can significantly slow down sync time, especially the initial sync, and in general, is meant to be used when using DCLI to create datastores for larger applications.

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

which will place the compiled tools in _src/target/release_
