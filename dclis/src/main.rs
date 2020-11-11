mod platform;

use exitfailure::ExitFailure;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use platform::Platform;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

const DESTINY_API_KEY: &'static str = env!("DESTINY_API_KEY");

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponse {
    #[serde(rename = "ErrorCode")]
    error_code: u32,

    #[serde(rename = "ThrottleSeconds")]
    throttle_seconds: u32,

    #[serde(rename = "ErrorStatus")]
    error_status: String,

    #[serde(rename = "Message")]
    message: String,
    //MessageData : {}
}

#[derive(StructOpt)]
/// Command line tool for retrieving Destiny 2 member ids.
///
/// Retrieves the Destiny 2 member id for specified username or
/// steam 64 id and platform.
struct Opt {
    //exitfailure = "0.5.1"/ Platform for specified id
    ///
    /// Platform for specified id. Valid values are:
    /// xbox, playstation, stadia or steam
    #[structopt(short = "p", long = "platform", required = true)]
    platform: Platform,

    #[structopt(short = "i", long = "id", required = true)]
    /// User name or steam 64 id
    ///
    /// User name or steam 64 idr
    id: String,
    //#[structopt(required = false)]
    //verbose:bool,

    //#[structopt(required = false)]
    //json:bool,
}

struct ApiCallError {
    message: String,
    error_type: ApiCallErrorType,
}

enum ApiCallErrorType {
    Request,
    Parse,
}

async fn retrieve_member_id(id: String, platform: Platform) -> Result<String, ApiCallError> {
    //TODO: add input
    //TODO: urlencode input
    //TODO:: need to branch for steam
    let url = format!(
        "https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/{platform_id}/{id}/",
        platform_id = platform.to_id(),
        id = utf8_percent_encode(&id, NON_ALPHANUMERIC),
    );

    println!("{}", url);

    //custom header
    //TODO: handle parsing error
    let url = Url::parse(&url).unwrap();

    let client = reqwest::Client::new();

    let resp = match client
        .get(url)
        .header("X-API-Key", DESTINY_API_KEY)
        .send()
        .await
    {
        Ok(e) => e,
        Err(e) => {
            return Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Request,
            })
        }
    };

    let resp = match resp.json::<DestinyResponse>().await {
        Ok(e) => e,
        Err(e) => {
            return Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Parse,
            })
        }
    };

    ////TODO: parse member_id here
    Ok(String::from("MEMBER_ID"))
}

fn get_request_error_message(error: reqwest::Error) -> String {
    format!("{}", error)

    /*
    if error.is_http() {

    } else if error.is_serialization() {

    } else if error.is_redirect() {

    } else if error.is_client_error() {

    } else if error.is_server_error() {

    } else {

    }
    */
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    println!(
        "Searching for {id} on {platform}",
        id = opt.id,
        platform = opt.platform,
    );

    let member_id = retrieve_member_id(opt.id, opt.platform).await;

    let member_id = match member_id {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e.message);
            std::process::exit(1);
        },
    };

    //TODO: compare original input to what was returned to make sure we got an exact
    //match

    //println!("Data Loaded : Error Status {:?}", resp);

    println!("Member ID: {}", member_id);

    Ok(())
}
