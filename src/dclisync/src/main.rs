/*
* Copyright 2022 Mike Chambers
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

use tokio::signal::ctrl_c;
use tokio::signal::unix::{signal, SignalKind};

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use dcli::activitystoreinterface::ActivityStoreInterface;
use dcli::apiinterface::ApiInterface;
use dcli::crucible::{Member, PlayerName};
use dcli::utils::{
    determine_data_dir, print_error, print_verbose, EXIT_FAILURE, EXIT_SUCCESS,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for downloading and syncing Destiny 2 Crucible activity
/// history to a sqlite3 database file.
///
/// You may add and remove users via the --add and --remove flags, as well as import
/// all clan members via the --import flag.
///
/// If multiple flags are specified, they will be run in the following order:
/// import, add, remove, sync, list
///
/// Some options require that a Bungie API key is specified via the --api-key KEY flag,
/// or DESTINY_API_KEY environment variable.
///
/// You can obtain a key from https://www.bungie.net/en/Application
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

    /// Directory where activity sqlite3 database will be stored. (optional)
    ///
    /// By default data will be loaded from and stored in the appropriate system
    /// local storage directory. Data will be stored in a sqlite3 database file
    /// named dcli.sqlite3
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,

    /// Sync player activities.
    ///
    /// If no arguments are provided, all players will be synced. Optionally,
    /// you can pass in one or more space seperated Bungie names and codes.
    /// If a name has a space in it, you must the entire name in quotes.
    ///
    /// Name(s) must be in the format of NAME#CODE. Example: foo#3280
    /// You can find your name in game, or on Bungie's site at:
    /// https://www.bungie.net/7/en/User/Account/IdentitySettings
    ///
    /// Requires that a Bungie API key is specified via the --api-key KEY flag,
    /// or DESTINY_API_KEY environment variable.
    ///
    /// You can obtain a key from https://www.bungie.net/en/Application
    #[structopt(
        long = "sync",
        short = "s",
        //conflicts_with_all = &["add", "remove", "list"],
        //required_unless_one=&["list", "add", "remove"],`
        required_unless_one = &["list", "add", "remove", "import-group"],
        requires="api-key"
    )]
    sync: Option<Vec<PlayerName>>,

    /// Add specified player(s) to have their activities synced the next time
    /// the database is synced.
    ///
    /// Name(s) must be in the format of NAME#CODE. Example: foo#3280
    /// You can find your name in game, or on Bungie's site at:
    /// https://www.bungie.net/7/en/User/Account/IdentitySettings
    ///
    /// Requires that a Bungie API key is specified via the --api-key KEY flag,
    /// or DESTINY_API_KEY environment variable.
    ///
    /// You can obtain a key from https://www.bungie.net/en/Application
    #[structopt(long = "add", short = "A", 
        //conflicts_with_all = &["sync", "remove"], 
        //required_unless_one=&["sync", "list", "remove"]
        requires="api-key"
    )]
    add: Option<Vec<PlayerName>>,

    /// Remove specified player(s) from having their acitivities synced.
    ///
    /// Note, player data will still be contained in the database, but no new
    /// activities will be synced for the player(s)
    ///
    /// Name(s) must be in the format of NAME#CODE. Example: foo#3280
    /// You can find your name in game, or on Bungie's site at:
    /// https://www.bungie.net/7/en/User/Account/IdentitySettings
    #[structopt(long = "remove", short = "r", 
        //required_unless_one = &["sync", "add", "list"], 
        //conflicts_with_all = &["sync", "add"],
    )]
    remove: Option<Vec<PlayerName>>,

    ///List all Bungie names which are flagged to be synced.
    #[structopt(short = "l", long = "list", 
        //required_unless_one = &["sync", "add", "remove"], 
        //conflicts_with_all = &["sync"]
    )]
    list: bool,

    /// Import all players for specified Destiny 2 Group / clan.
    ///
    /// You can get your groupid for your clan from the Bungie clan page:
    /// https://www.bungie.net/en/ClanV2/MyClans
    /// Click your clan, then copy the group id from the URL.
    ///
    /// Requires that a Bungie API key is specified via the --api-key KEY flag,
    /// or DESTINY_API_KEY environment variable.
    ///
    /// You can obtain a key from https://www.bungie.net/en/Application
    #[structopt(short = "i", long = "import-group", requires = "api-key")]
    import_group: Option<u32>,

    /// API key from Bungie required for some actions.
    ///
    /// You can obtain a key from https://www.bungie.net/en/Application
    #[structopt(short = "k", long = "api-key", env = "DESTINY_API_KEY")]
    api_key: Option<String>,
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

    let key = match opt.api_key {
        Some(e) => e,
        None => "".to_string(),
    };

    //NOTE: if a key is not provided StructOpt should reject input
    let mut store: ActivityStoreInterface =
        match ActivityStoreInterface::init_with_path(
            &data_dir,
            opt.verbose,
            Some(key.to_string()),
        )
        .await
        {
            Ok(e) => e,
            Err(e) => {
                print_error("Error initializing activity store.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

    if opt.import_group.is_some() {
        let group_id = opt.import_group.unwrap();

        println!("Import Group ID : {}", group_id);

        let api = match ApiInterface::new_with_key(opt.verbose, &key) {
            Ok(e) => e,
            Err(e) => {
                print_error("Error creating interface.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

        let members: Vec<Member> =
            match api.retrieve_group_members(group_id).await {
                Ok(e) => e,
                Err(e) => {
                    print_error("Error creating interface.", e);
                    std::process::exit(EXIT_FAILURE);
                }
            };

        for m in members.iter() {
            //if they dont have a valid bungie name / id, then skip
            if !m.name.is_valid_bungie_name() {
                println!(
                    "No valid bungie name and code. Skipping. : {}",
                    m.name.get_short_name()
                );
                continue;
            }

            match store.add_member_to_sync(&m).await {
                Ok(_) => println!("{}", m.name.get_bungie_name()),
                Err(e) => {
                    println!(
                        "Error adding {}. Skipping. {}",
                        m.name.get_bungie_name(),
                        e
                    );
                }
            }
        }
    }

    if opt.add.is_some() {
        let players = opt.add.unwrap();

        println!("Added");
        println!("-------------");
        for player in players.iter() {
            match store.add_player_to_sync(&player).await {
                Ok(_) => println!("{}", player.get_bungie_name()),
                Err(e) => {
                    println!(
                        "Error adding {}. {}",
                        player.get_bungie_name(),
                        e
                    );
                }
            }
        }
        println!();
    }

    if opt.remove.is_some() {
        let players = opt.remove.unwrap();

        println!("Removed");
        println!("-------------");
        for player in players.iter() {
            match store.remove_player_from_sync(&player).await {
                Ok(_) => println!("{}", player.get_bungie_name()),
                Err(e) => {
                    println!(
                        "Error removing {}. {}",
                        player.get_bungie_name(),
                        e
                    );
                }
            }
        }
        println!();
    }

    if opt.sync.is_some() {

        use std::{thread, time};
        use std::sync::Arc;
        use std::sync::atomic::AtomicBool;
        use ctrlc;

        //TODO: add option to put into daemon mode (-d)
        //add refresh interval options -r (that is only used with -d)
        //only run loop when in daemon mode
        //change to different library to catch sigint and sigterm
        //check which is sent from systemctl
        //make sure timing in systemctl service gives us enough time to shut down
        //the services

        let delay = time::Duration::from_secs(15);

        let is_sleeping = Arc::new(AtomicBool::new(false));
        let is_sleeping2 = is_sleeping.clone();
        let is_sleeping3 = is_sleeping.clone();

        //All three variable refer to the same one. We have to have three as
        //as the reference to the second is moved in the set_handled call, and
        //so we can't reference it within the loop.
        let should_interrupt = Arc::new(AtomicBool::new(false));
        let should_interrupt2 = should_interrupt.clone();
        let should_interrupt3 = should_interrupt.clone();

        ctrlc::set_handler(move || {
           
            println!("Ctrl-c received. Cleaning up and shutting down.");

            //if loop is sleeping just exit out immediately
            if is_sleeping2.load(std::sync::atomic::Ordering::Relaxed) {
                std::process::exit(EXIT_SUCCESS);
            }

            should_interrupt2.store(true, Ordering::Relaxed);
        });

            let players = opt.sync.unwrap();

        loop {
            if players.is_empty() {
                match store.sync_all().await {
                    Ok(_) => {}
                    Err(e) => {
                        print_error("Error syncing.", e);
                        std::process::exit(EXIT_FAILURE);
                    }
                }
            } else {
                match store.sync_players(&players).await {
                    Ok(_) => {}
                    Err(e) => {
                        print_error("Error syncing.", e);
                        std::process::exit(EXIT_FAILURE);
                    }
                }
            }

            if should_interrupt3.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
            
            is_sleeping3.store(true, Ordering::Relaxed);
            println!("Sleeping 15 seconds");
            thread::sleep(delay);
            is_sleeping3.store(false, Ordering::Relaxed);
        }

        println!("Sync Complete");
        std::process::exit(EXIT_SUCCESS);
    }

    if opt.list {
        let members = match store.get_sync_members().await {
            Ok(m) => m,
            Err(e) => {
                print_error("Error syncing.", e);
                std::process::exit(EXIT_FAILURE);
            }
        };

        println!("Synced Players");
        println!("-------------");
        for member in members.iter() {
            println!("{}", member.name.get_bungie_name());
        }
        println!();
    }
}
