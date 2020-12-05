# dclia

Command line tool for retrieving information on current activity for specified player character.

Information includes current activity, location, and in the case of PvP modes (Crucible and Gambit), current map.

The API updates pretty quickly, and can be used to see check the activity and / or map while you are loading in.


## USAGE
```
USAGE:
    dclia [FLAGS] [OPTIONS] --manifest-path <manifest-path> --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
        --manifest-path <manifest-path>    
            Local path for the Destiny 2 manifest database file

    -m, --member-id <member-id>            
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -o, --output <output>                  
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -p, --platform <platform>              
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
```

Valid platforms are:
* xbox
* playstation
* steam
* stadia

member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).

Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

### Examples

#### Check for current activity.


```
$ dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox
```

outputs:

```
Playing Deep Stone Crypt Raid on Castalia Macula, Europa
```

#### Check for current activity with tab seperated output:

```
$ dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox --output tsv
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