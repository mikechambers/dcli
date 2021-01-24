# dcliah

Command line tool for viewing Destiny 2 Crucible activity history and stats.

The application will display individual game results and stats, aggregate game results and stats, as well as individual weapon stats. You can specify specific crucible game modes, as well as time periods to create custom reports. Private and non-private stats are seperated from each other.

dcliah pulls its data from the local Destiny 2 activity database store. By default, dcliah will create and update this file with the latest activity data, but it can also be seperately managed using [dclias](https://github.com/mikechambers/dcli/tree/main/src/dclias).

The first time the database downloads activity data may take a couple of minutes (depending on bandwidth and number of activities). However, subsequent syncs should be very quick.

It supports storing and tracking stats for multiple players and characters.

If you want to sync the database seperately via dclias, you can pass the `-no-sync` flag to dcliah and it will not update the activity store.

The tool expects that the manifest has been downloaded and synced using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

[![Image of dcliah](../../images/dcliah_sm.png)](../../images/dcliah.png)

## USAGE
```
USAGE:
    dcliah [FLAGS] [OPTIONS] --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -N, --no-sync    
            Don't sync activities
            
            If flag is set, activities will not be retrieved before displaying stats. This is useful in case you are
            syncing activities in a seperate process.
    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -L, --activity-limit <activity-limit>      
            Limit the number of activity details that will be displayed
            
            Summary information will be generated based on all activities. [default: 10]
    -C, --class <character-class-selection>    
            Character to retrieve data for
            
            Valid values include hunter, titan, warlock, last_active and all. [default: last_active]
    -t, --custom-time <custom-time>            
            Custom start time in RFC 3339 date / time format
            
            Must be a valid date in the past.
            
            Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00
            
            Required when --moment is set to custom, but otherwise not applicable.
    -D, --data-dir <data-dir>                  
            Directory where Destiny 2 manifest and activity database files are stored. (optional)
            
            This will normally be downloaded using the dclim and dclias tools, and uses a system appropriate directory
            by default.
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
            season_of_arrivals, season_of_the_hunt.
            
            When custom is specified, the custom start date in RFC3339 format must be specified with the --end-custom-
            time argument.
            
            For example: --moment custom --end-custom-time 2020-12-08T17:00:00.774187+00:00 [default: now]
    -m, --member-id <member-id>                
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -M, --mode <mode>                          
            Activity mode to return stats for
            
            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private,
            rumble, pvp_competitive, quickplay and trials_of_osiris.
            
            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched,
            scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -T, --moment <moment>                      
            Start moment from which to pull activities from
            
            Activities will be retrieved from moment to end-moment.
            
            For example, Specifying: --moment weekly will return all activities since the last weekly reset on Tuesday.
            
            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom as well as the
            following season moments launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge,
            season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy,
            season_of_arrivals, season_of_the_hunt.
            
            When custom is specified, the custom start date in RFC3339 format must be specified with the --custom-time
            argument.
            
            For example: --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00 [default: week]
    -p, --platform <platform>                  
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
    -w, --weapon-count <weapon-count>          
            The number of weapons to display details for [default: 5]

    -W, --weapon-sort <weapon-sort>            
            Specify weapon stats sort order
            
            Valid values include name, kills (default), games, kills_per_game_kills, kills_per_game_total,
            precision_total, precision_percent, type [default: kills]
```


| ARGUMENT | OPTIONS |
|---|---|
| --platform | xbox, playstation, stadia, steam |
| --mode | all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine |
| --moment | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt |
| --end-moment | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt |
| --weapon-sort | name, kills (default), games, kills_per_game_kills kills_per_game_total, precision_total, precision_percent, type |

member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   
   
Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Activity data store can be created and synced seperately using [dclias](https://github.com/mikechambers/dcli/tree/main/src/dclias).

**NOTE** : Currently, due to a [bug](https://github.com/Bungie-net/api/issues/1386) in the Destiny 2 API, you will only get results for private matches when specifying *all_private*. The other options are still included in case the bug is fixed. If viewing private match stats is important to you, please leave a comment [here](https://github.com/mikechambers/dcli/issues/10).

### Examples

#### Retrieve all activities for past month for the most recently played character

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment month
```

#### Retrieve all Trials of Osiris stats for the Titan since the weekend reset

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment weekend --class titan --mode trials_of_osiris
```

#### Retrieve all stats for Season of Arrivals

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment season_of_arrivals --end-moment season_of_the_hunt
```

#### Retrieve all stats for all time for all characters

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment all_time --class all
```

#### Use dclitime to track all stats from a specific time (on unix based systems)

```
$ export SESSION_START=$(dclitime)
$ dcliah --member-id 4611686018429783292 --platform xbox --moment custom --custom-time $SESSION_START
```

#### View all time stats for Hand Canons
```
& dcliah --member-id $MEMBER_ID --platform $PLATFORM --mode all_pvp --moment all_time --weapon-count 10000 | grep "Hand Cannon"
```

or in Windows Powershell

```
& dcliah.exe --member-id $env:MEMBER_ID --platform $env:PLATFORM --mode all_pvp --moment all_time --weapon-count 10000 | Select-String "Hand Cannon"
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

which will place the compiled tools in *src/target/release*
