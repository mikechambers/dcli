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
use dcli::utils::{print_error, print_standard};
use memberidsearch::MemberIdSearch;

use exitfailure::ExitFailure;

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
/// Retrieves the primary Destiny 2 membershipId and platform for specified username or
/// steam 64 id and platform. That may a membershipId on a platform different
/// that the one specified, depending on the cross save status of the account. It
/// will return the primary membershipId that all data will be associate with.
struct Opt {
    /// Platform for specified id
    ///
    /// Platform for specified id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    #[structopt(short = "i", long = "id", required = true)]
    /// User name or steam 64 id
    ///
    /// User name (for Xbox, Playstation or Stadia) or steam 64 id :
    /// 00000000000000000 (17 digit ID)
    id: String,

    ///terse output in the form of membership_id:platform . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    if opt.id.chars().count() == 0 {
        print_error("Invalid id.", !opt.terse);
        std::process::exit(1);
    }

    if opt.platform == Platform::Steam && !is_valid_steam_id(&opt.id) {
        print_error("Invalid steam 64 id.", !opt.terse);
        std::process::exit(1);
    }

    print_standard(
        &format!(
            "Searching for '{id}' on {platform}",
            id = opt.id,
            platform = opt.platform,
        ),
        opt.verbose && !opt.terse,
    );

    let member_search = MemberIdSearch::new(opt.verbose && !opt.terse);

    let membership = member_search
        .retrieve_member_id(&opt.id, opt.platform)
        .await;

    let membership = match membership {
        Some(e) => match e {
            Ok(e) => e,
            Err(e) => {
                print_error(&format!("Error calling API : {}", e), !opt.terse);
                std::process::exit(1);
            }
        },
        None => {
            print_standard(
                &format!("Member not found : '{}' on {}", opt.id, opt.platform),
                !opt.terse,
            );
            std::process::exit(0);
        }
    };

    if opt.platform != Platform::Steam {
        let display_name = match membership.display_name {
            Some(e) => e,
            None => String::from(""),
        };

        if display_name != opt.id {
            print_standard(
                &format!(
                    "Member not found : Searched for '{}' : Found '{}'",
                    opt.id, display_name
                ),
                !opt.terse,
            );
            std::process::exit(0);
        }
    }

    print_standard(
        &format!(
            "{membership_id}:{platform_id}",
            membership_id = membership.id,
            platform_id = membership.platform
        ),
        opt.terse,
    );

    if membership.platform != Platform::Steam {
        print_standard(&format!("Display Name  : {}", opt.id), !opt.terse);
    }

    print_standard(
        &format!(
            "Membership Id : {membership_id}\nPlatform      : {platform}",
            membership_id = membership.id,
            platform = membership.platform
        ),
        !opt.terse,
    );

    Ok(())
}
