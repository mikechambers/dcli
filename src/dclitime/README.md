# dclitime

Command line tool for retrieving date / time stamps for Destiny 2 weekly event moments

Supported weekly event momenets are:
* Now : (now) Current date / time
* Last Weekly Reset (lastweeklyreset) : Previous weekly Tuesday reset date / time.
* Next Weekly Reset (nextweeklyreset) : Upcoming weekly Tuesday reset date / time.
* Last Daily Reset (lastdailyreset) : Previous daily reset date / time.
* Next Daily Reset (lastdailyreset) : Upcoming daily reset date / time.
* Last Xur Reset (lastxurreset) : Last Xur Weekly Reset (on Friday).
* Next Xur Reset (nextxurreset) : Upcoming Xur Weekly Reset (on Friday).
* Last Trials Reset (lastxurreset) : Last Trials of Osiris Weekly Reset (on Friday).
* Next Trials Reset (nextxurreset) : Upcoming Trials of Osiris Weekly Reset (on Friday).

Output formats supported are:
* [rfc3339](https://tools.ietf.org/html/rfc3339) : (default) Example: 2020-12-07T02:59:59.187080+00:00
* [rfc2822](https://tools.ietf.org/html/rfc2822) : Example: Mon, 07 Dec 2020 03:00:30 +0000

## USAGE
```
USAGE:
    dclitime [FLAGS] [OPTIONS] --moment <moment>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information

    -v, --verbose    
            Print out additional information
            
            Output is printed to stderr.

OPTIONS:
    -f, --format <format>    
            Date / time format to output moment
            
            Valid values are rfc3339 (default) and rfc2822 [default: rfc3339]
    -m, --moment <moment>    
            The weekly Destiny 2 moment to retrieve the date / time stamp for.
            
            Valid values are now, lastweeklyreset (previous Tuesday weekly reset), nextweeklyreset (upcoming Tuesday
            weekly reset), lastdailyreset, nextdailyreset, lastxureset (previous Friday Xur reset), nextxurreset
            (upcoming Friday Xur reset), lasttrialsreset (previous Friday Trials reset), nexttrialsreset (upcoming
            Friday Trials reset)
    -o, --output <output>    
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
```

### Examples

#### Get date / time for the weekly Tuesday reset for the current week:
```
$ dclitime--moment lastweeklyreset
```

#### Get date / time for the upcoming Xur reset on Friday in rfc2822 format:
```
$ dclitime --moment nextxurreset --format rfc2822
```

#### Get date / time for next week's weekly reset on Tuesday and output in tab seperated value format:
```
$ dclitime --moment nextweeklyreset --output tsv
```

which outputs:

```
date_time       2020-12-08T17:00:00.774187+00:00
format  RFC 3339
moment  Next Weekly Reset
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