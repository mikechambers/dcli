mod platform;
use platform::Platform;

mod apiclient;
use apiclient::ApiClient;

use exitfailure::ExitFailure;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct DestinySearchResponse {
    #[serde(rename = "Response")]
    response: Vec<DestinyResponseMember>,

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

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponseSteam {
    #[serde(rename = "Response")]
    response: DestinyResponseMember,

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

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponseMember {

    #[serde(rename = "membershipType")]
    membership_type: u64,

    #[serde(rename = "membershipId")]
    membership_id: String,
}

struct Membership {
    platform: Platform,
    id: String,
}

#[derive(StructOpt)]
/// Command line tool for retrieving primary Destiny 2 member ids.
///
/// Retrieves the primary Destiny 2 membershipId and platform for specified username or
/// steam 64 id and platform. That may a membershipId on a platform different
/// that the one specified, depending on the cross save status of the account. It
/// will return the primary membershipId that all data will be associate with.
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

async fn retrieve_member_id_from_steam(
    steam_id: &String,
) -> Option<Result<Membership, ApiCallError>> {
    let url = format!(
        "https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/12/{steam_id}/",
        steam_id = utf8_percent_encode(&steam_id, NON_ALPHANUMERIC),
    );

    let resp = match ApiClient::new().call_api(url).await {
        Ok(e) => e,
        Err(e) => {
            return Some(Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Request,
            }))
        }
    };

    let resp = match resp.json::<DestinyResponseSteam>().await {
        Ok(e) => e,
        Err(e) => {
            return Some(Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Parse,
            }))
        }
    };

    let m = Membership {
        id: resp.response.membership_id,
        platform: Platform::from_id(resp.response.membership_type),
    };

    Some(Ok(m))
}

async fn retrieve_member_id(
    id: &String,
    platform: Platform,
) -> Option<Result<Membership, ApiCallError>> {
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

    let resp = match ApiClient::new().call_api(url).await {
        Ok(e) => e,
        Err(e) => {
            return Some(Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Request,
            }))
        }
    };

    let resp = match resp.json::<DestinySearchResponse>().await {
        Ok(e) => e,
        Err(e) => {
            return Some(Err(ApiCallError {
                message: get_request_error_message(e),
                error_type: ApiCallErrorType::Parse,
            }))
        }
    };

    let results:Vec<DestinyResponseMember> = resp.response;
    if results.len() == 0 {
        return None;
    }

    let m = Membership {
        id:String::from(results[0].membership_id.as_str()),
        platform: Platform::from_id(results[0].membership_type),
    };

    Some(Ok(m))
}

fn get_request_error_message(error: reqwest::Error) -> String {
    format!("{}", error)
}

fn is_valid_steam_id(steam_id: &String) -> bool {
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

    let membership = retrieve_member_id(&opt.id, opt.platform).await;

    let membership = match membership {
        Some(e) => match e {
            Ok(e) => e,
            Err(e) => {
                println!("{}", e.message);
                //TODO: can we just return here?
                std::process::exit(1);
            }
        },
        None => {
            println!("Member not found");
            std::process::exit(0);
        }
    };

    //TODO: compare original input to what was returned to make sure we got an exact
    //match

    //println!("Data Loaded : Error Status {:?}", resp);

    println!("membershipId: {}", membership.id);
    println!("platform: {0} ({1})", membership.platform, membership.platform.to_id());

    Ok(())
}
