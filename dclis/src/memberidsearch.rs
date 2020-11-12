use crate::apiclient::{ApiCallError, ApiCallErrorType, ApiClient};
use crate::platform::Platform;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use serde_derive::{Deserialize, Serialize};

pub struct MemberIdSearch {
    print_url: bool,
    client: ApiClient,
}

impl MemberIdSearch {
    pub fn new(print_url: bool) -> MemberIdSearch {
        MemberIdSearch {
            print_url,
            client: ApiClient::new(),
        }
    }

    pub async fn retrieve_member_id_from_steam(
        &self,
        steam_id: &str,
    ) -> Option<Result<Membership, ApiCallError>> {
        let url = format!(
            "https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/12/{steam_id}/",
            steam_id = utf8_percent_encode(&steam_id, NON_ALPHANUMERIC),
        );

        let resp = match self.client.call_api(url).await {
            Ok(e) => e,
            Err(e) => {
                return Some(Err(ApiCallError {
                    message: get_request_error_message(e),
                    _error_type: ApiCallErrorType::Request,
                }))
            }
        };

        let resp = match resp.json::<DestinyResponseSteam>().await {
            Ok(e) => e,
            Err(e) => {
                return Some(Err(ApiCallError {
                    message: get_request_error_message(e),
                    _error_type: ApiCallErrorType::Parse,
                }))
            }
        };

        let m = Membership {
            id: resp.response.membership_id,
            platform: Platform::from_id(resp.response.membership_type),
        };

        Some(Ok(m))
    }

    pub async fn retrieve_member_id(
        &self,
        id: &str,
        platform: Platform,
    ) -> Option<Result<Membership, ApiCallError>> {
        if platform == Platform::Steam {
            return self.retrieve_member_id_from_steam(&id).await;
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

        let resp = match self.client.call_api(url).await {
            Ok(e) => e,
            Err(e) => {
                return Some(Err(ApiCallError {
                    message: get_request_error_message(e),
                    _error_type: ApiCallErrorType::Request,
                }))
            }
        };

        let resp = match resp.json::<DestinySearchResponse>().await {
            Ok(e) => e,
            Err(e) => {
                return Some(Err(ApiCallError {
                    message: get_request_error_message(e),
                    _error_type: ApiCallErrorType::Parse,
                }))
            }
        };

        let results: Vec<DestinyResponseMember> = resp.response;
        if results.is_empty() {
            return None;
        }

        let m = Membership {
            id: String::from(results[0].membership_id.as_str()),
            platform: Platform::from_id(results[0].membership_type),
        };

        Some(Ok(m))
    }
}

fn get_request_error_message(error: reqwest::Error) -> String {
    format!("{}", error)
}

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

pub struct Membership {
    pub platform: Platform,
    pub id: String,
}
