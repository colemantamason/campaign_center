use crate::schema::invitations;
use chrono::{DateTime, Duration, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

#[derive(PartialEq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
}

impl InvitationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvitationStatus::Pending => "pending",
            InvitationStatus::Accepted => "accepted",
            InvitationStatus::Expired => "expired",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "pending" => Some(InvitationStatus::Pending),
            "accepted" => Some(InvitationStatus::Accepted),
            "expired" => Some(InvitationStatus::Expired),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            InvitationStatus::Pending => "Pending",
            InvitationStatus::Accepted => "Accepted",
            InvitationStatus::Expired => "Expired",
        }
    }
}

impl Display for InvitationStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.display_name())
    }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = invitations)]
#[diesel(check_for_backend(Postgres))]
pub struct Invitation {
    pub id: i32,
    pub organization_id: i32,
    pub email: String,
    pub role: String,
    pub token: Uuid,
    pub status: String,
    pub invited_by: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
}

impl Invitation {
    pub fn get_status(&self) -> InvitationStatus {
        InvitationStatus::from_str(&self.status).unwrap_or(InvitationStatus::Pending)
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now() || self.get_status() == InvitationStatus::Expired
    }

    pub fn can_accept(&self) -> bool {
        !self.is_expired() && self.get_status() == InvitationStatus::Pending
    }
}

#[derive(Insertable)]
#[diesel(table_name = invitations)]
pub struct NewInvitation {
    pub organization_id: i32,
    pub email: String,
    pub role: String,
    pub token: Uuid,
    pub status: String,
    pub invited_by: i32,
    pub expires_at: DateTime<Utc>,
}

impl NewInvitation {
    pub fn new(organization_id: i32, email: String, role: String, invited_by: i32) -> Self {
        Self {
            organization_id,
            email,
            role,
            token: Uuid::new_v4(),
            status: InvitationStatus::Pending.as_str().to_string(),
            invited_by,
            expires_at: Utc::now() + Duration::days(7), // expires after 7 days
        }
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = invitations)]
pub struct InvitationUpdate {
    pub status: Option<String>,
    pub accepted_at: Option<DateTime<Utc>>,
}
