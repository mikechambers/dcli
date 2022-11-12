# dclistat

Command line tool for querying Destiny 2 PVP stats.

The application takes a list of stat types and returns a comma seperated list of the corresponding data for that stat and the specificed parameters.

dclistat pulls its data from the local Destiny 2 activity database store. Data can be synced using using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync) or by passing the --sync flag to dclistat.

The tool expects that the manifest has been downloaded and synced using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

## USAGE

```
USAGE:
    dclistat [FLAGS] [OPTIONS] --name <name> --stat <stat>...

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
            season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost,
            season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder.

            When custom is specified, the custom start date in RFC3339 format must be specified with the --end-custom-
            time argument.

            For example: --moment custom --end-custom-time 2020-12-08T17:00:00.774187+00:00 [default: now]
    -M, --mode <mode>
            Activity mode to return stats for

            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private,
            rumble, pvp_competitive, quickplay and trials_of_osiris.

            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_clash, private_control, private_survival, private_rumble, showdown, lockdown, iron_banner_rift,
            rift, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -T, --moment <moment>
            Start moment from which to pull activities from

            Activities will be retrieved from moment to end-moment.

            For example, Specifying: --moment weekly will return all activities since the last weekly reset on Tuesday.

            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom as well as the
            following season moments launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge,
            season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy,
            season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost,
            season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder.

            When custom is specified, the custom start date in RFC3339 format must be specified with the --custom-time
            argument.

            For example: --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00 [default: week]
    -n, --name <name>
            Bungie name for player

            Name must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -x, --stat <stat>...
            Stat to retrieve data for

            Valid values include kd, kda, efficiency, kills,
            opponents_defeated, deaths, assists, kills_avg,
            opponents_defeated_avg, deaths_avg, assists_avg, kd_max, kda_max,
            efficiency_max, kills_max, opponents_defeated_max, deaths_max,
            games, wins, losses, mercies.
```

| ARGUMENT     | OPTIONS                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --mode       | all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, rift, iron_banner_rift, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine                                                                                                                                                     |
| --moment     | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder |
| --end-moment | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen, season_of_the_haunted, season_of_the_plunder |
| --stat       | kd, kda, efficiency, kills, opponents_defeated, deaths, assists, kills_avg, opponents_defeated_avg, deaths_avg, assists_avg, kd_max, kda_max, efficiency_max, kills_max, opponents_defeated_max, deaths_max, games, wins, losses, mercies                                                                                                                                                                                                                                                                                                                     |

Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Activity data store can be created and synced seperately using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync).

**NOTE** : Currently, due to a [bug](https://github.com/Bungie-net/api/issues/1386) in the Destiny 2 API, you will only get results for private matches when specifying _all_private_. The other options are still included in case the bug is fixed.

### Examples

#### Retrieve KD for Trials of Osiris for the current weekend

```
$ dclistat --name mesh#3230 --moment weekend --mode trials_of_osiris --stat kd
```

Outputs:

```
1.39
```

#### Retrieve total kills, kills per game, and highest kills in a game for all pvp matches in Season of the Haunted

```
$ dclistat --name mesh#3230 --moment season_of_the_haunted --mode all_pvp --stat kills kills_avg kills_max
```

Outputs:

```
10858,7.19,31
```

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
