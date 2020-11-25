use serde_derive::{Deserialize, Serialize};
use crate::apiutils::{prepend_base_url_option};

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayPropertiesData {
    pub description:String,
    pub name:String,

    //https://stackoverflow.com/a/44303505/10232
    #[serde(default)]
    #[serde(rename="icon", deserialize_with = "prepend_base_url_option")]
    pub icon_path:Option<String>,

    #[serde(rename = "hasIcon")]
    pub has_icon:bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,

    #[serde(default)]
    #[serde(rename="pgcrImage", deserialize_with = "prepend_base_url_option")]
    pub pgcr_image:Option<String>,

    #[serde(rename = "destinationHash")]
    pub destination_hash:u32,

    #[serde(rename = "placeHash")]
    pub place_hash:u32,

    #[serde(rename = "activityTypeHash")]
    pub activity_type_hash:u32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTypeDefinitionData {

    #[serde(rename = "hash")]
    pub id:u32,

    #[serde(rename = "displayProperties")]
    pub display_properties:DisplayPropertiesData,
}

