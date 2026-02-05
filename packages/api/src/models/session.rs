use crate::enums::{DeviceInfo, Platform};
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
    pub platform: String,
}

impl Session {
    pub fn is_valid(&self) -> bool {
        self.expires_at > Utc::now()
    }

    pub fn platform(&self) -> Platform {
        Platform::from_str(&self.platform).unwrap_or(Platform::Web)
    }

    pub fn device_info(&self) -> Option<DeviceInfo> {
        self.user_agent
            .as_ref()
            .map(|user_agent| DeviceInfo::from_user_agent(user_agent))
    }

    // get a human-readable device description for display in UI
    pub fn device_display(&self) -> String {
        match self.device_info() {
            Some(info) => info.display_string(self.platform()),
            None => self.platform().display_name().to_string(),
        }
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
    pub platform: String,
}

impl NewSession {
    pub fn new(user_id: i32, expiry_seconds: i64, platform: Platform) -> Self {
        Self {
            token: Uuid::new_v4(),
            user_id,
            active_organization_membership_id: None,
            user_agent: None,
            ip_address: None,
            expires_at: Utc::now() + Duration::seconds(expiry_seconds),
            platform: platform.as_str().to_string(),
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
