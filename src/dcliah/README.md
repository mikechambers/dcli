# dcliah

Command line tool for retrieving Destiny 2 activity history.


## USAGE
```
USAGE:
    dcliah [FLAGS] [OPTIONS] --character-id <character-id> --member-id <member-id> --platform <platform>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -c, --character-id <character-id>    
            Destiny 2 API character id
            
            Destiny 2 API character id for the character to retrieve activities for.
    -m, --member-id <member-id>          
            Destiny 2 API member id
            
            This is not the user name, but the member id retrieved from the Destiny API.
        --mode <mode>                    
            Activity mode to return stats for
            
            Valid values are all (default), control, clash, mayhem, ironbanner, private, rumble, comp, quickplay and
            trialsofosiris. [default: all]
    -o, --output <output>                
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -p, --platform <platform>            
            Platform for specified id
            
            Valid values are: xbox, playstation, stadia or steam.
        --start-moment <start-moment>    
            Start moment from which to pull activities from
            
            Activities will be retrieved from start moment to the current time. For example, Specifying: --start-moment
            weekly_reset
            
            will return all activities since the last weekly reset on Tuesday.
            
            Valid values include daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly
            reset on Tuesday), day (last day), week (last week), month (last month), alltime and custom.
            
            When custom is specified, the custom start date in RFC3339 format must be specified with the --start-time
            argument.
            
            For example: --start-moment custom --start-time 2020-12-08T17:00:00.774187+00:00
            
            Specifying alltime retrieves all activitiy history and may take an extended amount of time to retrieve
            depending on the number of activities. [default: day]
    -d, --start-time <start-time>        
            Destiny 2 API character id
            
            Destiny 2 API character id. If not specified, data for all characters will be returned. Required when period
            is set to day, reset, week or month.
```


### Examples



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