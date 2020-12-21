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

use std::path::PathBuf;

use dcli::activitystoreinterface::ActivityStoreInterface;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::utils::{build_tsv, print_error, print_verbose, EXIT_FAILURE};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for downloading and syncing Crucible activity history
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

    ///Directory where the manifest and meta-data will be stored.
    ///
    ///The manifest will be stored in this directory in a file named manifest.sqlite3
    #[structopt(short = "S", long = "store-path", parse(from_os_str))]
    store_path: PathBuf,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Destiny 2 API member id
    ///
    /// This is not the user name, but the member id
    /// retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id for the character to retrieve activities for.
    #[structopt(short = "c", long = "character-id")]
    character_id: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let mut store: ActivityStoreInterface =
        match ActivityStoreInterface::init_with_path(&opt.store_path, opt.verbose).await {
            Ok(e) => e,
            Err(e) => {
                print_error("Error initializing activity store.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

    match store
        .sync(&opt.member_id, &opt.character_id, &opt.platform)
        .await
    {
        Ok(_e) => {
            println!("done");
        }
        Err(e) => {
            print_error("Error syncing ids.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    match opt.output {
        Output::Default => {
            println!("{}", "DEFAULT OUTPUT");
        }
        Output::Tsv => {
            let mut name_values: Vec<(&str, String)> = Vec::new();
            name_values.push(("status", "COMPLETE".to_string()));

            print!("{}", build_tsv(name_values));
        }
    }
}
