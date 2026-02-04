use crate::enums::{MemberRole, SubscriptionType};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Store)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub avatar_url: Option<String>,
    pub member_count: i32,
}

impl Organization {
    pub fn new(id: i32, name: String, avatar_url: Option<String>, member_count: i32) -> Self {
        Self {
            id,
            name,
            avatar_url,
            member_count,
        }
    }
}

pub type Permissions = HashMap<SubscriptionType, bool>;

#[derive(Store)]
pub struct OrganizationMembership {
    pub organization_id: i32,
    pub organization: Organization,
    pub user_role: MemberRole,
    pub permissions: Permissions,
}

impl OrganizationMembership {
    pub fn new(
        organization_id: i32,
        name: String,
        avatar_url: Option<String>,
        member_count: i32,
        user_role: MemberRole,
        permissions: Permissions,
    ) -> Self {
        Self {
            organization_id,
            organization: Organization::new(organization_id, name, avatar_url, member_count),
            user_role,
            permissions,
        }
    }
}

pub type OrganizationMemberships = HashMap<i32, OrganizationMembership>;
