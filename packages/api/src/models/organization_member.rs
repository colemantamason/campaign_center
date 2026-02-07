use crate::enums::MemberRole;
use crate::schema::organization_members;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};

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
    pub last_active_at: Option<Option<DateTime<Utc>>>,
}
