# dcliah

Command line tool for retrieving Destiny 2 activity history.

## USAGE
```
USAGE:
    dcliah [FLAGS] [OPTIONS] --character-id <character-id> --manifest-path <manifest-path> --member-id <member-id> --platform <platform>

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
    -t, --custom-time <custom-time>        
            Custom start time in RFC 3339 date / time format
            
            Must be a valid date in the past.
            
            Example RFC 3339 format: 2020-12-08T17:00:00.774187+00:00
            
            Required when start-moment is set to custom, but otherwise not applicable.
    -L, --limit <display-limit>            
            Limit the number of activity details that will be displayed.
            
            Summary information will be generated based on all activities. Ignored if --output-format is tsv. [default: 10]
    -P, --manifest-path <manifest-path>    
            Local path for Destiny 2 manifest database file

    -m, --member-id <member-id>            
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -M, --mode <mode>                      
            Activity mode to return stats for
            
            Supported values are all_pvp (default), control, clash, elimination, mayhem, iron_banner, private,
            rumble, pvp_competitive, quickplay and trials_of_osiris.
            
            Addition values available are crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles,
            private_matches_clash, private_matches_control, private_matches_survival, private_matches_rumble, showdown,
            lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine [default: all_pvp]
    -s, --moment <moment>                  
            Start moment from which to pull activities from
            
            Activities will be retrieved from start moment to the current time. For example, Specifying: --start-moment
            weekly
            
            will return all activities since the last weekly reset on Tuesday.
            
            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom.
            
            When custom is specified, the custom start date in RFC3339 format must be specified with the --custom-time
            argument.
            
            For example: --moment custom --custom-time 2020-12-08T17:00:00.774187+00:00
            
            Specifying all_time retrieves all activitiy history and may take an extended amount of time to retrieve
            depending on the number of activities. [default: day]
    -o, --output-format <output>                  
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value or column pairs with lines ending in a new line
            character (\n). [default: default]
    -p, --platform <platform>              
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
```


| ARGUMENT | OPTIONS |
|---|---|
| --platform | xbox, playstation, stadia, steam |
| --mode | all_pvp (default), control, clash, elimination, mayhem, iron_banner, private, rumble, pvp_competitive, quickplay and trials_of_osiris, crimsom_doubles, supremacy, survival, countdown, all_doubles, doubles private_matches_clash, private_matches_control, private_matches_survival, private_matches_rumble, showdown, lockdown, scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine |
| --moment | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time and custom |


member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   
character-id can be retrieved with [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic).   
Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

### Examples

#### Retrieve all activities for past month

```
$ dcliah --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --manifest-path /home/mesh/.config/destiny/manifest.sqlite3 --moment month
```

Output:

```
All PvP activities since November 13, 2020 (last month)
-------------------------------------------------------

ACTIVITIES
==================
Displaying details for the last 10 of 79 activities.

MAP               W / L       STREAK   KILLS    ASTS     K+A  DEATHS     K/D    KD/A     EFF
============================================================================================
...               ...            ...     ...     ...     ...     ...     ...     ...     ...

Control Quickplay
--------------------------
Exodus Blue       Win              1      13      10      23      10    1.30    1.80    2.30
The Dead Cliffs   Loss            -1       7       3      10      12    0.58    0.71    0.83
Pacifica          Loss            -2      10       9      19      11    0.91    1.32    1.73

None
--------------------------
The Fortress      Unknown          0      12       2      14     ^13    0.92    1.00    1.08

Control Quickplay
--------------------------
Pacifica          Loss            -1      18       2      20       8    2.25    2.38    2.50
Wormhaven         Win              1      11       6      17       7    1.57    2.00    2.43
The Burnout       Loss            -1      17       5      22       8    2.12    2.44    2.75
Midtown           Loss            -2       4       0       4       5    0.80    0.80    0.80
Endless Vale      Win              1      12      11      23      12    1.00    1.46    1.92
Rusted Lands      Loss            -1      14       7      21      11    1.27    1.59    1.91
--------------------------------------------------------------------------------------------
HIGHS             42-36        9W 5L      22      15      29      13    7.00    8.00    9.00
PER GAME          53.16% w             11.81    4.91   16.72    7.22    1.64    1.98    2.32
============================================================================================
MAP               W / L       STREAK   KILLS    ASTS     K+A  DEATHS     K/D    KD/A     EFF
```

#### Retrieve all Trials of Osiris Activities since the weekend reset (Friday)

```
$ dcliah --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --manifest-path /home/mesh/.config/destiny/manifest.sqlite3 --moment weekend --mode trials_of_osiris
```

#### Show all activity for past week and output to TSV format (tab seperated values)

```
$ dcliah --member-id 4611686018429783292 --character-id 2305843009264966985 --platform xbox --manifest-path /home/mesh/.config/destiny/manifest.sqlite3 --moment week --output-format tsv
```

Outputs:

```
VAR	START_TIME	2020-12-06T18:51:21.968322125+00:00
VAR	MOMENT	last week
VAR	MODE	All PvP
DATA_HEADER	MODE	MAP	DATE	RESULT	KILLS	DEATHS	ASSISTS	OPP_DEFEATED	KD	KDA	EFFICIENCY
DATA_ROW	Control Quickplay	Rusted Lands	2020-12-09T22:56:30+00:00	Loss	14	11	7	21	1.27	1.59	1.91
SUMMARY_HIGHS				0:1	14	11	7	21	1.27	1.59	1.91
SUMMARY_PER_GAME				0.00	14.00	11.00	7.00	21.00	1.27	1.59	1.91
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
