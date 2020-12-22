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

use dcli::apiinterface::ApiInterface;
use dcli::error::Error;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::response::character::CharacterData;
use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose, repeat_str, TSV_DELIM, TSV_EOL};
use structopt::StructOpt;

//todo: could move this to apiclient
async fn retrieve_characters(
    member_id: String,
    platform: Platform,
    verbose: bool,
) -> Result<Vec<CharacterData>, Error> {
    let interface = ApiInterface::new(verbose)?;

    let characters = interface.retrieve_characters(member_id, platform).await?;

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

    let chars: Vec<CharacterData> =
        match retrieve_characters(opt.member_id, opt.platform, opt.verbose)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                print_error("Error retrieving characters from API.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

    let char_data = get_char_info(&chars);

    match opt.output {
        Output::Default => {
            print_default(char_data);
        }
        Output::Tsv => {
            print_tsv(char_data);
        }
    }
}

fn print_default(char_data: Vec<(CharacterData, String)>) {
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

    for p in char_data.iter() {
        println!(
            "{:<0col_w$}{:<0col_id$}{:<0col_w$}",
            p.0.class_type,
            p.0.id,
            p.1,
            col_w = col_w,
            col_id = col_id,
        );
    }
}

fn get_char_info(characters: &[CharacterData]) -> Vec<(CharacterData, String)> {
    let mut out: Vec<(CharacterData, String)> = Vec::new();

    if characters.is_empty() {
        return out;
    }

    let mut most_recent: &CharacterData = &characters[0];

    for c in characters.iter() {
        if c.date_last_played > most_recent.date_last_played {
            most_recent = c;
        }
    }

    for c in characters.iter() {
        let status = if most_recent == c {
            "LAST ACTIVE".to_string()
        } else {
            "".to_string()
        };

        //note, this would be better as a reference, but need to figure out
        //how to do it
        out.push((c.clone(), status));
    }
    out
}

fn print_tsv(char_data: Vec<(CharacterData, String)>) {
    for p in char_data.iter() {
        print!(
            "{c}{delim}{i}{delim}{s}{eol}",
            c = p.0.class_type,
            i = p.0.id,
            s = p.1,
            delim = TSV_DELIM,
            eol = TSV_EOL
        );
    }
}
