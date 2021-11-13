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

use std::path::PathBuf;

use dcli::activitystoreinterface::ActivityStoreInterface;
use dcli::crucible::PlayerName;
use dcli::output::Output;
use dcli::utils::{
    build_tsv, create_db_path, determine_data_dir, print_error, print_verbose,
    EXIT_FAILURE,
};
use structopt::StructOpt;

use dcli::activitystoreinterface::SyncResult;

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
    #[structopt(
        short = "O",
        long = "output-format",
        default_value = "default"
    )]
    output: Output,

    /// Directory where activity sqlite3 database will be stored. (optional)
    ///
    /// By default data will be loaded from and stored in the appropriate system
    /// local storage directory. Data will be stored in a sqlite3 database file
    /// named dcli.sqlite3
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,

    /// Bungie name for player
    ///
    /// Name must be in the format of NAME#CODE. Example: foo#3280
    /// You can find your name in game, or on Bungie's site at:
    /// https://www.bungie.net/7/en/User/Account/IdentitySettings
    #[structopt(long = "name", short = "n", required = true)]
    name: PlayerName,
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

    let db_path = create_db_path(&data_dir);

    let mut store: ActivityStoreInterface =
        match ActivityStoreInterface::init_with_path(&db_path, opt.verbose)
            .await
        {
            Ok(e) => e,
            Err(e) => {
                print_error("Error initializing activity store.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

    let member = match store.get_member(&opt.name).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                    "Could not find bungie name. Please check name and try again. {}",
                    e
                );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let results = match store.sync(&member).await {
        Ok(e) => e,
        Err(e) => {
            print_error("Error syncing ids.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let path = db_path.display().to_string();
    match opt.output {
        Output::Default => {
            print_default(&results, &path);
        }
        Output::Tsv => {
            print_tsv(&results, &path);
        }
    }
}

fn print_tsv(results: &SyncResult, path: &str) {
    let name_values: Vec<(&str, String)> = vec![
        ("total_synced", results.total_synced.to_string()),
        ("total_available", results.total_available.to_string()),
        ("path", path.to_string()),
    ];

    print!("{}", build_tsv(name_values));
}

fn print_default(results: &SyncResult, path: &str) {
    println!();
    println!("{}", "Activity sync complete".to_string().to_uppercase());
    println!("------------------------------------------------");

    let s = if results.total_synced == 1 {
        "y"
    } else {
        "ies"
    };

    println!("{} activit{} synced", results.total_synced, s);

    let total_available = results.total_available;
    let queue_str = if total_available == 1 {
        "1 activity in queue. Activity will be synced the next time app is run."
            .to_string()
    } else if total_available == 0 {
        "No activities in queue".to_string()
    } else {
        format!(
            "{} activies in queue. Activities will be synced the next time app is run",
            results.total_available
        )
    };

    println!("{}", queue_str);

    println!("Database stored at: {}", path);
}
