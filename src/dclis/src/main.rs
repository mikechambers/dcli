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

mod memberidsearch;

use dcli::platform::Platform;
use dcli::utils::{print_error, print_verbose, TSV_EOL, TSV_DELIM, EXIT_FAILURE};
use memberidsearch::MemberIdSearch;
use dcli::output::Output;
use memberidsearch::Membership;

use structopt::StructOpt;

fn is_valid_steam_id(steam_id: &str) -> bool {
    //make sure it can be parsed into a u64
    let parses = match steam_id.parse::<u64>() {
        Ok(_e) => true,
        Err(_e) => false,
    };

    parses && steam_id.chars().count() == 17
}

#[derive(StructOpt)]
/// Command line tool for retrieving primary Destiny 2 member ids.
///
/// Retrieves the primary Destiny 2 membershipId and platform for specified 
/// username or steam 64 id and platform. That may be a membershipId on a platform 
/// different that the one specified, depending on the cross save status of the 
/// account. It will return the primary membershipId that all data will be 
/// associate with.
/// 
/// Created by Mike Chambers.
/// https://www.mikechambers.com
/// 
/// Released under an MIT License.
/// More info at: https://github.com/mikechambers/dcli
struct Opt {
    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    #[structopt(short = "i", long = "id", required = true)]
    /// User name or steam 64 id
    ///
    /// User name (for Xbox, Playstation or Stadia) or steam 64 id for Steam / pc :
    /// 00000000000000000 (17 digit ID) for steam.
    id: String,


    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of columns with lines
    /// ending in a new line character (\n).
    #[structopt(short = "o", long = "output", default_value = "default")]
    output: Output,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    if opt.platform == Platform::Steam && !is_valid_steam_id(&opt.id) {
        println!("Invalid steam 64 id. Must be a 17 digit Steam 64 ID.");
        return;
    }

    print_verbose(
        &format!(
            "Searching for '{id}' on {platform}",
            id = opt.id,
            platform = opt.platform,
        ),
        opt.verbose,
    );

    let member_search = MemberIdSearch::new(opt.verbose);

    let membership = member_search
        .retrieve_member_id(&opt.id, opt.platform)
        .await;

    let membership = match membership {
        Some(e) => match e {
            Ok(e) => e,
            Err(e) => {
                print_error("Error retrieving ID from API.", e);
                std::process::exit(EXIT_FAILURE);
            }
        },
        None => {
            println!("Member not found");
            return;
        }
    };

    if opt.platform != Platform::Steam {
        match membership.display_name {
            Some(ref e) => {
                if e != &opt.id {
                    println!("Member not found");
                    return;
                }
            },
            None => {
                println!("Member not found");
                return;
            },
        };
    }

    let mut name = None;
    if membership.platform != Platform::Steam {
        name = Some(&opt.id);
    }

    match opt.output {
        Output::Default => {
            print_default(&membership, name);
        },
        Output::Tsv => {
            print_tsv(&membership, name);
        },
    }
    
}


fn print_tsv(member:&Membership, name:Option<&String>) {

    let default = &"".to_string();
    let n = name.unwrap_or_else(||default);

    print!("{d}{delim}{i}{delim}{p}{delim}{pi}{eol}",
    d=n,
    i=member.id,
    p=member.platform,
    pi=member.platform.to_id(),
    delim=TSV_DELIM,
    eol=TSV_EOL,
    );
}


fn print_default(member:&Membership, name:Option<&String>) {

    let default = &"".to_string();
    let n = name.unwrap_or_else(||default);

    let col_w = 15;
    println!("{:<0col_w$}{}", "Display Name", n, col_w=col_w);
    println!("{:<0col_w$}{}", "id", member.id, col_w=col_w);
    println!("{:<0col_w$}{}", "Platform", member.platform, col_w=col_w);
    println!("{:<0col_w$}{}", "Platform Id", member.platform.to_id(), col_w=col_w);
}
