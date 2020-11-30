# dcli

dcli is a collection of utilities that provide a command line interface (CLI) for working with the [Destiny 2 API](https://github.com/Bungie-net/api).

This is an early alpha release, and there will probably be a lot of changes / refactoring. If you run into any issues, have any ideas, or just think something can be done better, please post in [issues](https://github.com/mikechambers/dcli/issues).

## Utilities
* [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis) Tool for retrieving primary platform and membership ids for Destiny 2 players.
* [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim) Tool for managing and syncing the remote Destiny 2 API manifest database.
* [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic) Tool for retrieving character information for specified member id.
* [dclims](https://github.com/mikechambers/dcli/tree/main/src/dclims) Tool for searching the Destiny 2 manifest by hash ids (from API calls).
* [dclia](https://github.com/mikechambers/dcli/tree/main/src/dclia) Tool for retrieving information on player's current activity within Destiny 2.
* [dclics](https://github.com/mikechambers/dcli/tree/main/src/dclics) Tool for retrieving Destiny 2 Crucible activity stats.
* [dcli](https://github.com/mikechambers/dcli/tree/main/src/dcli) Library used across all of the dcli apps.

## Compiling

Tools are written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for each app, and run:

```
cargo build --release
```

which will place the build in *target/release*


## License

All utilities released under a [MIT License](LICENSE.md).
