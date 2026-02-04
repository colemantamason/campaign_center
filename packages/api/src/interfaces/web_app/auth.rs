use crate::enums::{MemberRole, SubscriptionType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// request to register a new user
#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// request to login
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// auth response with user data
#[derive(Clone, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user_id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

// organization info for user account response
#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationInfo {
    pub id: i32,
    pub name: String,
    pub avatar_url: Option<String>,
    pub member_count: i32,
}

// organization membership for user account response
#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationMembershipInfo {
    pub organization_id: i32,
    pub organization: OrganizationInfo,
    pub user_role: MemberRole,
    pub permissions: HashMap<SubscriptionType, bool>,
}

// full user account response with organizations
#[derive(Clone, Deserialize, Serialize)]
pub struct UserAccountResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub active_organization_membership_id: Option<i32>,
    pub organization_memberships: HashMap<i32, OrganizationMembershipInfo>,
}
