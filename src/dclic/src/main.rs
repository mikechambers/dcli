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

use dcli::apiinterface::ApiInterface;
use dcli::character::{Character, CharacterClass};
use dcli::error::Error;
use dcli::platform::Platform;
use dcli::utils::{print_error, print_standard};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

//todo: could move this to apiclient
async fn retrieve_characters(
    member_id: String,
    platform: Platform,
    verbose: bool,
) -> Result<Vec<Character>, Error> {
    let url =
        format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=200",
            platform_id = platform.to_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
        );

    let interface = ApiInterface::new(verbose);

    interface.retrieve_characters(member_id, platform)?
}

#[derive(StructOpt)]
/// Command line tool for retrieving character information for specified member id.
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

fn c_status_error(e: &Error) -> Option<String> {
    match e {
        Error::ParameterParseFailure => Some(
            "Invalid parameters. Check that --member-id is correct.".to_string(),
        ),
        Error::InvalidParameters => Some(
            "Invalid parameters. Check that --member-id and --platform are correct.".to_string(),
        ),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let all = !(opt.hunter || opt.titan || opt.warlock || opt.last_active);

    let characters: Vec<Character> =
        match retrieve_characters(opt.member_id, opt.platform, opt.verbose).await {
            Ok(e) => e,
            Err(e) => {
                match c_status_error(&e) {
                    Some(e) => print_error(&e, !opt.terse),
                    None => print_error(&format!("{}", e,), !opt.terse),
                }
                std::process::exit(1);
            }
        };

    if !characters.is_empty() {

        //TODO: move this to seperate API?
        let mut most_recent: &Character = &characters[0];

        for c in characters.iter() {
            if c.date_last_played > most_recent.date_last_played {
                most_recent = c;
            }

            if (opt.hunter && c.class_type == CharacterClass::Hunter)
                || (opt.titan && c.class_type == CharacterClass::Titan)
                || (opt.warlock && c.class_type == CharacterClass::Warlock)
                || all
            {
                //regular
                print_standard(
                    &format!("{: <10} : {: <10}", c.class_type, c.id),
                    !opt.terse,
                );

                //terse
                print_standard(&format!("{}:{}", c.class_type, c.id), opt.terse);
            }
        }

        //regular
        print_standard(
            &format!(
                "{: <10} : {: <10} (Last Active)",
                most_recent.class_type, most_recent.id
            ),
            !opt.terse && (opt.last_active || all),
        );

        //terse
        print_standard(
            &format!("Last Active:{}:{}", most_recent.class_type, most_recent.id),
            opt.terse && (opt.last_active || all),
        );
    }

    Ok(())
}
