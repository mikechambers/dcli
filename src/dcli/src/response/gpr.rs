use serde_derive::{Deserialize, Serialize};
use crate::character::{Character};
use std::collections::HashMap;
use crate::response::drs::{DestinyResponseStatus, HasDestinyResponseStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProfileResponse {
    #[serde(rename = "Response")]
    pub response: Option<CharactersField>, //should this be an option?

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

impl HasDestinyResponseStatus for GetProfileResponse {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharactersField {
    pub characters:CharacterDataField,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterDataField {
    pub data:HashMap<String, Character>,
}