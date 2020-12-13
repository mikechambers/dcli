# dclic

Command line tool for retrieving Destiny 2 character information for specified member id.

Returns the class and charater id for each character, as well as which character was most recently played.


## USAGE
```
USAGE:
    dclic [FLAGS] [OPTIONS] --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -m, --member-id <member-id>    
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
    -o, --output <output>          
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -p, --platform <platform>      
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
```


| ARGUMENT | OPTIONS |
|---|---|
| --platform | xbox, playstation, stadia or steam |

member-id and platform can be retrieved with [dclis](https://github.com/mikechambers/dcli/tree/main/src/dclis).   


### Examples

#### Retrieve character information

```
$ dclic --member-id 4611686018429783292 --platform xbox
```

outputs:

```
CLASS       ID                      STATUS      
------------------------------------------------
Titan       2305843009264966984                 
Hunter      2305843009264966985     LAST ACTIVE 
Warlock     2305843009264966986                
```

#### Retrieve all character information with tab seperated output

```
$ dclic --member-id 4611686018429783292 --platform xbox --output tsv
```

outputs:

```
Warlock 2305843009264966986
Titan   2305843009264966984
Hunter  2305843009264966985     LAST ACTIVE
```
## Questions, Feature Requests, Feedback

If you have any questions, feature requests, need help, are running into issues, or just want to chat, join the [dcli Discord server](https://discord.gg/2Y8bV2Mq3p).

You can also log bugs and features requests on the [issues page](https://github.com/mikechambers/dcli/issues).


## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the `src/` directory and run:

```
$ cargo build --release
```

which will place the compiled tools in *src/target/release*
