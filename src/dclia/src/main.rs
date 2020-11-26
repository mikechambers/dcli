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

use structopt::StructOpt;

use dcli::apiinterface::ApiInterface;
//use dcli::error::Error;
use dcli::manifestinterface::ManifestInterface;
use dcli::mode::Mode;
use dcli::platform::Platform;
use dcli::utils::{print_error, print_standard};
use dcli::utils::EXIT_FAILURE;

use dcli::manifest::definitions::{
    ActivityDefinitionData, DestinationDefinitionData, PlaceDefinitionData,
};
use dcli::response::gpr::CharacterActivitiesData;

use std::path::PathBuf;

const ORBIT_PLACE_HASH: u32 = 2961497387;

#[derive(StructOpt)]
/// Command line tool for retrieving current Destiny 2 activity for player.
///
/// Command line tool for retrieving current Destiny 2 activity for player,
/// including activity, location, and map for PVP modes (Crucible and Gambit).
struct Opt {
    /// Platform for specified id
    ///
    /// Platform for specified member id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Destiny 2 API member id
    ///
    /// Destiny 2 API member id. This is not the user name, but the member id
    /// retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    ///Terse output. Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///Local path for the Destiny 2 manifest database file.
    #[structopt(long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    //TODO: why does this have to be mutable
    let mut client = ApiInterface::new(opt.verbose);

    print_standard("Calling API to retrieve current activity.", opt.verbose);
    let activities_data: Option<CharacterActivitiesData> = match client
        .retrieve_current_activity(opt.member_id, opt.platform)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error(
                &format!("Error retrieving data from API : {:?}", e),
                !opt.terse,
            );
            std::process::exit(EXIT_FAILURE);
            //Err(failure::err_msg("root cause failure"));
        }
    };

    let activity_data_a = match activities_data {
        Some(e) => e,
        None => {
            print_standard("Not currently in an activity.", true);
            return;
        }
    };

    print_standard("Initializing Manifest.", opt.verbose);
    let mut manifest = match ManifestInterface::new(opt.manifest_path, false).await {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Manifest Error : {:?}", e), !opt.terse);
            std::process::exit(EXIT_FAILURE);
        }
    };

    print_standard(&format!("Getting activity definition data from manifest : {}", activity_data_a.current_activity_hash), opt.verbose);
    let activity_data_m: ActivityDefinitionData = match manifest
        .get_activity_definition(activity_data_a.current_activity_hash)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error(
                &format!("Error Retrieving Data from Manifest : {:?}", e),
                !opt.terse,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    if activity_data_m.place_hash == ORBIT_PLACE_HASH {
        print_standard("Currently sitting in Orbit", true);
    }

    print_standard(&format!("Getting place definition data from manifest : {}", activity_data_m.place_hash), opt.verbose);
    let place_data_m: PlaceDefinitionData = match manifest
        .get_place_definition(activity_data_m.place_hash)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error(
                &format!("Error Retrieving Data from Manifest : {:?}", e),
                !opt.terse,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    print_standard(&format!("Getting destination definition data from manifest : {}", activity_data_m.destination_hash), opt.verbose);
    let destination_data_m: DestinationDefinitionData = match manifest
        .get_destination_definition(activity_data_m.destination_hash)
        .await
    {
        Ok(e) => e,
        Err(e) => {
            print_error(
                &format!("Error Retrieving Data from Manifest : {:?}", e),
                !opt.terse,
            );
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut mode = Mode::None;

    //lets find out the mode / activity type name
    print_standard("Determining activity mode", opt.verbose);
    let activity_type_name: String = match activity_data_a.current_activity_mode_type {
        //if its set in the API data, we use that
        Some(e) => {
            mode = e;
            format!("{}", e)
        }
        None => {
            print_standard(&format!("Activity mode not returned from API. Checking Manifest : {}", activity_data_m.activity_type_hash), opt.verbose);
            //otherwise, we go into the manifest to find it
            match manifest
                .get_activity_type_definition(activity_data_m.activity_type_hash)
                .await
            {
                Ok(e) => e.display_properties.name,
                Err(e) => {
                    print_standard(&format!("Activity Mode not found in Manifest : {:?}", e), opt.verbose);
                    //println!("{:?}", _e);
                    //Todo: this either means an error, unknown activity, or they are in orbit
                    "Unknown".to_string()
                }
            }
        }
    };

    let description = activity_data_m
        .display_properties
        .description
        .unwrap_or("".to_string());
    let activity_name = activity_data_m.display_properties.name;
    let place_name = place_data_m.display_properties.name;
    let destination_name = destination_data_m.display_properties.name;

    let out = if mode == Mode::Patrol {
        format!("Exploring on {}", place_name)
    } else if mode.is_gambit() {
        format!(
            "Playing {} on {} ({})",
            activity_type_name, activity_name, description
        )
    } else if mode.is_crucible() {
        format!(
            "Playing {} on {} ({})",
            activity_type_name, activity_name, description
        )
    } else if mode == Mode::Strike {
        format!(
            "Running {} {} on {}",
            activity_name, activity_type_name, place_name
        )
    } else if mode == Mode::Social {
        format!("Hanging out in {} on {}", activity_name, place_name)
    } else if mode == Mode::Story {
        format!("Playing {} story on {}", activity_name, place_name)
    } else if mode.is_nightfall() {
        format!(
            "Playing {} {} on {}",
            description, activity_name, place_name
        )
    } else {
        format!(
            "Playing {} {} on {}",
            activity_name, activity_type_name, place_name
        )
    };

    print_standard(&out, !opt.terse);

    print_standard(
        &format!(
            "Mode:{}\nActivity:{}\nPlace:{}\nDestination:{}\nDescription:{}",
            activity_type_name, activity_name, place_name, destination_name, description
        ),
        opt.terse,
    );
}
