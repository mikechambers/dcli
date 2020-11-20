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

use exitfailure::ExitFailure;
use structopt::StructOpt;

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::platform::Platform;
use dcli::utils::{print_error, print_standard};

#[derive(StructOpt)]
/// Command line tool for retrieving current Destiny 2 activity for player..
///
/// Command line tool for retrieving character information for specified member id
/// Retrieves character information for the specified member id.
struct Opt {
    /// Platform for specified id
    ///
    /// Platform for specified member id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    #[structopt(short = "m", long = "member-id", required = true)]
    /// Destiny 2 API member id
    ///
    /// Destiny 2 API member id. This is not the user name, but the member id
    /// retrieved from the Destiny API.
    member_id: String,

    ///terse output in the form of class_name:character_id . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///Display information on Hunter character
    #[structopt(long = "hunter")]
    hunter: bool,

    ///Display information on Warlock character
    #[structopt(long = "warlock")]
    warlock: bool,

    ///Display information on Titan character
    #[structopt(long = "titan")]
    titan: bool,

    ///Display information of last active character
    #[structopt(long = "last-active")]
    last_active: bool,
}


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    Ok(())
}
