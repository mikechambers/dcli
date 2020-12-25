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

use dcli::enums::platform::Platform;
use dcli::output::Output;
use dcli::utils::{build_tsv, determine_data_dir, print_error, print_verbose, EXIT_FAILURE};
use dcli::{activitystoreinterface::ActivityStoreInterface, manifestinterface::ManifestInterface};
use structopt::StructOpt;

use dcli::enums::mode::Mode;
use dcli::enums::moment::Moment;

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for downloading and syncing Destiny 2 Crucible activity
/// history to a sqlite3 database file.
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

    /// Directory where activity sqlite3 database will be stored. (optional)
    ///
    /// By default data will be loaded from and stored in the appropriate system
    /// local storage directory. Data will be stored in a sqlite3 database file
    /// named dcli.sqlite3
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,

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

    let data_dir = match determine_data_dir(opt.data_dir) {
        Ok(e) => e,
        Err(e) => {
            print_error("Error initializing storage directory store.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut store: ActivityStoreInterface =
        match ActivityStoreInterface::init_with_path(&data_dir, opt.verbose).await {
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
            let mode = Mode::AllPvP;
            let start_time = Moment::AllTime.get_date_time();
            let mut manifest = match ManifestInterface::new(&data_dir, false).await {
                Ok(e) => e,
                Err(e) => {
                    print_error("Cant create manifest.", e);
                    std::process::exit(EXIT_FAILURE);
                }
            };
            println!("START : retrieve_activities_since");
            let now = std::time::Instant::now();
            match store
                .retrieve_activities_since(
                    &opt.member_id,
                    &opt.character_id,
                    &opt.platform,
                    &mode,
                    &start_time,
                    &mut manifest,
                )
                .await
            {
                Ok(_e) => {
                    println!(
                        "END retrieve_activities_since {}",
                        now.elapsed().as_millis()
                    );
                }
                Err(e) => {
                    print_error("Error retrieve_activities_since.", e);
                    std::process::exit(EXIT_FAILURE);
                }
            }
        }
        Err(e) => {
            print_error("Error syncing ids.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    match opt.output {
        Output::Default => {
            println!("Sync complete. Database stored at:");
            println!("{}", store.get_storage_path());
        }
        Output::Tsv => {
            let mut name_values: Vec<(&str, String)> = Vec::new();
            name_values.push(("status", "COMPLETE".to_string()));

            print!("{}", build_tsv(name_values));
        }
    }
}
