use crate::enums::{MemberRole, Platform, SubscriptionType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_staff: bool,
    pub platform: Platform,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub platform: Platform,
}

#[derive(Deserialize, Serialize)]
pub struct LogoutRequest {
    pub platform: Platform,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user_id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_staff: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationInfo {
    pub id: i32,
    pub name: String,
    pub avatar_url: Option<String>,
    pub member_count: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct OrganizationMembershipInfo {
    pub id: i32,
    pub organization_id: i32,
    pub organization: OrganizationInfo,
    pub user_role: MemberRole,
    pub permissions: HashMap<SubscriptionType, bool>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserAccountResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub active_organization_membership_id: Option<i32>,
    pub organization_memberships: HashMap<i32, OrganizationMembershipInfo>,
}
