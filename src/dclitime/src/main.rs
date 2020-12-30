/*
* Copyright 2020 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

mod datetimeformat;

use std::str::FromStr;

use datetimeformat::DateTimeFormat;
use dcli::enums::moment::Moment;
use dcli::output::Output;
use dcli::utils::{build_tsv, print_verbose};
use structopt::StructOpt;

//we do a custom parse / validation here so we can reuse Moment enum
//across apps but not have to have all apps support all time ranges.
fn parse_and_validate_moment(src: &str) -> Result<Moment, String> {
    let moment = Moment::from_str(src)?;

    //note, we positive capture what we want in case new properties
    //are added in the future
    match moment {
        Moment::Now => {}
        Moment::Daily => {}
        Moment::NextDaily => {}
        Moment::Weekend => {}
        Moment::NextWeekend => {}
        Moment::Weekly => {}
        Moment::NextWeekly => {}
        Moment::Day => {}
        Moment::NextDay => {}
        Moment::Week => {}
        Moment::NextWeek => {}
        Moment::Month => {}
        Moment::NextMonth => {}
        Moment::AllTime => {}
        _ => {
            return Err(format!("Unsupported moment specified : {}", src));
        }
    };

    Ok(moment)
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving date / time stamps for Destiny 2 weekly event
/// moments
///
/// Created by Mike Chambers.
/// https://www.mikechambers.com
///
/// Get support, request features or just chat on the dcli Discord server:
/// https://discord.gg/2Y8bV2Mq3p
///
/// Get the latest version, download the source and log issues at:
/// https://github.com/mikechambers/dcli
///
/// Released under an MIT License.
struct Opt {
    /// The weekly Destiny 2 moment to retrieve the date / time stamp for
    ///
    /// Valid values are now, current_weekly (previous Tuesday weekly reset),
    /// next_weekly (upcoming Tuesday weekly reset), current_daily, next_daily,
    /// current_xur (previous Friday Xur reset), next_xur (upcoming Friday Xur reset),
    /// current_trials (previous Friday Trials reset), next_trials (upcoming Friday Trials reset)
    #[structopt(short = "T", parse(try_from_str = parse_and_validate_moment), long = "moment", default_value = "now")]
    moment: Moment,

    /// Date / time format to output moment
    ///
    /// Valid values are rfc3339 (default), rfc2822 and unix (unix timestamp,
    /// number of non-leap seconds since January 1, 1970 0:00:00 UTC).
    #[structopt(short = "f", long = "time-format", default_value = "rfc3339")]
    time_format: DateTimeFormat,

    /// Print out additional information
    ///
    /// Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(short = "O", long = "output-format", default_value = "default")]
    output: Output,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let dt = opt.moment.get_date_time();
    let date_time_str = match opt.time_format {
        DateTimeFormat::RFC3339 => dt.to_rfc3339(),
        DateTimeFormat::RFC2822 => dt.to_rfc2822(),
        DateTimeFormat::Unix => dt.timestamp().to_string(),
    };

    match opt.output {
        Output::Default => {
            println!("{}", date_time_str);
        }
        Output::Tsv => {
            let mut name_values: Vec<(&str, String)> = Vec::new();
            name_values.push(("date_time", date_time_str));
            name_values.push(("format", format!("{}", opt.time_format)));
            name_values.push(("moment", format!("{}", opt.moment)));

            print!("{}", build_tsv(name_values));
        }
    }
}
