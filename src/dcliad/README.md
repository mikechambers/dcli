# dcliad

Command line tool for retrieving and viewing Destiny 2 Crucible activity / match details.

By default the details on the last activity will be displayed, with options (`--mode`) to specify the mode from which to retrieve the last activity.

You can also specify the specific activity via the `--activity-index` argument. The index can be retrieved from dcliah.

By default, the app will display summary data for the match, including each player and an overview of weapon usage. By passing in the `--details` flag, per user weapon usage and stats will be displayed.

dcliad pulls its data from the local Destiny 2 activity database store. By default, dcliad will create and update this file with the latest activity data, but it can also be seperately managed using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync).

The first time the database downloads activity data may take a couple of minutes (depending on bandwidth and number of activities). However, subsequent syncs should be very quick.

It supports storing and tracking stats for multiple players and characters.

If you want to sync the database seperately via dclisync, you can pass the `--no-sync` flag to dcliad and it will not update the activity store.

The tool expects that the manifest has been downloaded and synced using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

[![Image of dcliah](../../images/dcliad_sm.png)](../../images/dcliad.png)

## USAGE
```
USAGE:
    dcliad [FLAGS] [OPTIONS] --name <name>

FLAGS:
    -d, --details    
            Display extended activity details
            
            If flag is set, additional information will be displayed, including per user weapon stats.
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
    -a, --activity-index <activity-index>      
            The index of the activity to display data about
            
            By default, the last activity will be displayed. The index can be retrieved from other dcli apps, such as
            dcliah, or directly from the sqlite datastore.
    -C, --class <character-class-selection>    
            Character class to retrieve data for
            
            Valid values include hunter, titan, warlock, last_active and all. [default: last_active]
    -D, --data-dir <data-dir>                  
            Directory where Destiny 2 manifest and activity database files are stored. (optional)
            
            This will normally be downloaded using the dclim tool, and uses a system appropriate directory by default.
    -M, --mode <mode>                          
            Activity mode from which to return last activity
            
            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private,
            rumble, pvp_competitive, quickplay and trials_of_osiris.
            
            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched,
            scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -n, --name <name>                          
            Bungie name for player
            
            Name must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -w, --weapon-count <weapon-count>          
            The number of weapons to display details for [default: 5]
```


| ARGUMENT | OPTIONS |
|---|---|
| --mode | all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine |
   
Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

Activity data store can be created and synced seperately using [dclisync](https://github.com/mikechambers/dcli/tree/main/src/dclisync).

**NOTE** : Currently, due to a [bug](https://github.com/Bungie-net/api/issues/1386) in the Destiny 2 API, you will only get results for private matches when specifying *all_private*. The other options are still included in case the bug is fixed. If viewing private match stats is important to you, please leave a comment [here](https://github.com/mikechambers/dcli/issues/10).

### Examples

#### View details for last activity played

```
$ dcliad --name mesh#3230
```

#### View details for last Iron Banner match played on hunter class

```
$ dcliad --name mesh#3230 --mode iron_banner --class hunter
```

#### View details for last activity played displaying extended details 

```
$ dcliad --name mesh#3230 --details
```

#### View details for a specific activity via its index (retrieved from dcliah)

```
$ dcliad --name mesh#3230 --activity-index 7329
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
