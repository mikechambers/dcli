# dcliah

Command line tool for viewing Destiny 2 Crucible activity history and stats.

The application will display individual game results and stats, aggregate game results and stats, as well as individual weapon and medal stats. You can specify specific crucible game modes, as well as time periods to create custom reports. Private and non-private stats are separated from each other.

dcliah pulls its data from the local Destiny 2 activity database store. Data can be synced using using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync) or by passing the --sync flag to dcliah.

The tool expects that the manifest has been downloaded and synced using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

[![Image of dcliah](../../images/dcliah_sm.png)](../../images/dcliah.png)

## USAGE

```
USAGE:
    dcliah [FLAGS] [OPTIONS] --name <name>

FLAGS:
    -h, --help
            Prints help information

    -s, --sync
            Sync activities for specified user

    -V, --version
            Prints version information

    -v, --verbose
            Print out additional information

OPTIONS:
    -L, --activity-limit <activity-limit>
            Limit the number of activity details that will be displayed

            Summary information will be generated based on all activities. [default: 10]
    -k, --api-key <api-key>
            API key from Bungie required for some actions.

            If specified the key will be passed to all Destiny API calls.

            You can obtain a key from https://www.bungie.net/en/Application [env:
            DESTINY_API_KEY=8eacb6527ea648fbbd8106990231c21c]
    -C, --class <character-class-selection>
            Character to retrieve data for

            Valid values include hunter, titan, warlock, last_active and all. [default: all]
    -t, --custom-time <custom-time>
            Custom start time in RFC 3339 date / time format

            Must be a valid date in the past.

            Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00

            Required when --moment is set to custom, but otherwise not applicable.
    -D, --data-dir <data-dir>
            Directory where Destiny 2 manifest and activity database files are stored. (optional)

            This will normally be downloaded using the dclim tool, and uses a system appropriate directory by default.
    -e, --end-custom-time <end-custom-time>
            Custom end time in RFC 3339 date / time format

            Must be a valid date in the past.

            Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00

            Required when --end-moment is set to custom, but otherwise not applicable.
    -E, --end-moment <end-moment>
            End moment from which to pull activities from

            Activities will be retrieved from moment to end-moment. End moment must be greater than moment

            For example, Specifying: --moment month --end-moment weekly will return all activities from a month ago up
            to the most recent weekly reset.

            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom as well as the
            following season moments launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge,
            season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy,
            season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen.

            When custom is specified, the custom start date in RFC3339 format must be specified with the --end-custom-
            time argument.

            For example: --moment custom --end-custom-time 2020-12-08T17:00:00.774187+00:00 [default: now]
    -m, --medal-count <medal-count>
            The number of medals to display details for. Gold medals will be listed first [default: 5]

    -M, --mode <mode>
            Activity mode to return stats for

            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private,
            rumble, pvp_competitive, quickplay and trials_of_osiris.

            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, rift, iron_banner_rift,
            scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -T, --moment <moment>
            Start moment from which to pull activities from

            Activities will be retrieved from moment to end-moment.

            For example, Specifying: --moment weekly will return all activities since the last weekly reset on Tuesday.

            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom as well as the
            following season moments launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge,
            season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy,
            season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen.

            When custom is specified, the custom start date in RFC3339 format must be specified with the --custom-time
            argument.

            For example: --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00 [default: week]
    -n, --name <name>
            Bungie name for player

            Name must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -w, --weapon-count <weapon-count>
            The number of weapons to display details for [default: 5]

    -W, --weapon-sort <weapon-sort>
            Specify weapon stats sort order

            Valid values include name, kills (default), games, kills_per_game_kills, precision_total, precision_percent,
            type, wins_percent [default: kills]
```

| ARGUMENT      | OPTIONS                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --mode        | all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, rift, iron_banner_rift, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine                                                                                                                                                     |
| --moment      | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder |
| --end-moment  | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder |
| --weapon-sort | name, kills (default), games, kills_per_game_kills kills_per_game_total, precision_total, precision_percent, type                                                                                                                                                                                                                                                                                                                                                                                                                                             |

Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Activity data store can be created and synced seperately using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync).

**NOTE** : Currently, due to a [bug](https://github.com/Bungie-net/api/issues/1386) in the Destiny 2 API, you will only get results for private matches when specifying _all_private_. The other options are still included in case the bug is fixed.

### Examples

#### Retrieve all activities for past month for the most recently played character

```
$ dcliah --name mesh#3230 --moment month
```

#### Retrieve all Trials of Osiris stats for the Titan since the weekend reset

```
$ dcliah --name mesh#3230 --moment weekend --class titan --mode trials_of_osiris
```

#### Retrieve all stats for Season of Arrivals

```
$ dcliah --name mesh#3230 --moment season_of_arrivals --end-moment season_of_the_hunt
```

#### Retrieve all stats for all time for all characters

```
$ dcliah --name mesh#3230 --moment all_time --class all
```

#### Use dclitime to track all stats from a specific time (on unix based systems)

```
$ export SESSION_START=$(dclitime)
$ dcliah --name mesh#3230 --moment custom --custom-time $SESSION_START
```

#### View all time stats for Hand Canons

```
& dcliah --name mesh#3230 --mode all_pvp --moment all_time --weapon-count 10000 | grep "Hand Cannon"
```

or in Windows Powershell

```
& dcliah.exe --name mesh#3230 --mode all_pvp --moment all_time --weapon-count 10000 | Select-String "Hand Cannon"
```

## Questions, Feature Requests, Feedback

If you have any questions, feature requests, need help, are running into issues, or just want to chat, join the [dcli Discord server](https://discord.gg/2Y8bV2Mq3p).

You can also log bugs and features requests on the [issues page](https://github.com/mikechambers/dcli/issues).

### Environment Variables

#### DCLI_FIX_DATA

If the `DCLI_FIX_DATA` environment variable is set to `TRUE` then when corrupt or missing data is returned from the Bungie API, and there is not a valid local version, DCLI will attempt to retrieve updated, non-corrupt data from Bungie. (This sometimes happens if a lot of people leave a game, and no player names will be returned from the server).

Setting this to true can significantly slow down sync time, especially the initial sync, and in general, is meant to be used when using DCLI to create datastores for larger applications.

#### RUST_LOG

All dcli apps have support for log output via the [env_logger](https://docs.rs/env_logger/0.9.3/env_logger/) library. This is mostly used for development, but may be helpful when trying to debug any issues.

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the `src/` directory and run:

```
$ cargo build --release
```

which will place the compiled tools in _src/target/release_
