//For error handling approach, we are going to start with one error for all
//APIs and individual apps. Given that there is only a general range of what the apps
//do, mostly loading and parsing api data, then we should be able to cover
//error cases without super ballooning the number of error types.
//If it turns out this becomes unwieldy, then we will break it out, into API
//and app specific errors

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    ApiRequest,
    ApiStatus,
    ApiParse,
    IoError,
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ApiRequest => write!(f, "Error calling Destiny 2 API."),
            Error::ApiStatus => write!(f, "Destiny 2 API call returned an error."),
            Error::ApiParse => write!(f, "Error parsing results from Destiny 2 API call."),
            Error::IoError => write!(f, "Error working with file system."),
            Error::Unknown => write!(f, "An unknown error occured."),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ApiParse //TODO:: impliment this for all error types
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        match err {
            _ => Error::ApiRequest,
        } //TODO:: impliment this for all error types
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        match err {
            _ => Error::IoError,
        } //TODO:: impliment this for all error types
    }
}