use serde_derive::{Deserialize, Serialize};
use crate::apiutils::prepend_base_url;

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayPropertiesData {
    pub description:String,
    pub name:String,

    #[serde(deserialize_with = "prepend_base_url")]
    pub icon_path:String,

    #[serde(rename = "hasIcon")]
    pub has_icon:bool
}