# dclim

Command line utility for managing and syncing the remote Destiny 2 API manifest database.

The utility will check whether a more current version of the Destiny 2 API manifest database is avaliable (comparing it to the last version which has been downloaded). If a new version is found, it will download the database, uncompress it, and save it to the directory specified when calling the utility. It will also save a file containting metadata about the current version, which is used for future checks for updates.

The utility expects that the downloaded manifest will not be moved from the directory it is downloaded to, and uses that information to determine whether a new version is avaliable. If the manifest is moved, the utility will re-download the manifest on next check.

The utility uses the download url for the manifest to check for a new version. While it displays the version number, that is not used to detect whether a new version is avaliable.

The manifest is is a [Sqlite 3](https://www.sqlite.org/index.html) database.

## USAGE
```
USAGE:
    dclim [FLAGS] --dir <manifest-dir>

FLAGS:
    -c, --check      Check whether a new manifest version is available, but do not download. If overridden by --force
    -f, --force      Force a download of manifest regardless of whether it has been updated. Overrides --check
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Output additional information

OPTIONS:
    -d, --dir <manifest-dir>    Directory where the manifest and associated files will be stored.
```

### Example

Check for an updated manifest and store and new version in *~/manifest/*:
```
dclim --manifest-dir ~/manifest/
```

Download remote manifest and store in *~/manifest/* directory regardless of whether it is updated.
```
dclim --manifest-dir ~/manifest/ --force
```

Check status of remote manifest, but do not download. Print out additional information.
```
dclim --manifest-dir ~/manifest/ --check --verbose
```

which outputs:

```
https://www.bungie.net/Platform/Destiny2/Manifest/
Remote Manifest version : 89248.20.11.16.2016-1
Remote Manifest url     : https://www.bungie.net/common/destiny2_content/sqlite/en/world_sql_content_0620bb54c59d99be87842a16ffbf22b2.content
Local Manifest version  : 89248.20.11.16.2016-1
Local Manifest url      : https://www.bungie.net/common/destiny2_content/sqlite/en/world_sql_content_0620bb54c59d99be87842a16ffbf22b2.content
No new manifest avaliable.
```

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*