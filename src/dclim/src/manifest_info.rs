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

use dcli::error::Error;
use dcli::manifest::Manifest;
use serde_derive::{Deserialize, Serialize};

//TODO: move to its own file
#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestInfo {
    pub version: String,
    pub url: String,
}

impl ManifestInfo {
    pub fn from_manifest(manifest: &Manifest) -> ManifestInfo {
        ManifestInfo {
            version: String::from(&manifest.version),
            url: String::from(&manifest.mobile_world_content_paths.en),
        }
    }

    pub fn from_json(json: &str) -> Result<ManifestInfo, Error> {
        let m: ManifestInfo = serde_json::from_str(json)?;

        Ok(m)
    }

    pub fn to_json(&self) -> Result<String, Error> {
        //todo: do wes need to catch errors here? Would this ever fail?
        let out = serde_json::to_string(self)?;

        Ok(out)
    }
}
