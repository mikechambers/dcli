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

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::platform::Platform;
use dcli::utils::{print_error, print_standard};

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
    /// #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id for the chracter to retrieve status on.
    /// #[structopt(short = "m", long = "member-id", required = true)]
    character_id: String,    

    ///terse output in the form of class_name:character_id . Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information for the API call
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    //"/Platform/Destiny2/1/Profile/4611686018429783292/?components=200,202,204,1000";
    //components 204
    //currentActivityModeTypes : will be avaliable if player is online

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
