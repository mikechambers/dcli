# dclisync

Command line tool for downloading and syncing Destiny 2 Crucible activity
history to a sqlite3 database file. The app can be run as a one off sync, or run in daemon mode (_--daemon_) which will continuously sync data, with a pause between syncs (can set via _--interval_).

dclisync requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment variable. You can obtain a key from [https://www.bungie.net/en/Application](https://www.bungie.net/en/Application).

Users may be added and removed via the --add and --remove flags, and clans can be imported via the --import-group flag.

If multiple flags are specified, they will be run in the following order:
import, add, remove, sync, list

On initial sync, the tool will download all Crucible activity history and data for the specified players(s), and store it in a [sqlite3](https://www.sqlite.org/index.html) database file. On subsequent runs, it will download any new activities since the last sync.

This provides a local sqlite3 database that contains all crucible matches and individual stats for the specified player. It can be used with other dcli apps, or you can make custom queries against the database.

Depending on the number of activities, the initial sync may take a couple of minutes to run for each player. Subsequent syncs should be much faster.

The app will download and store all public and private PVP activities for all currently active characters.

The app syncs in 3 stages:

1. First, check if there are any local un-synced activities from previous syncs. If so, download their details (step 3).
2. Call the Destiny API, and get a list of all new activities since the last sync. If it is the first time the app has been synced, then retrieve all Crucible activity ids for all time for the specified character. Store the activity ids.
3. Loop through all of the activity ids that have been found, and download all of the data on each activity and store it in the database.

If an error occurs when downloading the list of activities (step 2), then the app will abort. Just rerun.

If any errors occur while downloading activity details (step 3), then that specific activity will be skipped, and saved to retry the next time there is a sync.

## USAGE

```
USAGE:
    dclisync.exe [FLAGS] [OPTIONS] --api-key <api-key> --sync <sync>...

FLAGS:
    -d, --daemon
            Run dclisync in daemon mode. dclisync will run continuously with a pause (specified by --interval) between syncs

    -h, --help
            Prints help information

    -l, --list
            List all Bungie names which are flagged to be synced

    -V, --version
            Prints version information

    -v, --verbose
            Print out additional information

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
    -I, --interval <interval>
            Interval in seconds between player syncs when running in daemon mode

    -r, --remove <remove>...
            Remove specified player(s) from having their activities synced.

            Note, player data will still be contained in the database, but no new activities will be synced for the
            player(s)

            Name(s) must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -s, --sync <sync>...
            Sync player activities.

            If no arguments are provided, all players will be synced. Optionally, you can pass in one or more space
            separated Bungie names and codes. If a name has a space in it, you must the entire name in quotes.

            Name(s) must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings

            Requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment
            variable.

            You can obtain a key from https://www.bungie.net/en/Application
```

### Daemon Mode

dclisync has support for daemon mode, which will continuously sync data, with pauses in-between syncs. This is useful if you need to run dclisync as a system service, to automatically keep data in sync.

When running in daemon mode, sync progress will not be displayed, although messages on sync start and complete, as well as errors will be displayed.

By default, sync will pause for 60 seconds between each sync. This can be set via the _--interval_ flag, with a minimum value of 30 seconds.

You can stop dclisync via Ctrl-C or sending a SIGINT or SIGTERM system message. dclisync will capture these calls and try to gracefully shut down by finishing its current sync. You can force the app to exit by sending another Ctrl-C / SIGINIT / SIGTERM.

#### Run in daemon mode

Run in daemon mode, with pause of 60 seconds between syncs.

```
$ dclisync --sync --daemon --interval 60
```

### Run as a service

It is possible to run dclisync as a system service to automatically sync data.

To run as a service via systemctl on Linux based systems, you can use the following service file:

```
[Unit]
Description=dclisync service
After=multi-user.target

[Service]
Type=simple
Restart=always
User=mesh
Group=mesh
ExecStart=/usr/bin/bash -lc "dclisync --sync --daemon --interval 60"
TimeoutStopSec=60
[Install]
WantedBy=multi-user.target
```

This will launch, log and monitor _dclisync_ in the background. With _dclisync_ pausing 60 seconds between syncs.

Note that the service does not need to run as admin, but it does need to run as a user which has access to any required environment variables (such as _DESTINY_API_KEY_). The user's path must also include the _dclisync_ executable, or you must provide an absolute path above. Finally, the user that it runs under will also impact the path where the database will be created.

Also note that we set _TimeoutStopSec_ to 60 seconds. When stopping the service, dclisync will try to finish it's existing sync and gracefully shutdown. This tells systemctl to wait 60 seconds for dclisync to shutdown before forcefully killing it.

When adding and syncing new users it is recommended that you:

1. Shut down the dclisync service
2. Add the new players
3. Do an initial sync for all of the new players
4. Restart the service

The initial sync for each player can take an extended amount of this, and manually running it makes it easier to debug any issues.

As a general rule, you should stop the service anytime you manually run or update _dclisync_.

More info:

-   [How To Use Systemctl to Manage Systemd Services and Units](https://www.digitalocean.com/community/tutorials/how-to-use-systemctl-to-manage-systemd-services-and-units)
-   [How to create a Systemd service in Linux](https://www.shubhamdipt.com/blog/how-to-create-a-systemd-service-in-linux/)
-   [systemctl man page](https://www.freedesktop.org/software/systemd/man/systemctl.html)

### Examples

#### Add players to sync

```
$ dclisync --add mesh#3230 BUNGIENAME#3450 --api-key YOUR_DESTINY_API_KEY
```

API key may also be specified via the DESTINY_API_KEY environment variable.

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

API key may also be specified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

#### Sync activities for all players

```
$ dclisync --sync --api-key YOUR_DESTINY_API_KEY
```

API key may also be specified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

#### Sync activities for specific players

```
$ dclisync --sync mesh#3230 --api-key YOUR_DESTINY_API_KEY
```

API key may also be specified via the DESTINY_API_KEY environment variable.

You can obtain an api key from https://www.bungie.net/en/Application

### Environment Variables

#### DESTINY_API_KEY

dclisync requires that a Bungie API key is specified via the --api-key KEY flag, or DESTINY_API_KEY environment variable. You can obtain a key from [https://www.bungie.net/en/Application](https://www.bungie.net/en/Application).

#### DCLI_FIX_DATA

If the `DCLI_FIX_DATA` environment variable is set to `TRUE` then when corrupt or missing data is returned from the Bungie API, and there is not a valid local version, DCLI will attempt to retrieve updated, non-corrupt data from Bungie. (This sometimes happens if a lot of people leave a game, and no player names will be returned from the server).

Setting this to true can significantly slow down sync time, especially the initial sync, and in general, is meant to be used when using DCLI to create datastores for larger applications.

#### RUST_LOG

All dcli apps have support for log output via the [env_logger](https://docs.rs/env_logger/0.9.3/env_logger/) library. This is mostly used for development, but may be helpful when trying to debug any issues.

## Privacy

Note, in order for dclisync to be able to sync your data, you must have the following privacy options selected on your Bungie account at [https://www.bungie.net/7/en/User/Account/Privacy](https://www.bungie.net/7/en/User/Account/Privacy)

-   Show my Destiny game Activity feed on Bungie.net

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
