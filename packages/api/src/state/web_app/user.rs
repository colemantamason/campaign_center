use crate::state::{Notifications, OrganizationMemberships};
use dioxus::prelude::*;

#[derive(Clone, Store)]
pub struct UserAccount {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub active_organization_membership_id: Option<i32>,
    pub organization_memberships: OrganizationMemberships,
    pub notifications: Notifications,
}
