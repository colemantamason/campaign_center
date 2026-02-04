use crate::schema::sessions;
use chrono::{DateTime, Duration, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use ipnetwork::IpNetwork;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(Postgres))]
pub struct Session {
    pub id: i32,
    pub token: Uuid,
    pub user_id: i32,
    pub active_organization_membership_id: Option<i32>,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
}

impl Session {
    pub fn is_valid(&self) -> bool {
        self.expires_at > Utc::now()
    }
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub token: Uuid,
    pub user_id: i32,
    pub active_organization_membership_id: Option<i32>,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub expires_at: DateTime<Utc>,
}

impl NewSession {
    pub fn new(user_id: i32, expiry_hours: i64) -> Self {
        Self {
            token: Uuid::new_v4(),
            user_id,
            active_organization_membership_id: None,
            user_agent: None,
            ip_address: None,
            expires_at: Utc::now() + Duration::hours(expiry_hours),
        }
    }

    pub fn set_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn set_ip_address(mut self, ip_address: IpNetwork) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn set_active_organization_membership(mut self, membership_id: i32) -> Self {
        self.active_organization_membership_id = Some(membership_id);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = sessions)]
pub struct SessionUpdate {
    pub active_organization_membership_id: Option<Option<i32>>,
    pub last_accessed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}
