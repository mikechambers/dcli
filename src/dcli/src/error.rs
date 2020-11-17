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

#[derive(Debug)]
pub enum Error {
    ApiRequest,
    ApiStatus,
    ApiParse,
    IoError,
    IoErrorDirIsFile,
    ZipError,
    Unknown,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::ApiRequest => write!(f, "Error calling Destiny 2 API."),
            Error::ApiStatus => write!(f, "Destiny 2 API call returned an error."),
            Error::ApiParse => write!(f, "Error parsing results from Destiny 2 API call."),
            Error::IoError => write!(f, "Error working with file system."),
            Error::ZipError => write!(f, "Error decompressing manifest."),
            Error::IoErrorDirIsFile => write!(f, "Expected directory but found file."),
            Error::Unknown => write!(f, "An unknown error occured."),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Error {
        Error::ApiParse //TODO:: impliment this for all error types
    }
}

impl From<reqwest::Error> for Error {
    fn from(_err: reqwest::Error) -> Error {
        Error::ApiRequest //TODO:: impliment this for all error types
    }
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::IoError //TODO:: impliment this for all error types
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(_err: zip::result::ZipError) -> Error {
        Error::ZipError //TODO:: impliment this for all error types
    }
}