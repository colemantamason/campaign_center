use crate::enums::OrganizationType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub organization_type: OrganizationType,
}

#[derive(Deserialize, Serialize)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct OrganizationMemberResponse {
    pub user_id: i32,
    pub organization_id: i32,
    pub role: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct InviteMemberRequest {
    pub email: String,
    pub role: String,
}
