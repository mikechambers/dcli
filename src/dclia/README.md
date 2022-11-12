# dclia

Command line tool for retrieving current Destiny 2 activity status for players.

Information includes current activity, location, and in the case of PvP modes (Crucible and Gambit), current map.

The API updates pretty quickly, and can be used to see check the activity and / or map while you are loading in.

The too expects that the manifest has been downloaded and synced to the default location using [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim). You can specify a custom path to the manifest using the --data-dir argument.

## USAGE

```
USAGE:
    dclia [FLAGS] [OPTIONS] --name <name>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information

    -v, --verbose
            Print out additional information

OPTIONS:
    -k, --api-key <api-key>
            API key from Bungie required for some actions.

            If specified the key will be passed to all Destiny API calls.

            You can obtain a key from https://www.bungie.net/en/Application [env:
            DESTINY_API_KEY=8eacb6527ea648fbbd8106990231c21c]
    -D, --data-dir <data-dir>
            Directory where Destiny 2 manifest database file is stored. (optional)

            This will normally be downloaded using the dclim tool, and stored in a file named manifest.sqlite3 (in the
            manifest directory specified when running dclim).
    -n, --name <name>
            Bungie name for player

            Name must be in the format of NAME#CODE. Example: foo#3280 You can find your name in game, or on Bungie's
            site at: https://www.bungie.net/7/en/User/Account/IdentitySettings
    -O, --output-format <output>
            Format for command output

            Valid values are default (Default) and tsv.

            tsv outputs in a tab (\t) separated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]

```

Manifest can be downloaded and synced with from [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim).

### Examples

#### Check for current activity.

```
$ dclia --name mesh#3230
```

outputs:

```
Playing Deep Stone Crypt Raid on Castalia Macula, Europa
```

#### Check for current activity with tab separated output:

```
$ dclia --name mesh#3230 --output-format tsv
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

### Environment Variables

#### DCLI_FIX_DATA

If the `DCLI_FIX_DATA` environment variable is set to `TRUE` then when corrupt or missing data is returned from the Bungie API, and there is not a valid local version, DCLI will attempt to retrieve updated, non-corrupt data from Bungie. (This sometimes happens if a lot of people leave a game, and no player names will be returned from the server).

Setting this to true can significantly slow down sync time, especially the initial sync, and in general, is meant to be used when using DCLI to create datastores for larger applications.

#### RUST_LOG

All dcli apps have support for log output via the [env_logger](https://docs.rs/env_logger/0.9.3/env_logger/) library. This is mostly used for development, but may be helpful when trying to debug any issues.

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the `src/` directory and run:

```
$ cargo build --release
```

which will place the compiled tools in _src/target/release_
