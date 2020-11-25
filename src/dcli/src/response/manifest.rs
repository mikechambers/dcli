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

use crate::response::drs::{DestinyResponseStatus, HasDestinyResponseStatus};
use crate::apiutils::prepend_base_url;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestResponse {
    #[serde(rename = "Response")]
    pub manifest: ManifestData,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for ManifestResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestData {
    pub version: String,

    #[serde(rename = "mobileWorldContentPaths")]
    pub mobile_world_content_paths: MobileWorldContentPathsData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileWorldContentPathsData {
    #[serde(deserialize_with = "prepend_base_url")]
    pub en: String,
}

