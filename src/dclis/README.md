# dclis

Command line interface for retrieving primary platform and membership ids for Destiny 2 players.

Retrieves the primary Destiny 2 membershipId and platform for specified username or steam 64 id and platform. That may a membershipId on a platform different that the one specified, depending on the cross save status of the account. It will return the primary membershipId that all data will be associate with.d

In cases of players who have enabled cross save / play, the primary id and platform will be returned.

## USAGE
```
USAGE:
    dclis [FLAGS] --id <id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -t, --terse      
            terse output in the form of membership_id:platform . Errors are suppresed

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information for the API call


OPTIONS:
    -i, --id <id>                
            User name or steam 64 id
            
            User name (for Xbox, Playstation or Stadia) or steam 64 id : 00000000000000000 (17 digit ID) for steam.
    -p, --platform <platform>    
            Platform for specified id
            
            Platform for specified id. Valid values are: xbox, playstation, stadia or steam
```

Valid platforms are xbox, playstation, steam and stadia.

### Examples

Retrieve membership id for a player on xbox and print the url for the API call:
```
dclis --id mesh --platform xbox
```

which will output:

```
Display Name  : mesh
Membership Id : 4611686018429783292
Platform      : Xbox
```

Search for the membership id using the steam 64 id, and print out in terse format membershipid:platform_id
```
dclis --id 76561197984551459 --platform steam --terse
```

which will output:

```
4611686018429783292:Xbox
```


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*