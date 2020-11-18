
use serde_derive::{Deserialize, Serialize};
use crate::error::Error;

pub fn check_destiny_response_status(status:&DestinyResponseStatus) -> Result<(), Error> {

    let out = match status.error_code {
        1       => Ok(()),
        5       => Err(Error::ApiNotAvailableException),
        7       => Err(Error::ParameterParseFailure),
        18      => Err(Error::InvalidParameters),
        1665    => Err(Error::PrivacyException),
        2102    => Err(Error::ApiKeyMissingFromRequest),
        _       => Err(Error::ApiStatus{description:format!("Response Status Error : {}({}) : {}", status.error_status, status.error_code, status.message)}),
    };

    out
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

pub trait HasDestinyResponseStatus {
    fn get_status(&self) -> &DestinyResponseStatus;
}