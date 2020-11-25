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

    println!("{:?}", activity_data_a.current_activity_mode_type);
    //lets find out the mode / activity type name
    let activity_type_name: String = match activity_data_a.current_activity_mode_type {

        //if its set in the API data, we use that
        Some(e) => format!("{}", e),
        None => {

            //otherwise, we go into the manifest to find it
            match manifest
                .get_activity_type_definition(activity_data_m.activity_type_hash)
                .await
            {
                Ok(e) => e.display_properties.name,
                Err(_e) => {
                    //Todo: this either means an error, unknown activity, or they are in orbit
                    "Unknown".to_string()
                }
            }
        }
    };

    println!(
        "Playing {mode} on {place0}{place1} ({description})",
        //if we get to this point, mode should never be none, so it should be safe to unwrap
        mode = activity_type_name,
        place0 = activity_data_m.display_properties.name,
        place1 = place_data_m.display_properties.name,
        description = activity_data_m.display_properties.description,
    );

    println!("{: <15}: {: <10}", "Mode", activity_type_name);
    println!(
        "{: <15}: {: <10}",
        "Activity", activity_data_m.display_properties.name
    );
    println!(
        "{: <15}: {: <10}",
        "Place", place_data_m.display_properties.name
    );
    println!(
        "{: <15}: {: <10}",
        "Description", activity_data_m.display_properties.description
    );
    println!(
        "{: <15}: {: <10}",
        "Destination", destination_data_m.display_properties.name
    );

    //Playing Strike on The Insight Terminus (Break into the ancient Vex installation.)

    //need to get destinationHash (DestinyDestinationDefinition) (Arcadian Valley) and placeHash (DestinyPlaceDefinition) hashes (Nessus)

    //todo: just return raw data from manifest, dont try to make it clean object

    Ok(())
}

/*

    "Response": {
        "characterActivities": {
            "data": {
                "2305843009264966984": {


                    "currentActivityHash": 1813752023, //destination (will be 0 if not active)
                    "currentActivityModeHash": 3497767639, //activity (will be 0 if not active)
                    "currentActivityModeType": 6, //activity (patrol)
                    "currentActivityModeHashes": [
                        3497767639, //activty
                        1164760493 //pve
                    ],
                    "currentActivityModeTypes": [
                        6, //patrol
                        7 //AllPVE
                    ],
                    "currentPlaylistActivityHash": 1813752023, //destination
                    "lastCompletedStoryHash": 0

                    //look for presence of currentActivityModeTypes

                    //can also see if anyone is in playlist with them.

        //output (make fireteam optional)
        //Playing patrol on Europa with mesh, foo and Bar.

*/
