/*
* Copyright 2021 Mike Chambers
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
use dcli::enums::platform::Platform;
use dcli::error::Error;
use dcli::response::drs::{DestinyResponseStatus, IsDestinyAPIResponse};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_derive::{Deserialize, Serialize};

pub struct MemberIdSearch {
    client: ApiClient,
}

impl MemberIdSearch {
    pub fn new(print_url: bool) -> Result<MemberIdSearch, Error> {
        let client = ApiClient::new(print_url)?;

        Ok(MemberIdSearch { client })
    }

    pub async fn retrieve_member_id_from_steam(
        &self,
        steam_id: &str,
    ) -> Result<Option<Membership>, Error> {
        let url = format!(
            "https://www.bungie.net/Platform/User/GetMembershipFromHardLinkedCredential/12/{steam_id}/",
            steam_id = utf8_percent_encode(&steam_id, NON_ALPHANUMERIC),
        );

        let member = match self
            .client
            .call_and_parse::<DestinyResponseSteam>(&url)
            .await?
            .response
        {
            Some(e) => e,
            None => return Err(Error::ApiResponseMissing), //we should never get here as this will be caught earlier
        };

        let m = Membership {
            id: member.membership_id,
            platform: Platform::from_id(member.membership_type as u32),
            display_name: None,
        };

        Ok(Some(m))
    }

    pub async fn retrieve_member_id(
        &self,
        id: &str,
        platform: Platform,
    ) -> Result<Option<Membership>, Error> {
        if platform == Platform::Steam {
            return self.retrieve_member_id_from_steam(&id).await;
        }

        let url = format!(
            "https://www.bungie.net/Platform/Destiny2/SearchDestinyPlayer/{platform_id}/{id}/",
            platform_id = platform.to_id(),
            id = utf8_percent_encode(&id, NON_ALPHANUMERIC),
        );

        let mut results: Vec<DestinyResponseMember> = match self
            .client
            .call_and_parse::<DestinySearchResponse>(&url)
            .await?
            .response
        {
            Some(e) => e,
            None => return Err(Error::ApiResponseMissing), //we should never get here as this will be caught earlier
        };

        if results.is_empty() {
            return Ok(None);
        }

        let r_member: &DestinyResponseMember = &results[0];

        let m = Membership {
            id: String::from(r_member.membership_id.as_str()),
            platform: Platform::from_id(r_member.membership_type as u32),
            display_name: results[0].display_name.take(), //this is probably not the right way to do this
        };

        Ok(Some(m))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DestinySearchResponse {
    #[serde(rename = "Response")]
    response: Option<Vec<DestinyResponseMember>>,

    #[serde(flatten)]
    status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for DestinySearchResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DestinyResponseSteam {
    #[serde(rename = "Response")]
    response: Option<DestinyResponseMember>,

    #[serde(flatten)]
    status: DestinyResponseStatus,
}

impl IsDestinyAPIResponse for DestinyResponseSteam {
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
