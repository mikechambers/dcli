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
use dcli::error::Error;
use dcli::mode::CrucibleMode;
use dcli::platform::Platform;
use dcli::response::stats::PvpStatsData;

use dcli::utils::EXIT_FAILURE;
use dcli::utils::{print_error, print_standard};



#[derive(StructOpt)]
/// Command line tool for retrieving current Destiny 2 activity for player.
///
/// 
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

    ///  Crucible mode to return stats for.
    ///
    /// Crucible mode to return stats for. Valid values are all (default), 
    /// control, clash, mayhem, ironbanner, private, trialsofnine, rumble, 
    /// comp, quickplay and trialsofosiris
    #[structopt(long = "mode")]
    mode: Option<CrucibleMode>,

    /// Destiny 2 API character id. If not specified, data for all characters will be returned.
    ///
    /// Destiny 2 API character id. If not specified, data for all characters will be returned.
    #[structopt(short = "c", long = "character-id")]
    character_id: Option<String>,

    ///Terse output. Errors are suppresed.
    #[structopt(short = "t", long = "terse", conflicts_with = "verbose")]
    terse: bool,

    ///Print out additional information
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

async fn retrieve_all_time_stats (
    member_id: String,
    character_id: String,
    platform: Platform,
    mode: CrucibleMode,
    verbose: bool,
) -> Result<PvpStatsData, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let data: PvpStatsData = client
        .retrieve_alltime_crucible_stats(member_id, character_id, platform, mode)
        .await?;

    Ok(data)
}

#[tokio::main]
async fn main() {

    let opt = Opt::from_args();

    let character_id:String = opt.character_id.unwrap_or("0".to_string());
    let mode:CrucibleMode = opt.mode.unwrap_or(CrucibleMode::AllPvP);

    let data: PvpStatsData = match retrieve_all_time_stats(
        opt.member_id,
        character_id,
        opt.platform,
        mode,
        opt.verbose
    ).await {
        Ok(_e) => _e,
        Err(e) => {
            print_standard(&format!("Error : {:#?}", e), true);
            std::process::exit(EXIT_FAILURE);
        },
    };

    println!("{:#?}", data);
}
