# dclia

Command line interface for retrieving information on current activity for specified player character.


## USAGE
```
```

Valid platforms are xbox, playstation, steam and stadia.

### Example

```
dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox
```

outputs:

```
Playing Deep Stone Crypt Raid on Castalia Macula, Europa
```

```
dclia --manifest-path ~/tmp/manifest.sqlite3 --member-id 4611686018429783292 --platform xbox --terse
```

outputs:

```
Mode:Strike
Activity:The Inverted Spire
Place:Nessus
Destination:Arcadian Valley
Description:End the Red Legion expedition that's ripped open the planet's surface.
```


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*