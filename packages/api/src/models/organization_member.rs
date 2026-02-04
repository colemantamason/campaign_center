use crate::schema::organization_members;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum MemberRole {
    Owner,
    Admin,
    Manager,
    Member,
}

impl MemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemberRole::Owner => "owner",
            MemberRole::Admin => "admin",
            MemberRole::Manager => "manager",
            MemberRole::Member => "member",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "owner" => Some(MemberRole::Owner),
            "admin" => Some(MemberRole::Admin),
            "manager" => Some(MemberRole::Manager),
            "member" => Some(MemberRole::Member),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            MemberRole::Owner => "Owner",
            MemberRole::Admin => "Admin",
            MemberRole::Manager => "Manager",
            MemberRole::Member => "Member",
        }
    }

    // check if this role can manage the target role
    pub fn can_manage(&self, target: &MemberRole) -> bool {
        match (self, target) {
            (MemberRole::Owner, _) => true,
            (MemberRole::Admin, MemberRole::Owner) => false,
            (MemberRole::Admin, _) => true,
            _ => false,
        }
    }
}

impl Display for MemberRole {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = organization_members)]
#[diesel(check_for_backend(Postgres))]
pub struct OrganizationMember {
    pub id: i32,
    pub organization_id: i32,
    pub user_id: i32,
    pub role: String,
    pub invited_by: Option<i32>,
    pub joined_at: DateTime<Utc>,
    pub last_active_at: Option<DateTime<Utc>>,
}

impl OrganizationMember {
    pub fn get_role(&self) -> MemberRole {
        MemberRole::from_str(&self.role).unwrap_or(MemberRole::Member)
    }
}

#[derive(Insertable)]
#[diesel(table_name = organization_members)]
pub struct NewOrganizationMember {
    pub organization_id: i32,
    pub user_id: i32,
    pub role: String,
    pub invited_by: Option<i32>,
}

impl NewOrganizationMember {
    pub fn new(organization_id: i32, user_id: i32, role: MemberRole) -> Self {
        Self {
            organization_id,
            user_id,
            role: role.as_str().to_string(),
            invited_by: None,
        }
    }

    pub fn set_invited_by(mut self, invited_by: i32) -> Self {
        self.invited_by = Some(invited_by);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = organization_members)]
pub struct OrganizationMemberUpdate {
    pub role: Option<String>,
    pub last_active_at: Option<DateTime<Utc>>,
}
