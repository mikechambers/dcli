# dclis

Command line interface for retrieving primary platform and membership ids for Destiny 2 players.

Retrieves the primary Destiny 2 membershipId and platform for specified username or steam 64 id and platform. That may a membershipId on a platform different that the one specified, depending on the cross save status of the account. It will return the primary membershipId that all data will be associate with.d

In cases of players who have enabled cross save / play, the primary id and platform will be returned.

The id and platform can be used as input to other [dcli](https://github.com/mikechambers/dcli) tools.

## USAGE
```
USAGE:
    dclis [FLAGS] [OPTIONS] --id <id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information for the API call


OPTIONS:
    -i, --id <id>                
            User name or steam 64 id
            
            User name (for Xbox, Playstation or Stadia) or steam 64 id for Steam / pc : 00000000000000000 (17 digit ID)
            for steam.
    -o, --output <output>        
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of columns with lines ending in a new line character (\n).
            [default: default]
    -p, --platform <platform>    
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam
```

Valid platforms are:
* xbox
* playstation
* steam
* stadia

Id is either an Xbox, or PSN gamertag, a Stadia gamertag in the form of NAME#ID, or a 17 digit, Steam 64 Steam ID.

### Examples

#### Search for member id for a player on xbox
```
dclis --id mesh --platform xbox
```

which will output:

```
Display Name   mesh
id             4611686018429783292
Platform       Xbox
Platform Id    1
```

#### Search for the membership id using the steam 64 id

```
dclis --id 76561197984551459 --platform steam
```

which will output:

```
Display Name   76561197984551459
id             4611686018429783292
Platform       Xbox
Platform Id    1
```
When searching via steam id, Display Name will be the steam id.

#### Search for member id for a player on xbox and output to a tab seperated format (tsv)

```
dclis --id mesh --platform xbox --output tsv
```
outputs:

```
mesh    4611686018429783292     Xbox    1
```



#### 

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*