use serde_derive::{Deserialize, Serialize};
use serde::{Deserialize, Deserializer};

use crate::apiclient::DestinyResponseStatus;

//TODO: we can collapse this into a single object to reuse
//TODO: move into own file
#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "Response")]
    pub version: String,

    pub mobile_world_content_paths:MobileWorldContentPaths,

    #[serde(flatten)]
    pub status:DestinyResponseStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileWorldContentPaths {
    #[serde(deserialize_with = "prepend_base_url")]
    pub en:String,
}

fn prepend_base_url<'de, D>(deserializer: D) -> Result<String, D::Error> where D: serde::de::Deserializer<'de>
{
    //TODO: move to URL base to constant
    MobileWorldContentPaths::deserialize(deserializer).map(|a| format!("https://www.bungie.net{:?}", a))
}

