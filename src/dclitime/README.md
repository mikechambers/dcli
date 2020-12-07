# dclitime

Command line tool for retrieving date / time stamps for Destiny 2 weekly event moments

Supported weekly event momenets are:
* **Now** : *(now)* Current date / time
* **Current Weekly Reset** *(current_weekly) : Current weekly Tuesday reset date / time (i.e. previous Tuesday).
* **Next Weekly Reset** *(next_weekly)* : Upcoming weekly Tuesday reset date / time.
* **Current Daily Reset** *(current_daily)* : Currently daily reset date / time.
* **Next Daily Reset** *(next_daily)* : Upcoming daily reset date / time.
* **Current Xur Reset** *(current_xur)* : Current Xur Weekly Reset (on Friday).
* **Next Xur Reset** *(next_xur)* : Upcoming Xur Weekly Reset (on Friday).
* **Current Trials Reset** *(current_trials)* : Current Trials of Osiris Weekly Reset (on Friday).
* **Next Trials Reset** *(next_trials)* : Upcoming Trials of Osiris Weekly Reset (on Friday).

Output formats supported are:
* [rfc3339](https://tools.ietf.org/html/rfc3339) : (default) Example: *2020-12-07T02:59:59.187080+00:00*
* [rfc2822](https://tools.ietf.org/html/rfc2822) : Example: *Mon, 07 Dec 2020 03:00:30 +0000*
* unix : Unix timestamp, number of non-leap seconds since January 1, 1970 0:00:00 UTC. Example: *1607446800*

## USAGE
```
USAGE:
    dclitime [FLAGS] [OPTIONS]

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
            
            Valid values are rfc3339 (default), rfc2822 and unix (unix timestamp, number of non-leap seconds since
            January 1, 1970 0:00:00 UTC). [default: rfc3339]
    -m, --moment <moment>    
            The weekly Destiny 2 moment to retrieve the date / time stamp for
            
            Valid values are now, current_weekly (previous Tuesday weekly reset), next_weekly (upcoming Tuesday weekly
            reset), current_daily, next_daily, current_xur (previous Friday Xur reset), next_xur (upcoming Friday Xur
            reset), current_trials (previous Friday Trials reset), next_trials (upcoming Friday Trials reset) [default:
            now]
    -o, --output <output>    
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
```

### Examples

#### Get date / time for the weekly Tuesday reset for the current week:
```
$ dclitime--moment current_weekly
```

#### Get date / time for the upcoming Xur reset on Friday in rfc2822 format:
```
$ dclitime --moment next_xur --format rfc2822
```

#### Get date / time for next week's weekly reset on Tuesday and output in tab seperated value format:
```
$ dclitime --moment next_weekly --output tsv
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