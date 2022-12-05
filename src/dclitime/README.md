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

            tsv outputs in a tab (\t) separated format of name / value pairs with lines ending in a new line character
            (\n). [default: default]
    -f, --time-format <time-format>
            Date / time format to output moment

            Valid values are rfc3339 (default), rfc2822 and unix (unix timestamp, number of non-leap seconds since
            January 1, 1970 0:00:00 UTC). [default: rfc3339]
```

| ARGUMENT      | OPTIONS                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --moment      | daily (last daily reset), weekend (last weekend reset on Friday), weekly (last weekly reset on Tuesday), day (last day), week (last week), month (last month), all_time, custom, launch, curse_of_osiris, warmind, season_of_the_outlaw, season_of_the_forge, season_of_the_drifter, season_of_opulence, season_of_the_undying, season_of_dawn, season_of_the_worthy, season_of_arrivals, season_of_the_hunt, season_of_the_chosen, season_of_the_splicer, season_of_the_lost, season_of_the_risen, witch_queen, season_of_the_haunted, season_of_plunder, season_of_redacted |
| --time-format | rfc3339 (default), rfc2822, unix                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

| FORMATS | DESCRIPTION                                                                                                             |
| ------- | ----------------------------------------------------------------------------------------------------------------------- |
| rfc3339 | [RFC3339](https://tools.ietf.org/html/rfc3339) standard date / time format: Example: _2020-12-07T02:59:59.187080+00:00_ |
| rfc2822 | [RFC2822](https://tools.ietf.org/html/rfc2822) standard date / time format : Example: _Mon, 07 Dec 2020 03:00:30 +0000_ |
| unix    | Unix timestamp which is the number of non-leap seconds since January 1, 1970 0:00:00 UTC. Example: _1607446800_         |

### Examples

#### Get date / time for the weekly Tuesday reset for the current week:

```
$ dclitime --moment weekly
```

#### Get date / time for the upcoming Xur reset on Friday in rfc2822 format:

```
$ dclitime --moment next_weekend --format rfc2822
```

#### Get date / time for next week's weekly reset on Tuesday and output in tab separated value format:

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

### Environment Variables

#### DCLI_FIX_DATA

If the `DCLI_FIX_DATA` environment variable is set to `TRUE` then when corrupt or missing data is returned from the Bungie API, and there is not a valid local version, DCLI will attempt to retrieve updated, non-corrupt data from Bungie. (This sometimes happens if a lot of people leave a game, and no player names will be returned from the server).

Setting this to true can significantly slow down sync time, especially the initial sync, and in general, is meant to be used when using DCLI to create datastores for larger applications.

#### RUST_LOG

All dcli apps have support for log output via the [env_logger](https://docs.rs/env_logger/0.9.3/env_logger/) library. This is mostly used for development, but may be helpful when trying to debug any issues.

## Compiling

This utility is written and compiled in [Rust](https://www.rust-lang.org/).

When compiling you must have an environment variable named `DESTINY_API_KEY` which contains your [Bungie API key](https://www.bungie.net/en/Application).

To compile, switch to the `src/` directory and run:

```
$ cargo build --release
```

which will place the compiled tools in _src/target/release_
