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
use std::str::FromStr;

use dcli::crucible::CrucibleActivity;
use dcli::{enums::platform::Platform, utils::human_duration};

use dcli::enums::mode::Mode;
use dcli::manifestinterface::ManifestInterface;

use dcli::enums::character::CharacterClassSelection;

use dcli::activitystoreinterface::ActivityStoreInterface;

use dcli::utils::{determine_data_dir, format_f32, human_date_format, repeat_str};

use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_verbose};
use structopt::StructOpt;

fn parse_and_validate_mode(src: &str) -> Result<Mode, String> {
    let mode = Mode::from_str(src)?;

    if !mode.is_crucible() {
        return Err(format!("Unsupported mode specified : {}", src));
    }

    Ok(mode)
}

fn generate_score(data: &CrucibleActivity) -> String {
    let mut tokens: Vec<String> = Vec::new();

    for (_k, t) in &data.teams {
        tokens.push(t.score.to_string());
        tokens.push("-".to_string());
    }

    tokens.pop();

    tokens.join("")
}

fn print_default(data: &CrucibleActivity, member_id: &str, detailed: bool) {
    let col_w = 10;
    let name_col_w = 18;

    let member_performance = data.get_member_performance(member_id).unwrap();

    let team_title_border = repeat_str("-", name_col_w + col_w);
    let activity_title_border = repeat_str("=", name_col_w + col_w + col_w);

    println!("ACTIVITY");
    println!("{}", activity_title_border);

    println!(
        "{} on {} :: {} ({})",
        data.details.mode,
        data.details.map_name,
        human_date_format(&data.details.period),
        human_duration(member_performance.stats.activity_duration_seconds)
    );

    println!("{}", generate_score(data));
    println!("{}!", member_performance.stats.standing);
    println!();

    let header = format!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
    "PLAYER",
    "KILLS",
    "ASTS",
    "K+A",
    "DEATHS",
    "K/D",
    "KD/A",
    "EFF",
    "SUPERS",
    "GRENADES",
    "MELEES",
    "MEDALS",
    "STATUS",
    col_w=col_w,
    name_col_w = name_col_w,
    );

    let table_width = header.chars().count();
    let header_border = repeat_str("=", table_width);
    let entry_border = repeat_str(".", table_width);

    for (_k, v) in &data.teams {
        println!("[{}] {} Team {}", v.score, v.display_name, v.standing);
        println!("{}", team_title_border);
        println!("{}", header);
        println!("{}", header_border);

        for p in &v.player_performances {
            let extended = p.stats.extended.as_ref().unwrap();
            println!("{:<0name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}",
                p.player.display_name,
                p.stats.kills.to_string(),
                p.stats.assists.to_string(),
                p.stats.opponents_defeated.to_string(),
                p.stats.deaths.to_string(),
                format_f32(p.stats.kills_deaths_ratio, 2),
                format_f32(p.stats.kills_deaths_assists, 2),
                format_f32(p.stats.efficiency, 2),
                extended.weapon_kills_super.to_string(),
                extended.weapon_kills_grenade.to_string(),
                extended.weapon_kills_ability.to_string(),
                extended.all_medals_earned.to_string(),
                p.stats.generate_status(),
                col_w=col_w,
                name_col_w = name_col_w,
            );

            //todo: what if they dont have weapon kills (test)
            if detailed {
                println!("{}", entry_border);

                for w in &extended.weapons {
                    println!(
                        "{:>0w_name_col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w$}{:>0col_w2$}",
                        w.weapon.name,
                        w.kills.to_string(),
                        w.precision_kills.to_string(),
                        format!("{}%", format_f32(w.precision_kills_percent * 100.0, 0)),
                        format!("{}", w.weapon.item_sub_type),
                        w_name_col_w = col_w + col_w + name_col_w,
                        col_w = col_w,
                        col_w2 = col_w * 2,
                    );
                }
                println!();
            }
        }
        //println!("{}", header_border);
        //println!("{}", header);
        println!();
    }

    //PLAYER      KILLS       ASTS     K+A   DEATHS      K/D         KD/A       EFF         SUPERS  GRENADES    MELEES    MEDALS  STATUS
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving and viewing Destiny 2 Crucible activity details.
///
/// Display player details for individual Crucible games.
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
    /// This is not the user name, but the member id retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Activity mode to return stats for
    ///
    /// Supported values are all_pvp (default), control, clash, elimination,
    /// mayhem, iron_banner, private, rumble, pvp_competitive,
    /// quickplay and trials_of_osiris.
    ///
    /// Addition values available are crimsom_doubles, supremacy, survival,
    /// countdown, all_doubles, doubles, private_clash, private_control,
    /// private_survival, private_rumble, showdown, lockdown,
    /// scorched, scorched_team, breakthrough, clash_quickplay, trials_of_the_nine
    #[structopt(long = "mode", short = "M", 
        parse(try_from_str=parse_and_validate_mode), default_value = "all_pvp")]
    mode: Mode,

    /// Character to retrieve data for.
    ///
    /// Valid values include hunter, titan, warlock, last_active and all.
    #[structopt(short = "C", long = "class", default_value = "last_active")]
    character_class_selection: CharacterClassSelection,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Don't sync activities.
    ///
    /// If flag is set, activities will not be retrieved before displaying stats.
    /// This is useful in case you are syncing activities in a seperate process.
    #[structopt(short = "N", long = "no-sync")]
    no_sync: bool,

    /// Only display a expanded view of activity details.
    ///
    /// If flag is set, additional informationwill be displayed, including weapon stats
    /// and match overview data.
    #[structopt(short = "d", long = "detailed")]
    detailed: bool,

    /// Directory where Destiny 2 manifest and activity database files are stored. (optional)
    ///
    /// This will normally be downloaded using the dclim and dclias tools, and uses
    /// a system appropriate directory by default.
    #[structopt(short = "D", long = "data-dir", parse(from_os_str))]
    data_dir: Option<PathBuf>,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    let data_dir = match determine_data_dir(opt.data_dir) {
        Ok(e) => e,
        Err(e) => {
            print_error("Error initializing manifest directory.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut store = match ActivityStoreInterface::init_with_path(&data_dir, opt.verbose).await {
        Ok(e) => e,
        Err(e) => {
            print_error(
                "Could not initialize activity store. Have you run dclias?",
                e,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut manifest = match ManifestInterface::new(&data_dir, false).await {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not initialize manifest. Have you run dclim?", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    if !opt.no_sync {
        match store.sync(&opt.member_id, &opt.platform).await {
            Ok(_e) => (),
            Err(e) => {
                eprintln!("Could not sync activity store {}", e);
                eprintln!("Using existing data");
            }
        };
    }

    let data = match store
        .retrieve_last_activity(
            &opt.member_id,
            &opt.platform,
            &opt.character_class_selection,
            &opt.mode,
            &mut manifest,
        )
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error("Could not retrieve data from activity store.", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    print_default(&data, &opt.member_id, opt.detailed);
}
