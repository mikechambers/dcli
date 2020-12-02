# dcli

dcli is a collection of utilities that provide a command line interface (CLI) for working with the [Destiny 2 API](https://github.com/Bungie-net/api).

This is an early alpha release, and there will probably be a lot of changes / refactoring. If you run into any issues, have any ideas, or just think something can be done better, please post in [issues](https://github.com/mikechambers/dcli/issues).

## Download

You can download the latest binaries for Windows, Linux and x86_64 Mac from the [releases](https://github.com/mikechambers/dcli-gha/releases/latest) page.

[![](https://img.shields.io/github/v/release/mikechambers/dcli?style=social)](https://github.com/mikechambers/dcli-gha/releases/latest)

**IMPORTANT**: Mac binaries are not signed, which can cause some hassle the first time you run them. You can find info on how to easily run them [here](https://github.com/mikechambers/dcli/wiki/Running-dcli-tools-on-Mac-OS-X).

## Utilities
* [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis) Tool for retrieving primary platform and membership ids for Destiny 2 players.
* [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim) Tool for managing and syncing the remote Destiny 2 API manifest database.
* [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic) Tool for retrieving character information for specified member id.
* [dclims](https://github.com/mikechambers/dcli/tree/main/src/dclims) Tool for searching the Destiny 2 manifest by hash ids (from API calls).
* [dclia](https://github.com/mikechambers/dcli/tree/main/src/dclia) Tool for retrieving information on player's current activity within Destiny 2.
* [dclics](https://github.com/mikechambers/dcli/tree/main/src/dclics) Tool for retrieving Destiny 2 Crucible activity stats.
* [dcli](https://github.com/mikechambers/dcli/tree/main/src/dcli) Library used across all of the dcli apps.

## Questions, Feature Requests, Feedback

If you have any questions, feature requests, need help, or just want to chat, join the [dcli Discord server](https://discord.gg/2Y8bV2Mq3p).

You can also log bugs and features requests on the [issues page](https://github.com/mikechambers/dcli/issues).

## Compiling

Tools are written and compiled in [Rust](https://www.rust-lang.org/).

![Build Status](https://github.com/mikechambers/dcli/workflows/dcli/badge.svg)

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for each app, and run:

```
cargo build --release
```

which will place the build in *target/release*

## License

All utilities released under a [MIT License](LICENSE.md).

[![License: MIT](https://img.shields.io/badge/License-MIT-orange.svg)](LICENSE.md)
