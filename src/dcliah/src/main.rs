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

mod startmoment;
use startmoment::StartMoment;

use dcli::apiinterface::ApiInterface;
use dcli::error::Error;
use dcli::mode::ActivityMode;
use dcli::output::Output;
use dcli::platform::Platform;
use dcli::response::activities::Activity;
use chrono::{DateTime, Duration, Utc, ParseError};
use structopt::StructOpt;

use dcli::utils::EXIT_FAILURE;
use dcli::utils::{build_tsv, format_f32, human_duration, print_error, print_verbose};

/*
fn print_tsv(
    data: PvpStatsData,
    member_id: &str,
    character_id: &str,
    platform: Platform,
    mode: CrucibleMode,
    period: TimePeriod,
) {
    let mut name_values: Vec<(&str, String)> = Vec::new();

    name_values.push(("member_id", member_id.to_string()));
    print!("{}", build_tsv(name_values));
}


fn print_default(data: PvpStatsData, mode: CrucibleMode, period: TimePeriod) {

}
*/


fn parse_rfc3339(src: &str) -> Result<DateTime<Utc>, String> {
    let d = match DateTime::parse_from_rfc3339(src) {
        Ok(e) => e,
        Err(_e) => {return Err("Invalid RFC 3339 Date / Time String : Example : 2020-12-08T17:00:00.774187+00:00".to_string())},
    };
    Ok(d.with_timezone(&Utc))
}

async fn retrieve_activities(
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &ActivityMode,
    verbose: bool,
) -> Result<Option<Activity>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let activity: Activity = match client
        .retrieve_last_activity(&member_id, &character_id, &platform, &mode)
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    println!("{:#?}", activity);

    Ok(Some(activity))
}

async fn retrieve_activities_since(
    member_id: &str,
    character_id: &str,
    platform: &Platform,
    mode: &ActivityMode,
    verbose: bool,
) -> Result<Option<Vec<Activity>>, Error> {
    let client: ApiInterface = ApiInterface::new(verbose);

    let date_filter = Utc::now() - Duration::weeks(52 * 6);

    let activities: Vec<Activity> = match client
        .retrieve_activities_since(&member_id, &character_id, &platform, &mode, date_filter)
        .await?
    {
        Some(e) => e,
        None => {
            return Ok(None);
        }
    };

    println!("{:#?}", activities.len());

    Ok(Some(activities))
}

#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
/// Command line tool for retrieving current Destiny 2 activity stats.
///
/// Enables control of which stats are retrieved via game mode, time period and
/// character.
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
    /// This is not the user name, but the member id
    /// retrieved from the Destiny API.
    #[structopt(short = "m", long = "member-id", required = true)]
    member_id: String,

    /// Platform for specified id
    ///
    /// Valid values are: xbox, playstation, stadia or steam.
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id. If not specified, data for all characters
    /// will be returned.
    /// Required when period is set to day, reset, week or month.
    #[structopt(short = "d", long = "start-time", parse(try_from_str = parse_rfc3339), required_if("start-moment", "custom"))]
    start_time: Option<DateTime<Utc>>,
    //required_ifs=&[("start_moment","custom"),]
    //required_if("start-moment", "custom")

    /// Time range to pull stats from
    ///
    /// Valid values include day (last day), reset (since reset), week
    /// (last week), month (last month), alltime (default).
    #[structopt(long = "start-moment" )]
    start_moment: StartMoment,

    // required_ifs=&[("start_moment","custom"),]

    /// Activity mode to return stats for
    ///
    /// Valid values are all (default), control, clash, mayhem, ironbanner,
    /// private, rumble, comp, quickplay and trialsofosiris.
    #[structopt(long = "mode", default_value = "all")]
    mode: ActivityMode,

    /// Format for command output
    ///
    /// Valid values are default (Default) and tsv.
    ///
    /// tsv outputs in a tab (\t) seperated format of name / value pairs with lines
    /// ending in a new line character (\n).
    #[structopt(short = "o", long = "output", default_value = "default")]
    output: Output,

    /// Destiny 2 API character id
    ///
    /// Destiny 2 API character id. If not specified, data for all characters
    /// will be returned.
    /// Required when period is set to day, reset, week or month.
    #[structopt(short = "c", long = "character-id")]
    character_id: String,

    ///Print out additional information
    ///
    ///Output is printed to stderr.
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    print_verbose(&format!("{:#?}", opt), opt.verbose);

    if opt.start_time.is_some() {
        println!("{}", opt.start_time.unwrap());
    }

    //todo: is there any need to send a reference to an enum?
    match retrieve_activities_since(
        &opt.member_id,
        &opt.character_id,
        &opt.platform,
        &opt.mode,
        opt.verbose,
    )
    .await
    {
        Ok(_e) => {}
        Err(e) => {
            print_error("Error Loading Activities", e);
        }
    };
}
