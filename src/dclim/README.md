# dclim

Command line utility for managing and syncing the remote Destiny 2 API manifest database.

The utility will check whether a more current version of the Destiny 2 API manifest database is avaliable (comparing it to the last version which has been downloaded). If a new version is found, it will download the database, uncompress it, and save it to the directory specified when calling the utility. It will also save a file containting metadata about the current version, which is used for future checks for updates.

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
dclim --dir ~/manifest/
```

Download remote manifest and store in *~/manifest/* directory regardless of whether it is updated.
```
dclim --dir ~/manifest/ --force
```

Check status of remote manifest, but do not download. Print out additional information.
```
dclim --dir ~/manifest/ --check --verbose
```

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).