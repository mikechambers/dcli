use serde::Deserialize;
use serde_derive::{Deserialize, Serialize};

use crate::apiclient::DestinyResponseStatus;

const BASE_URL: &str = "https://www.bungie.net";

//TODO: we can collapse this into a single object to reuse
//TODO: move into own file
#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestResponse {
    #[serde(rename = "Response")]
    pub manifest: Manifest,

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub version: String,

    #[serde(rename = "mobileWorldContentPaths")]
    pub mobile_world_content_paths: MobileWorldContentPaths,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileWorldContentPaths {
    #[serde(deserialize_with = "prepend_base_url")]
    pub en: String,
}

fn prepend_base_url<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    //TODO: move to URL base to constant
    String::deserialize(deserializer).map(|a| {
        let mut s = String::from(BASE_URL);
        s.push_str(&a);
        s
    })
}
