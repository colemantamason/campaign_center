use crate::models::{MemberRole, SubscriptionType};
use crate::state::{
    Notifications, Organization, OrganizationMembership, OrganizationMemberships, Permissions,
};
use dioxus::prelude::*;
use std::collections::HashMap;

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
    let mut permissions = Permissions::new();
    permissions.insert(SubscriptionType::Events, true);

    let organization = Organization {
        id: 1,
        name: "Test Organization".to_string(),
        avatar_url: None,
        member_count: 5,
    };

    let membership = OrganizationMembership {
        organization_id: 1,
        user_role: MemberRole::Owner,
        permissions: permissions.clone(),
        organization: organization.clone(),
    };

    let mut memberships = HashMap::new();
    memberships.insert(1, membership);

    UserAccount {
        id: 1,
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        avatar_url: None,
        active_organization_membership_id: Some(1),
        organization_memberships: memberships,
        notifications: HashMap::new(),
    }
}
