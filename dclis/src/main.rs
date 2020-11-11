mod platform;

use exitfailure::ExitFailure;
use platform::Platform;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

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

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();
    println!(
        "Searching for {id} on {platform}",
        id = opt.id,
        platform = opt.platform,
    );

    //TODO: add input
    //TODO: urlencode input
    //TODO:: need to branch for steam
    let url = format!("https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/{platform_id}/{id}/",
        platform_id=opt.platform.to_id(),
        id=utf8_percent_encode(&opt.id, NON_ALPHANUMERIC),
);

    println!("{}", url);

    //custom header
    let url = Url::parse(&url)?;

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("X-API-Key", DESTINY_API_KEY)
        .send()
        .await;

    let resp = match resp {
        Ok(e) => e,
        Err(e) => panic!("ERROR calling API : {}", e),
    };

    let resp = resp.json::<DestinyResponse>().await.unwrap_or_else(|err| {
        panic!("Error Parsing JSON Response : {}", err);
    });

    //TODO: compare original input to what was returned to make sure we got an exact
    //match

    //println!("Data Loaded : Error Status {:?}", resp);

    println!("Data Loaded : Error Status {:?}", resp.error_status);

    Ok(())
}
