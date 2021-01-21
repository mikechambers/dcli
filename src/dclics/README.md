# dclics

Command line tool for retrieving historic Destiny 2 Crucible activity stats.

**dclics has been deprecated and is being replaced by [dcliah](https://github.com/mikechambers/dcli/tree/main/src/dcliah)** The last release that includes dclics is [v0.3.71](https://github.com/mikechambers/dcli/releases/tag/v0.3.71).

Enables control of which stats are retrieved via:

* Game Mode such as all, control, iron_banner, trials_of_osiris, etc...
* Time period such as alltime, sincereset, lastmonth...
* Character, specify for all characters or specific character (any period other than alltime requires that the character is specified)

Retrieves stats based on the period specified, up to, but excluding the current day.

## USAGE
```
USAGE:
    dclics [FLAGS] [OPTIONS] --member-id <member-id> --platform <platform>

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
            
            Destiny 2 API character id. If not specified, data for all characters will be returned.
            
            Required unless moment is set to all_time
    -m, --member-id <member-id>          
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -M, --mode <mode>                    
            Activity mode to return stats for
            
            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble,
            pvp_competitive, quickplay and trials_of_osiris.
            
            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_clash, private_control, private_survival, private_rumble, showdown,
            lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -T, --moment <moment>                
            Time range to pull stats from
            
            Valid values include day (last day), daily (since last daily reset), week (last week), weekly (since last
            weekly reset on Tuesday), month (last month), weekend (since last Friday reset) and all_time.
            
            All ranges are up to, but not including current day, and thus some values may not return data depending on
            time of day. [default: all_time]
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
| --platform | xbox, playstation, stadia or steam |
| --mode | all_pvp (default), control, clash, elimination, mayhem, iron_banner, all_private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_clash, private_control, private_survival, private_rumble, showdown, lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine |
| --moment | day (last day), daily (since last daily reset), week (last week), weekly (since last weekly reset on Tuesday), month (last month), weekend (since last Friday reset) and all_time |


member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   
character-id can be retrieved with [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic).   

### Examples

#### Print all stats for last week

```
$ dclics --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --mode all_pvp --moment week`
```

outputs:

```
Destiny 2 stats for all Crucible modes for the last the last week
========================================================
Time played is 3 hours 11 minutes 6 seconds
13 wins and 12 losses for a 52.00% win rate


            K/D         KD/A        EFFICIENCY  KILLS       ASSISTS     DEFEATS     DEATHS      SUICIDES    
------------------------------------------------------------------------------------------------------------
  PER GAME                                      12.16       5.12        17.28       6.36        0.04        
     TOTAL  1.91        2.31        2.72        304         128         432         159         1           


You have had an average life span of 1 minute 11 seconds with an average kill distance of 16.28 meters. 32.89% of your kills were precision kills.
```

#### Show life time stats for Iron Banner across all characters

```
$ dclics --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --mode iron_banner
```

#### Show life time stats for comp, outputing to a tab seperated format

```
$ dclics --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --mode pvp_competitive --moment all_time --output-format tsv
```

outputs:

```
member_id	4611686018429783292
platform	Xbox
platform_id	1
character_id	2305843009264966985
start_moment_dt	2017-09-06 17:00:00 UTC
end_moment_dt	2020-12-13 19:02:50.248414173 UTC
moment_human	all time
mode	PvP Competitive
mode_id	69
activities_entered	990
activities_won	526
activities_lost	464
assists	3036
kills	9671
average_kill_distance	14.37173
total_kill_distance	138989
seconds_played	643375
human_time_played	7 days 10 hours 42 minutes 55 seconds
deaths	7955
average_lifespan	80.86664
human_average_lifespan	1 minute 20 seconds
total_lifespan	643294.1
opponents_defeated	12707
efficiency	1.5973601
kills_deaths_ratio	1.2157134
kills_deaths_assists	1.4065368
suicides	70
precision_kills	1780
best_single_game_kills	31
```

best_single_game_kills data is only available for '--period all_time` and will return -1 for other time periods.

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
