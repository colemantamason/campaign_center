use dioxus::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

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

#[derive(Clone, PartialEq)]
pub enum UserRoleType {
    Admin,
    Member,
}

impl Display for UserRoleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            UserRoleType::Admin => write!(f, "Admin"),
            UserRoleType::Member => write!(f, "Member"),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum PermissionType {
    Events,
}

pub type Permissions = HashMap<PermissionType, bool>;

#[derive(Clone, Store)]
pub struct OrganizationMembership {
    pub organization_id: i32,
    pub organization: Organization,
    pub user_role: UserRoleType,
    pub permissions: Permissions,
}

impl OrganizationMembership {
    pub fn new(
        organization_id: i32,
        name: String,
        avatar_url: Option<String>,
        member_count: i32,
        user_role: UserRoleType,
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

pub fn get_mock_organization_memberships() -> OrganizationMemberships {
    let create_organization_membership = |id,
                                          name,
                                          avatar_url,
                                          member_count,
                                          user_role,
                                          permissions| {
        (
            id,
            OrganizationMembership::new(id, name, avatar_url, member_count, user_role, permissions),
        )
    };

    OrganizationMemberships::from([
        create_organization_membership(
            0,
            "Test Organization".to_string(),
            Some(
                "https://m.media-amazon.com/images/I/61siVuPB-0L._AC_UF894,1000_QL80_.jpg"
                    .to_string(),
            ),
            12,
            UserRoleType::Admin,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            1,
            "Test Organization 2".to_string(),
            None,
            5,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, false)]),
        ),
        create_organization_membership(
            2,
            "Test Organization 3".to_string(),
            None,
            8,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            3,
            "Test Organization 4".to_string(),
            None,
            150,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            4,
            "Test Organization 50".to_string(),
            None,
            3,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            5,
            "Test Organization 51".to_string(),
            None,
            42,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            6,
            "Test Organization 51".to_string(),
            None,
            1,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            7,
            "Test Organization 8".to_string(),
            None,
            10,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
        create_organization_membership(
            8,
            "Test Organization 9".to_string(),
            None,
            7,
            UserRoleType::Member,
            Permissions::from([(PermissionType::Events, true)]),
        ),
    ])
}
