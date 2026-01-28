use crate::web_app::organization::{get_mock_organizations, Organizations};
use dioxus::prelude::*;

#[derive(Store)]
pub struct UserAccount {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub active_organization_id: i32,
    pub organizations: Organizations,
    pub notifications: i32,
}

pub fn get_mock_user_account() -> UserAccount {
    UserAccount {
        id: 0,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        avatar_url: None,
        active_organization_id: 0,
        organizations: get_mock_organizations(),
        notifications: 3,
    }
}
