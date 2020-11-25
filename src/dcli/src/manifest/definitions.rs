use serde_derive::{Deserialize, Serialize};
use crate::apiutils::prepend_base_url;

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayPropertiesData {
    pub description:String,
    pub name:String,

    #[serde(rename="icon", deserialize_with = "prepend_base_url")]
    pub icon_path:String,

    #[serde(rename = "hasIcon")]
    pub has_icon:bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,

    #[serde(rename = "pgcrImage", deserialize_with = "prepend_base_url")]
    pub pgcr_image:String,

    #[serde(rename = "destinationHash")]
    pub destination_hash:u32,

    #[serde(rename = "placeHash")]
    pub place_hash:u32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DestinationDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,

    #[serde(rename = "placeHash")]
    pub place_hash:u32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PlaceDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,
}