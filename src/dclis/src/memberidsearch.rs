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

use dcli::apiclient::ApiClient;
use dcli::error::Error;
use dcli::platform::Platform;
use dcli::response::drs::{DestinyResponseStatus, HasDestinyResponseStatus};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_derive::{Deserialize, Serialize};

pub struct MemberIdSearch {
    client: ApiClient,
}

impl MemberIdSearch {
    pub fn new(print_url: bool) -> MemberIdSearch {
        MemberIdSearch {
            client: ApiClient::new(print_url),
        }
    }

    pub async fn retrieve_member_id_from_steam(
        &self,
        steam_id: &str,
    ) -> Option<Result<Membership, Error>> {
        let url = format!(
            "https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/12/{steam_id}/",
            steam_id = utf8_percent_encode(&steam_id, NON_ALPHANUMERIC),
        );

        let resp = match self
            .client
            .call_and_parse::<DestinyResponseSteam>(&url)
            .await
        {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        let m = Membership {
            id: resp.response.membership_id,
            platform: Platform::from_id(resp.response.membership_type),
            display_name: None,
        };

        Some(Ok(m))
    }

    pub async fn retrieve_member_id(
        &self,
        id: &str,
        platform: Platform,
    ) -> Option<Result<Membership, Error>> {
        if platform == Platform::Steam {
            return self.retrieve_member_id_from_steam(&id).await;
        }

        let url = format!(
            "https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/{platform_id}/{id}/",
            platform_id = platform.to_id(),
            id = utf8_percent_encode(&id, NON_ALPHANUMERIC),
        );

        let resp = match self
            .client
            .call_and_parse::<DestinySearchResponse>(&url)
            .await
        {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };

        let mut results: Vec<DestinyResponseMember> = resp.response;
        if results.is_empty() {
            return None;
        }

        let r_member: &DestinyResponseMember = &results[0];

        let m = Membership {
            id: String::from(r_member.membership_id.as_str()),
            platform: Platform::from_id(r_member.membership_type),
            display_name: results[0].display_name.take(), //this is probably not the right way to do this
        };

        Some(Ok(m))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DestinySearchResponse {
    #[serde(rename = "Response")]
    response: Vec<DestinyResponseMember>,

    #[serde(flatten)]
    status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for DestinySearchResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponseSteam {
    #[serde(rename = "Response")]
    response: DestinyResponseMember,

    #[serde(flatten)]
    status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for DestinyResponseSteam {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponseMember {
    #[serde(rename = "membershipType")]
    membership_type: u64,

    #[serde(rename = "membershipId")]
    membership_id: String,

    #[serde(rename = "displayName")]
    display_name: Option<String>,
}

pub struct Membership {
    pub platform: Platform,
    pub id: String,
    pub display_name: Option<String>,
}
