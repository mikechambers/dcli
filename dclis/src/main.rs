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

    #[serde(rename = "Response")]
    response:serde_json::Value,

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

struct Membership {
    membership_platform:Platform,
    membership_id:String,
    cross_save_override_platform:Platform,
    cross_save_override_membership_id:String,
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
    /// User name or steam 64 id in the format 00000000000000000 (17 digit ID)
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

async fn call_api(url: String) -> Result<reqwest::Response, reqwest::Error> {
    let url = Url::parse(&url).unwrap();

    println!("{}", url);

    let client = reqwest::Client::new();

    let resp = match client
        .get(url)
        .header("X-API-Key", DESTINY_API_KEY)
        .send()
        .await
    {
        Ok(e) => e,
        Err(e) => return Err(e),
    };

    Ok(resp)
}

async fn retrieve_member_id_from_steam(steam_id:&String) -> Result<String, ApiCallError> {
    let url = format!(
        "https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/12/{steam_id}/",
        steam_id = utf8_percent_encode(&steam_id, NON_ALPHANUMERIC),
    );

    let resp = match call_api(url).await {
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
    Ok(String::from(resp.response["membershipId"].as_str().unwrap()))
    //Ok(String::from("MEMBER_ID_FROM_STEAM"))
}

async fn retrieve_member_id(id: &String, platform: Platform) -> Result<String, ApiCallError> {

    if platform == Platform::Steam {
        return retrieve_member_id_from_steam(&id).await;
    }

    //TODO: add input
    //TODO: urlencode input
    //TODO:: need to branch for steam
    let url = format!(
        "https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/{platform_id}/{id}/",
        platform_id = platform.to_id(),
        id = utf8_percent_encode(&id, NON_ALPHANUMERIC),
    );

    //custom header
    //TODO: handle parsing error

    let resp = match call_api(url).await {
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

fn is_valid_steam_id(steam_id:&String) -> bool {

    //make sure it can be parsed into a u64
    let parses = match steam_id.parse::<u64>() {
        Ok(e) => true,
        Err(_e) => false,
    };

    parses && steam_id.chars().count() == 17
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    println!(
        "Searching for {id} on {platform}",
        id = opt.id,
        platform = opt.platform,
    );

    if opt.platform == Platform::Steam {
        if !is_valid_steam_id(&opt.id) {
            println!("{}", "Invalid steam 64 id.");
            std::process::exit(1);
        }
    }

    let member_id = retrieve_member_id(&opt.id, opt.platform).await;

    let member_id = match member_id {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e.message);
            //TODO: can we just return here?
            std::process::exit(1);
        }
    };

    //TODO: compare original input to what was returned to make sure we got an exact
    //match

    //println!("Data Loaded : Error Status {:?}", resp);

    println!("Member ID: {}", member_id);

    Ok(())
}
