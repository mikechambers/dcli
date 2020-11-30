# dcli

dcli is a collection of utilities that provide a command line interface (CLI) for working with the [Destiny 2 API](https://github.com/Bungie-net/api).

This is an early alpha release, and there will probably be a lot of changes / refactoring. If you run into any issues, have any ideas, or just think something can be done better, please post in [issues](https://github.com/mikechambers/dcli/issues).

## Utilities
* [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis) Command line tool for retrieving primary platform and membership ids for Destiny 2 players.
* [dclim](https://github.com/mikechambers/dcli/tree/main/src/dclim) Command line tool for managing and syncing the remote Destiny 2 API manifest database.
* [dclic](https://github.com/mikechambers/dcli/tree/main/src/dclic) Command line tool for retrieving character information for specified member id.
* [dclims](https://github.com/mikechambers/dcli/tree/main/src/dclims) Command line tool for searching the Destiny 2 manifest by hash ids (from API calls).
* [dclia](https://github.com/mikechambers/dcli/tree/main/src/dclia) Command line toll for retrieving information on player's current activity within Destiny 2.
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
