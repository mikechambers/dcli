/*
* Copyright 2021 Mike Chambers
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

use dcli::apiinterface::ApiInterface;
use dcli::character::Characters;
use dcli::enums::platform::Platform;
use dcli::error::Error;
use dcli::output::Output;
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose, repeat_str, TSV_DELIM, TSV_EOL};
use structopt::StructOpt;

//todo: could move this to apiclient
async fn retrieve_characters(
    member_id: String,
    platform: Platform,
    verbose: bool,
) -> Result<Option<Characters>, Error> {
    let interface = ApiInterface::new(verbose)?;

    let characters =
        interface.retrieve_characters(&member_id, &platform).await?;

    Ok(characters)
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving character information for specified member id.
///
/// Retrieves character information for all characters, as well as most recently
/// played character.
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
    /// Destiny 2 API member id
    ///
    /// This is not the user name, but the member id retrieved from the Destiny
    /// API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(
        short = "O",
        long = "output-format",
        default_value = "default"
    )]
    output: Output,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let chars: Characters =
        match retrieve_characters(opt.member_id, opt.platform, opt.verbose)
            .await
        {
            Ok(e) => match e {
                Some(e) => e,
                None => {
                    println!("No Characters found for member.");
                    return;
                }
            },
            Err(e) => {
                print_error("Error retrieving characters from API.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

    match opt.output {
        Output::Default => {
            print_default(&chars);
        }
        Output::Tsv => {
            print_tsv(&chars);
        }
    }
}

fn print_default(characters: &Characters) {
    let col_w = 12;
    let col_id = 24;
    println!(
        "{:<0col_w$}{:<0col_id$}{:<0col_w$}",
        "CLASS",
        "ID",
        "STATUS",
        col_w = col_w,
        col_id = col_id,
    );

    println!("{}", repeat_str("-", col_w * 2 + col_id));

    for p in characters.characters.iter() {
        //we unwrap here because we wont be in loop if there are no items in
        //characters, and thus last active should always return a reference
        let label = if p == characters.get_last_active_ref().unwrap() {
            "LAST ACTIVE"
        } else {
            ""
        };

        println!(
            "{:<0col_w$}{:<0col_id$}{:<0col_w$}",
            p.class_type,
            p.id,
            label,
            col_w = col_w,
            col_id = col_id,
        );
    }
}

fn print_tsv(characters: &Characters) {
    for p in characters.characters.iter() {
        let label = if p == characters.get_last_active_ref().unwrap() {
            "LAST ACTIVE"
        } else {
            ""
        };

        print!(
            "{c}{delim}{i}{delim}{s}{eol}",
            c = p.class_type,
            i = p.id,
            s = label,
            delim = TSV_DELIM,
            eol = TSV_EOL
        );
    }
}
