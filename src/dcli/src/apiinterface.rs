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

use crate::apiclient::ApiClient;
use crate::character::Character;
use crate::error::Error;
use crate::platform::Platform;
use crate::response::gpr::GetProfileResponse;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub struct ApiInterface {
    client: ApiClient,
}

impl ApiInterface {
    pub fn new(print_url: bool) -> ApiInterface {
        ApiInterface {
            client: ApiClient::new(print_url),
        }

        //Have an option on to take a manifest, if manifest is avaliable it will use it
        //some methods may require it and will throw errors if its not set
    }

    /// Retrieves characters for specified member_id and platform
    pub async fn retrieve_characters(
        &self,
        member_id: String,
        platform: Platform,
    ) -> Result<Vec<Character>, Error> {
        let url =
            format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=200",
                platform_id = platform.to_id(),
                member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
            );

        let profile: GetProfileResponse = self
            .client
            .call_and_parse::<GetProfileResponse>(&url)
            .await?;

        let response = match profile.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from("No response data from API Call."),
                })
            }
        };

        let mut characters: Vec<Character> = Vec::new();
        for c in response.characters.data.values() {
            characters.push(c.clone());
        }

        Ok(characters)
    }
}
