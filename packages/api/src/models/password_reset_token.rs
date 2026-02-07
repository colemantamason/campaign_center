use crate::schema::password_reset_tokens;
use chrono::{DateTime, Duration, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use uuid::Uuid;

pub const PASSWORD_RESET_TOKEN_EXPIRY_SECONDS: i64 = 3600;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = password_reset_tokens)]
#[diesel(check_for_backend(Postgres))]
pub struct PasswordResetToken {
    pub id: i32,
    pub user_id: i32,
    pub token: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

impl PasswordResetToken {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    pub fn is_used(&self) -> bool {
        self.used_at.is_some()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used()
    }
}

#[derive(Insertable)]
#[diesel(table_name = password_reset_tokens)]
pub struct NewPasswordResetToken {
    pub user_id: i32,
    pub token: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl NewPasswordResetToken {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id,
            token: Uuid::new_v4(),
            expires_at: Utc::now() + Duration::seconds(PASSWORD_RESET_TOKEN_EXPIRY_SECONDS),
        }
    }
}
