# dclics

Command line tool for retrieving Destiny 2 Crucible activity stats.

Enables control of which stats are retrieved via:

* Game Mode such as all, control, ironbanner, trialsofosiris, etc...
* Time period such as alltime, reset, day...
* Character, specify for all characters or specific character (any period other than alltime requires that the character is specified)


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
            
            Destiny 2 API character id. If not specified, data for all characters will be returned. Required when period
            is set to day, reset, week or month.
    -m, --member-id <member-id>          
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
        --mode <mode>                    
            Crucible mode to return stats for
            
            Valid values are all (default), control, clash, mayhem, ironbanner, private, trialsofnine, rumble, comp,
            quickplay and trialsofosiris.
    -o, --output <output>                
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n).
        --period <period>                
            Time range to pull stats from
            
            Valid values include day (last day), reset (since reset), week (last week), month (last month), alltime (default).
    -p, --platform <platform>            
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
```

Valid platform values are:
* xbox
* playstation
* steam
* stadia

Valid game modes are:
* all (default)
* control
* clash
* mayhem
* ironbanner
* private
* trialsofnine
* rumble
* comp
* quickplay
* trialsofosiris

Valid time period values are:
* day (last day)
* reset (since reset)
* week (last week)
* month (last month)
* alltime (default)

member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).

### Examples

#### Print all stats for last week

```
$ dclics --member-id 4611686018429783292 --platform xbox --mode all --character-id 2305843009264966985 --period week
```

outputs:

```
Destiny 2 stats for all Crucible modes for the last week
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
$ dclics --member-id 4611686018429783292 --platform xbox --mode ironbanner --period alltime
```

#### Show stats for Trials of Osiris since the last weekly reset

```
$ dclics --member-id 4611686018429783292 --platform xbox --mode trialsofosiris --character-id 2305843009264966985 --period reset
```

#### Show life time stats for comp, outputing to a tab seperated format

```
$ dclics --member-id 4611686018429783292 --platform xbox --mode comp --period alltime --output tsv
```

outputs:

```
member_id       4611686018429783292
platform        Xbox
platform_id     1
character_id    0
period_dt       2013-12-11 23:12:29.889142 UTC
period_human    for all time
mode    Competitive
mode_id 69
activities_entered      994
activities_won  528
activities_lost 466
assists 3048
kills   9697
average_kill_distance   14.373105
total_kill_distance     139376
seconds_played  646392
human_time_played       7 days 11 hours 33 minutes 12 seconds
deaths  7982
average_lifespan        80.97106
human_average_lifespan  1 minute 20 seconds
total_lifespan  646311
opponents_defeated      12745
efficiency      1.5967176
kills_deaths_ratio      1.2148584
kills_deaths_assists    1.4057881
suicides        70
precision_kills 1784
best_single_game_kills  31
```

best_single_game_kills data is only available for '--period alltime` and will return -1 for other time periods..

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