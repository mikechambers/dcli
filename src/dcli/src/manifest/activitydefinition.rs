use serde_derive::{Deserialize, Serialize};
use crate::apiutils::prepend_base_url;
use crate::mode::Mode;
use crate::manifest::displayproperties::DisplayPropertiesData;

#[derive(Serialize, Deserialize, Debug)]
pub struct DestinyActivityDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,

    #[serde(rename = "pgcrImage", deserialize_with = "prepend_base_url")]
    pub pgcr_image:String,

    #[serde(rename = "directActivityModeType")]
    pub direct_activity_mode_type:Mode,
}