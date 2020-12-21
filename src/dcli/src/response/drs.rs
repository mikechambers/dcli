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

use serde_derive::{Deserialize, Serialize};

use crate::error::Error;

pub const API_RESPONSE_STATUS_SUCCESS: u32 = 1;

pub fn check_destiny_response_status(status: &DestinyResponseStatus) -> Result<(), Error> {
    match status.error_code {
        1 => Ok(()),
        5 => Err(Error::ApiNotAvailableException),
        7 => Err(Error::ParameterParseFailure),
        18 => Err(Error::InvalidParameters),
        1665 => Err(Error::PrivacyException),
        2102 => Err(Error::ApiKeyMissingFromRequest),
        _ => Err(Error::ApiStatus {
            description: format!(
                "Response Status Error : {}({}) : {}",
                status.error_status, status.error_code, status.message
            ),
        }),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyResponseStatus {
    #[serde(rename = "ErrorCode")]
    pub error_code: u32,

    #[serde(rename = "ThrottleSeconds")]
    pub throttle_seconds: u32,

    #[serde(rename = "ErrorStatus")]
    pub error_status: String,

    #[serde(rename = "Message")]
    pub message: String,
}

pub trait IsDestinyAPIResponse {
    fn get_status(&self) -> &DestinyResponseStatus;
}
