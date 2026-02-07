use crate::enums::OrganizationType;
#[cfg(feature = "server")]
use crate::models::Organization;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub organization_type: OrganizationType,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub avatar_url: Option<Option<String>>,
    pub website_url: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
    pub address_line_1: Option<Option<String>>,
    pub address_line_2: Option<Option<String>>,
    pub city: Option<Option<String>>,
    pub state: Option<Option<String>>,
    pub zip_code: Option<Option<String>>,
    pub country: Option<Option<String>>,
    pub timezone: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub organization_type: OrganizationType,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "server")]
impl From<Organization> for OrganizationResponse {
    fn from(organization: Organization) -> Self {
        let organization_type = organization.get_organization_type();
        Self {
            id: organization.id,
            name: organization.name,
            slug: organization.slug,
            organization_type,
            description: organization.description,
            avatar_url: organization.avatar_url,
            website_url: organization.website_url,
            email: organization.email,
            phone_number: organization.phone_number,
            address_line_1: organization.address_line_1,
            address_line_2: organization.address_line_2,
            city: organization.city,
            state: organization.state,
            zip_code: organization.zip_code,
            country: organization.country,
            timezone: organization.timezone,
            created_at: organization.created_at,
            updated_at: organization.updated_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationMemberListResponse {
    pub members: Vec<OrganizationMemberResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
