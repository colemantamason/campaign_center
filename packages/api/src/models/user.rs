use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Identifiable, Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Postgres))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub phone_number_verified_at: Option<DateTime<Utc>>,
    pub avatar_url: Option<String>,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub timezone: String,
}

impl NewUser {
    pub fn new(
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
        timezone: String,
    ) -> Self {
        Self {
            email,
            password_hash,
            first_name,
            last_name,
            phone_number: None,
            timezone,
        }
    }
}

#[derive(AsChangeset, Debug, Default)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
}
