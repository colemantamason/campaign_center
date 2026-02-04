use crate::error::AppError;
use crate::models::{NewUser, User, UserUpdate};
use crate::postgres::get_postgres_connection;
use crate::schema::users;
use crate::services::{hash_password, validate_email, validate_password, verify_password};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn register_user(
    email: String,
    password: String,
    first_name: String,
    last_name: String,
) -> Result<User, AppError> {
    validate_email(&email)?;
    validate_password(&password)?;

    if first_name.trim().is_empty() {
        return Err(AppError::validation("first_name", "First name is required"));
    }

    if last_name.trim().is_empty() {
        return Err(AppError::validation("last_name", "Last name is required"));
    }

    let connection = &mut get_postgres_connection().await?;

    let existing: Option<User> = users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    if existing.is_some() {
        return Err(AppError::already_exists("User with this email"));
    }

    let password_hash = hash_password(&password)?;

    let new_user = NewUser::new(
        email.to_lowercase(),
        password_hash,
        first_name.trim().to_string(),
        last_name.trim().to_string(),
        // TODO: Ask user to set timezone?
        "America/New_York".to_string(),
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

// pre-computed dummy hash for timing attack mitigation
// this ensures consistent response time whether user exists or not
const DUMMY_PASSWORD_HASH: &str =
    "$argon2id$v=19$m=16,t=2,p=1$cnE4cnVmWGNhaTVLSXBrag$csCTOmrwecqL022wLOtkWA";

pub async fn authenticate_user(email: &str, password: &str) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    // fetch user (may be None if not found)
    let user: Option<User> = users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    // timing attack mitigation: always verify a hash to normalize response time
    // whether the user exists or not, we perform the same expensive hash verification
    let hash_to_verify = user
        .as_ref()
        .map(|u| u.password_hash.as_str())
        .unwrap_or(DUMMY_PASSWORD_HASH);

    let password_valid = verify_password(password, hash_to_verify).unwrap_or(false);

    // only succeed if both user exists AND password is valid
    match (user, password_valid) {
        (Some(valid_user), true) => {
            // update last login time
            diesel::update(users::table.find(valid_user.id))
                .set(users::last_login_at.eq(Some(Utc::now())))
                .execute(connection)
                .await
                .map_err(|error| AppError::ExternalServiceError {
                    service: "Postgres".to_string(),
                    message: error.to_string(),
                })?;

            Ok(valid_user)
        }
        _ => Err(AppError::InvalidCredentials),
    }
}

pub async fn get_user_by_id(user_id: i32) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    users::table
        .find(user_id)
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::not_found("User"))
}

pub async fn get_user_by_email(email: &str) -> Result<Option<User>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn update_user(user_id: i32, update: UserUpdate) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(users::table.find(user_id))
        .set(&update)
        .get_result::<User>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn change_password(
    user_id: i32,
    current_password: &str,
    new_password: &str,
) -> Result<(), AppError> {
    let user = get_user_by_id(user_id).await?;

    if !verify_password(current_password, &user.password_hash)? {
        return Err(AppError::validation(
            "current_password",
            "Current password is incorrect",
        ));
    }

    validate_password(new_password)?;

    let new_hash = hash_password(new_password)?;

    let connection = &mut get_postgres_connection().await?;

    diesel::update(users::table.find(user_id))
        .set(users::password_hash.eq(new_hash))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}
