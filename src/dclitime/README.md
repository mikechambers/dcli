# dclitime

Command line tool for retrieving date / time stamps for Destiny 2 weekly event moments.

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
    -T, --moment <moment>              
            The weekly Destiny 2 moment to retrieve the date / time stamp for
            
            Valid values are now, current_weekly (previous Tuesday weekly reset), next_weekly (upcoming Tuesday weekly
            reset), current_daily, next_daily, current_xur (previous Friday Xur reset), next_xur (upcoming Friday Xur
            reset), current_trials (previous Friday Trials reset), next_trials (upcoming Friday Trials reset) [default:
            now]
    -o, --output-format <output>              
            Format for command output
            
            Valid values are default (Default) and tsv.
            
            tsv outputs in a tab (\t) seperated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -f, --time-format <time-format>    
            Date / time format to output moment
            
            Valid values are rfc3339 (default), rfc2822 and unix (unix timestamp, number of non-leap seconds since
            January 1, 1970 0:00:00 UTC). [default: rfc3339]
```

| ARGUMENT | OPTIONS |
|---|---|
| --moment | now (default), daily (last daily reset), next_daily (next daily reset), weekend (last Friday reset), next_weekend (next Friday reset), weekly (last weekly reset on Tuesday), next_weekly (next weekly reset on Tuesday), day (previous 24 hours), next_day, week (previous week), next_week, month (previous month), next_month, all_time |
| --time-format | rfc3339 (default), rfc2822, unix |



| FORMATS | DESCRIPTION |
|---|---|
| rfc3339 | [RFC3339](https://tools.ietf.org/html/rfc3339) standard date / time format: Example: *2020-12-07T02:59:59.187080+00:00* |
| rfc2822 | [RFC2822](https://tools.ietf.org/html/rfc2822) standard date / time format : Example: *Mon, 07 Dec 2020 03:00:30 +0000*
| unix | Unix timestamp which is the number of non-leap seconds since January 1, 1970 0:00:00 UTC. Example: *1607446800* |



### Examples

#### Get date / time for the weekly Tuesday reset for the current week:
```
$ dclitime --moment weekly
```

#### Get date / time for the upcoming Xur reset on Friday in rfc2822 format:
```
$ dclitime --moment next_weekend --format rfc2822
```

#### Get date / time for next week's weekly reset on Tuesday and output in tab seperated value format:
```
$ dclitime --moment next_weekly --output-format tsv
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
