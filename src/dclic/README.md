# dclic

Command line tool for retrieving character information for specified member id


## USAGE
```
USAGE:
    dclic [FLAGS] --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help           
            Prints help information

        --hunter         
            Display information on Hunter character

        --last-active    
            Display information of last active character

    -t, --terse          
            terse output in the form of class_name:character_id . Errors are suppresed

        --titan          
            Display information on Titan character

    -V, --version        
            Prints version information

    -v, --verbose        
            Print out additional information for the API call

        --warlock        
            Display information on Warlock character


OPTIONS:
    -m, --member-id <member-id>    
            Destiny 2 API member id
            
            Destiny 2 API member id. This is not the user name, but the member id retrieved from the Destiny API.
    -p, --platform <platform>      
            Platform for specified id
            
            Platform for specified member id. Valid values are: xbox, playstation, stadia or steam
```

### Example

Retrieve all character information:

```
dclic --member-id 4611686018429783292 --platform xbox
```

outputs:

```
Titan      : 2305843009264966984
Warlock    : 2305843009264966986
Hunter     : 2305843009264966985
Hunter     : 2305843009264966985 (Last Active)
```

Retrieve information on last active character:

```
dclic --member-id 4611686018429783292 --platform xbox --last-active
```

outputs:

```
Hunter     : 2305843009264966985 (Last Active)
```


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the base directory for the program, and run:

```
cargo build --release
```

which will place the build in *target/release*