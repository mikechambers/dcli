# dcliah

Command line tool for viewing Destiny 2 Crucible activity history and stats.

The application will display individual game results and stats, aggregate game results and stats, as well as individual weapon stats. You can specify specific crucible game modes, as well as time periods to create custom reports.

dcliah pulls its data from the local Destiny 2 activity database store. By default, dcliah will create and update this file with the latest activity data, but it can also be seperately managed using dclias (which is an app specifically for syncing remote data locally).

The first time the database is synced with activities may take a couple of minutes (depending on bandwidth and number of activities). However, subsequent synced should be very quick.

If you want to sync the database seperately, via dclias, you can pass the '--no-sync' flag to dcliah, and it will not update the activity store.


The tool expects that the manifest has been downloaded and synced to the default location using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

![Image of dcliah](../../images/dcliah.png | width=100)

## USAGE
```
USAGE:
    dcliah [FLAGS] [OPTIONS] --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -N, --no-sync    
            Don't sync activities.
            
            If flag is set, activities will not be retrieved before displaying stats. This is useful in case you are
            syncing activities in a seperate process.
    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -L, --activity-limit <activity-limit>      
            Limit the number of activity details that will be displayed.
            
            Summary information will be generated based on all activities. [default: 10]
    -C, --class <character-class-selection>    
            Character to retrieve data for.
            
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
    -m, --member-id <member-id>                
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -M, --mode <mode>                          
            Activity mode to return stats for
            
            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, private, rumble,
            pvp_competitive, quickplay and trials_of_osiris.
            
            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_matches_clash, private_matches_control, private_matches_survival, private_matches_rumble, showdown,
            lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -T, --moment <moment>                      
            Start moment from which to pull activities from
            
            Activities will be retrieved from moment to the current time.
            
            For example, Specifying: --moment weekly will return all activities since the last weekly reset on Tuesday.
            
            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom.
            
            When custom is specified, the custom start date in RFC3339 format must be specified with the --custom-time
            argument.
            
            For example: --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00 [default: week]
    -p, --platform <platform>                  
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
    -w, --weapon-count <weapon-count>          
            The number of weapons to display details for [default: 5]
```


| ARGUMENT | OPTIONS |
|---|---|
| --platform | xbox, playstation, stadia, steam |
| --mode | all_pvp (default), control, clash, elimination, mayhem, iron_banner, private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_matches_clash, private_matches_control, private_matches_survival, private_matches_rumble, showdown, lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine |
| --moment | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom |


member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   
   
Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Activity data store can be created and synced seperately using [dclias](https://github.com/mikechambers/dcli/tree/main/src/dclias).

### Examples

#### Retrieve all activities for past month for the most recently played character

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment month
```

#### Retrieve all Trials of Osiris stats for the Titan since the weekend reset

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment weekend --class titan --mode trials_of_osiris
```

#### Retrieve all all stats stats for for all time for all characters

```
$ dcliah --member-id 4611686018429783292 --platform xbox --moment all_time --class all
```

#### Use dclitime to track all stats from a specific time

```
$ export SESSION_START=$(dclitime)
$ dcliah --member-id 4611686018429783292 --platform xbox --moment custom --custom-time $SESSION_START
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
