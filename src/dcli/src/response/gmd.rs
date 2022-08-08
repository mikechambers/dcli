use super::{
    drs::{DestinyResponseStatus, IsDestinyAPIResponse},
    pgcr::UserInfoCard,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMembershipData {
    #[serde(rename = "Response")]
    pub response: Option<UserMembershipData>, //should this be an option?

    #[serde(flatten)]
    pub status: DestinyResponseStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMembershipData {
    #[serde(rename = "destinyMemberships")]
    pub destiny_memberships: Vec<UserInfoCard>,
}

impl IsDestinyAPIResponse for GetMembershipData {
    fn get_status(&self) -> &DestinyResponseStatus {
        &self.status
    }
}
