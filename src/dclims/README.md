# dclims

Command line tool for searching the Destiny 2 manifest by hash ids (from API calls).


## USAGE
```
USAGE:
    dclims [FLAGS] --hash <hash> --manifest-path <manifest-path>

FLAGS:
    -h, --help       
            Prints help information

    -t, --terse      
            terse output in the form of class_name:character_id . Errors are suppresed

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information for the API call


OPTIONS:
        --hash <hash>                      
            The hash id from the Destiny 2 API for the item to be searched for. Example : 326060471

    -m, --manifest-path <manifest-path>    
            Local path the Destiny 2 manifest database file
```

Valid platforms are xbox, playstation, steam and stadia.

### Example

Retrieve information for *Luna's Howl* by its API hash id.
```
dclims --manifest-path ~/tmp/manifest.sqlite3 --hash 3260604718
```

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).