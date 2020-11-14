use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ManifestInfo {
    pub version:String,
    pub url:String,
}

//info on serializing
//https://docs.serde.rs/serde_json/