# dclia

Command line interface for retrieving information on current activity for specified player character.

Information includes current activity, location, and in the case of PvP modes (Crucible and Gambit), current map.

The API updates pretty quickly, and can be used to see check the activity and / or map while you are loading in.


## USAGE
```
USAGE:
    dclia [FLAGS] --manifest-path <manifest-path> --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -t, --terse      
            Terse output. Errors are suppresed

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information


OPTIONS:
        --manifest-path <manifest-path>    
            Local path for the Destiny 2 manifest database file

    -m, --member-id <member-id>            
            Destiny 2 API member id
            
            Destiny 2 API member id. This is not the user name, but the member id retrieved from the Destiny API.
    -p, --platform <platform>              
            Platform for specified id
            
            Platform for specified member id. Valid values are: xbox, playstation, stadia or steam
```

Valid platforms are xbox, playstation, steam and stadia.

### Examples

Check for current activity. Note, member-id can be retrieved from [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).

```
dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox
```

outputs:

```
Playing Deep Stone Crypt Raid on Castalia Macula, Europa
```

Check for current activity, with terse / compact output:

```
dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox --terse
```

outputs:

```
Mode:Strike
Activity:The Inverted Spire
Place:Nessus
Destination:Arcadian Valley
Description:End the Red Legion expedition that's ripped open the planet's surface.
```


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*