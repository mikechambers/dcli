# dclia

Command line tool for retrieving current Destiny 2 activity status for players.

Information includes current activity, location, and in the case of PvP modes (Crucible and Gambit), current map.

The API updates pretty quickly, and can be used to see check the activity and / or map while you are loading in.

The too expects that the manifest has been downloaded and synced to the default location using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim). You can specify a custom path to the manifest using the --data-dir argument.


## USAGE
```
USAGE:
    dclia [FLAGS] [OPTIONS] --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -D, --data-dir <data-dir>       
            Directory where Destiny 2 manifest database file is stored. (optional)
            
            This will normally be downloaded using the dclim tool, and stored in a file named manifest.sqlite3 (in the
            manifest directory specified when running dclim).
    -m, --member-id <member-id>     
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
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


Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

### Examples

#### Check for current activity.


```
$ dclia --member-id 4611686018429783292 --platform xbox
```

outputs:

```
Playing Deep Stone Crypt Raid on Castalia Macula, Europa
```

#### Check for current activity with tab seperated output:

```
$ dclia --member-id 4611686018429783292 --platform xbox --output-format tsv
```

outputs:

```
in_activity	true
activity_type_name	Strike
activity_name	The Inverted Spire
place_name	Nessus
destination_name	Arcadian Valley
description	End the Red Legion expedition that's ripped open the planet's surface.
human_status	Running The Inverted Spire Strike on Nessus
is_crucible	false
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
