use crate::web_app::{
    notification::{get_mock_notifications, Notifications},
    organization::{get_mock_organization_memberships, OrganizationMemberships},
};
use dioxus::prelude::*;

#[derive(Store)]
pub struct UserAccount {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub active_organization_membership_id: Option<i32>,
    pub organization_memberships: OrganizationMemberships,
    pub notifications: Notifications,
}

pub fn get_mock_user_account() -> UserAccount {
    UserAccount {
        id: 0,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        avatar_url: None,
        active_organization_membership_id: Some(0),
        organization_memberships: get_mock_organization_memberships(),
        notifications: get_mock_notifications(),
    }
}
