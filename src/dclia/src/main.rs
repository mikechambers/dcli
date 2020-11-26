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

use exitfailure::ExitFailure;
use structopt::StructOpt;

use dcli::apiinterface::ApiInterface;
//use dcli::error::Error;
use dcli::manifestinterface::ManifestInterface;
use dcli::mode::Mode;
use dcli::platform::Platform;
use dcli::utils::{print_error, print_standard};

use dcli::manifest::definitions::{
    ActivityDefinitionData, DestinationDefinitionData, PlaceDefinitionData,
};
use dcli::response::gpr::CharacterActivitiesData;

use std::path::PathBuf;

const ORBIT_PLACE_HASH: u32 = 2961497387;

#[derive(StructOpt)]
/// Command line tool for retrieving current Destiny 2 activity for player..
///
/// Command line tool for retrieving character information for specified member id
/// Retrieves character information for the specified member id.
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

    ///terse output in the form of class_name:character_id . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    ///Local path the Destiny 2 manifest database file.
    #[structopt(long = "manifest-path", parse(from_os_str))]
    manifest_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    //TODO: why does this have to be mutable
    let mut client = ApiInterface::new(opt.verbose);

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
            std::process::exit(1);
            //Err(failure::err_msg("root cause failure"));
        }
    };

    let activity_data_a = match activities_data {
        Some(e) => e,
        None => {
            print_standard("Not currently in an activity.", true);
            std::process::exit(1);
        }
    };

    let mut manifest = match ManifestInterface::new(opt.manifest_path, false).await {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Manifest Error : {:?}", e), !opt.terse);
            std::process::exit(0);
        }
    };

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
            std::process::exit(0);
        }
    };

    if activity_data_m.place_hash == ORBIT_PLACE_HASH {
        print_standard("Currently sitting in Orbit", true);
        return Ok(());
    }

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
            std::process::exit(0);
        }
    };

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
            std::process::exit(0);
        }
    };

    let mut mode = Mode::None;

    //lets find out the mode / activity type name
    let activity_type_name: String = match activity_data_a.current_activity_mode_type {
        //if its set in the API data, we use that
        Some(e) => {
            mode = e;
            format!("{}", e)
        }
        None => {
            //otherwise, we go into the manifest to find it
            match manifest
                .get_activity_type_definition(activity_data_m.activity_type_hash)
                .await
            {
                Ok(e) => e.display_properties.name,
                Err(_e) => {
                    println!("{:?}", _e);
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

    Ok(())
}
