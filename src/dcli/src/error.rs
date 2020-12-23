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

//For error handling approach, we are going to start with one error for all
//APIs and individual apps. Given that there is only a general range of what the apps
//do, mostly loading and parsing api data, then we should be able to cover
//error cases without super ballooning the number of error types.
//If it turns out this becomes unwieldy, then we will break it out, into API
//and app specific errors

use std::fmt::{Display, Formatter, Result};

use crate::response::activities::MAX_ACTIVITIES_REQUEST_COUNT;

#[derive(PartialEq, Debug)]
pub enum Error {
    ApiRequest { description: String },
    ApiStatus { description: String },
    ApiResponseMissing,

    //when parameters are malformed in wrong format (i.e. expecting id, getting a name)
    ParameterParseFailure,
    //when id & platform are not correct combination
    InvalidParameters,
    //Api key not set correctly
    ApiKeyMissingFromRequest,
    ApiNotAvailableException,
    RequestTimedOut,
    Request,
    PrivacyException,
    Database { description: String },
    ApiParse { description: String },
    IoError { description: String },
    IoErrorDirIsFile { description: String },
    IoFileDoesNotExist { description: String },
    ZipError { description: String },
    Unknown { description: String },
    ManifestNotSet,
    ManifestItemNotFound { description: String },
    MaxActivitiesRequestCountExceeded,
    CharacterDataNotFound,
    SystemDirectoryNotFound,
    ChronoParse { description: String },
    UnknownEnumValue,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ApiRequest { description } => {
                write!(f, "Error calling Destiny 2 API. {}", description)
            },
            Error::ApiStatus { description } => {
                write!(f, "Destiny 2 API call returned an error. {}", description)
            },
            Error::ApiParse { description } => write!(
                f,
                "Error parsing results from Destiny 2 API call. {}",
                description
            ),
            Error::IoError { description } => {
                write!(f, "Error working with file system. {}", description)
            },
            Error::ZipError { description } => {
                write!(f, "Error decompressing manifest. {}", description)
            },
            Error::IoErrorDirIsFile { description } => {
                write!(f, "Expected directory but found file. {}", description)
            },
            Error::Unknown { description } => {
                write!(f, "An unknown error occured. {}", description)
            },
            Error::ParameterParseFailure => write!(f, "Could not parse Parameters. Make sure your inputs were correct and try again. (code 7)"),
            Error::InvalidParameters => write!(f, "Invalid input parameters. (code 18)"),
            Error::ManifestNotSet => write!(f, "Manifest was not set in Manifest Interface."),
            Error::ApiKeyMissingFromRequest => write!(
                f,
                "Missing API Key. Set DESTINY_API_KEY environment variable before compiling."
            ),
            Error::ApiNotAvailableException => {
                write!(f, "The Destiny API is currently not available. Please try again later.")
            },
            Error::PrivacyException => write!(
                f,
                "Privacy settings for Bungie account are too restrictive."
            ),
            Error::IoFileDoesNotExist { description } => {
                write!(f, "Expected File does not exist: {}", description)
            },
            Error::Database { description } => {
                write!(f, "Error working with SQLite database : {}", description)
            },
            Error::ManifestItemNotFound { description } => {
                write!(f, "Manifest Item not found : {}", description)
            },
            Error::ApiResponseMissing => write!(
                f,
                "Received response from API but no response property was present."
            ),
            Error::RequestTimedOut => write!(
                f,
                "The API request took too long. Check your network connection and\
                 try again. (The API servers may be slow right now)."
            ),
            Error::Request => write!(
                f,
                "There was an error during the API request. This often means \
                that we could not reach the Destiny servers. Check the network \
                connection and try again (The API servers might not be available.)."
            ),

            Error::MaxActivitiesRequestCountExceeded => write!(
                f,
                "The maximum number of activities ({}) requested was exceeded.",
                MAX_ACTIVITIES_REQUEST_COUNT
            ),
            Error::CharacterDataNotFound => write!(
                f,
                "Could not find entry in activity data for specified character."
            ),
            Error::SystemDirectoryNotFound  => {
                write!(f, "Could not locate system directory.")
            },
            Error::ChronoParse { description } => {
                write!(f, "Error parsing String to date / time : {}", description)
            },
            Error::UnknownEnumValue  => {
                write!(f, "Could not convert value to enum.")
            },

        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ApiParse {
            description: format!("serde_json::Error : {:#?}", err),
        } //TODO:: impliment this for all error types
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        /*
        //todo: need to figure out how to downcast to hyber error
        //so we can get more details on the error (i.e. network failure)
        //https://stackoverflow.com/a/61100595/10232
        let hyper_error: Option<&hyper::Error> = reqwest_error
            .source()
            .unwrap()
            .downcast_ref();
        */

        if err.is_timeout() {
            Error::RequestTimedOut
        } else if err.is_request() {
            Error::Request
        } else {
            Error::ApiRequest {
                description: format!("reqwest::Error : {:#?}", err),
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError {
            description: format!("std::io::Error : {:#?}", err),
        }
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Error {
        Error::ZipError {
            description: format!("zip::result::ZipError : {:#?}", err),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Error {
        Error::Database {
            description: format!("sqlx::Error : {:#?}", err),
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Error {
        Error::ChronoParse {
            description: format!("chrono::format::ParseError : {:#?}", err),
        }
    }
}
