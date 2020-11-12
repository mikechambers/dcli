# dcli

dcli is a collection of utilities that provide a command line interface (CLI) for working with the [Destiny 2 API](https://github.com/Bungie-net/api). 

The apps require an API key from Bungie for working with the API. More info at: [https://www.bungie.net/en/Application](https://www.bungie.net/en/Application).

## Utilities

### dclis

Command line interface for retrieving primary platform and membership ids for Destiny 2 players.

Note, in cases of players who have enabled cross save / play, the primary id and platform will be returned.

#### USAGE
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

##### Example

Retrieve membership id for a player on xbox and print the url for the API call:
```
dclis --id mesh --platform mesh --url
```

Search for the membership id using the steam 64 id, and print out in compact format membershipid:platform_id
```
dclis --id 76561197984551459 --platform steam --compact
```
Note, platform ids, can be found [here](https://bungie-net.github.io/multi/schema_BungieMembershipType.html#schema_BungieMembershipType).

#### Compilation

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

### dclidp

Python 3 based Command line utility for managing and syncing the remote Destiny 2 API manifest database. The utility can be used to:

* Download latest version of the manifest sqlite database file
* Query whether the manifest has been updated since it was last downloaded
* Query the latest URL and version for the remote manifest

####USAGE
```
usage: dclidp.py [-h] --key KEY --manifest_dir MANIFEST_DIR [--version]
             [--info {local.version,local.url,remote.version,remote.url}]
             [--check] [--force]
```

The `--manifest_dir` argument is required, and should point a directory where the manifest and manifest info file can be saved anand maintained.

The `--key` argument is required and specifies the API key from Bungie.