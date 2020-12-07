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

mod eventmoment;
mod datetimeformat;

use eventmoment::EventMoment;
use datetimeformat::DateTimeFormat;

use dcli::utils::print_verbose;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving current Destiny 2 activity stats.
///
/// Enables control of which stats are retrieved via game mode, time period and
/// character.
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

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "m", long = "moment")]
    moment: EventMoment,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "f", long = "format", default_value="rfc3339")]
    format: DateTimeFormat,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

//TODO: specify format output
//to_rfc2822
//to_rfc3339

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let dt = opt.moment.get_date_time();
    let date_time_str = match opt.format {
        DateTimeFormat::Chrono => format!("{}", dt),
        DateTimeFormat::RFC3339 => dt.to_rfc3339(),
        DateTimeFormat::RFC2822 => dt.to_rfc2822(),
    };

    println!("{}", date_time_str);
    

}
