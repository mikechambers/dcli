# dclis

Command line interface for retrieving primary platform and membership ids for Destiny 2 players.

Note, in cases of players who have enabled cross save / play, the primary id and platform will be returned.

## USAGE
```
USAGE:
    dclis [FLAGS] --id <id> --platform <platform>

FLAGS:
    -c, --compact    Compact output in the form of membership_id:platform_id
    -h, --help       Prints help information
    -u, --url        Print out the url used for the API call
    -V, --version    Prints version information

OPTIONS:
    -i, --id <id>                User name or steam 64 id
    -p, --platform <platform>    Platform for specified id
```

Valid platforms are xbox, playstation, steam and stadia.

### Example

Retrieve membership id for a player on xbox and print the url for the API call:
```
dclis --id mesh --platform xbox --url
```

Search for the membership id using the steam 64 id, and print out in compact format membershipid:platform_id
```
dclis --id 76561197984551459 --platform steam --compact
```
Note, platform ids, can be found [here](https://bungie-net.github.io/multi/schema_BungieMembershipType.html#schema_BungieMembershipType).

## Compilation

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).