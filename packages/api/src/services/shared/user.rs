use crate::error::{postgres_error, AppError};
use crate::models::{NewUser, User, UserUpdate};
use crate::postgres::get_postgres_connection;
use crate::schema::users;
use crate::services::{
    delete_all_user_sessions, hash_password, validate_email, validate_password,
    validate_required_string, verify_password, MAX_USER_NAME_LENGTH,
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn register_user(
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    is_staff: bool,
) -> Result<User, AppError> {
    validate_email(&email)?;
    validate_password(&password)?;
    validate_required_string("first_name", &first_name, MAX_USER_NAME_LENGTH)?;
    validate_required_string("last_name", &last_name, MAX_USER_NAME_LENGTH)?;

    let connection = &mut get_postgres_connection().await?;

    let existing: Option<User> = users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

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
        is_staff,
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(connection)
        .await
        .map_err(postgres_error)
}

// hash for "hunter42" used for timing attack mitigation
const DUMMY_PASSWORD_HASH: &str =
    "$argon2id$v=19$m=16,t=2,p=1$cnE4cnVmWGNhaTVLSXBrag$csCTOmrwecqL022wLOtkWA";

pub async fn authenticate_user(email: &str, password: &str) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let user: Option<User> = users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

    // timing attack mitigation: even if the user doesn't exist, we verify a hash to normalize response time
    let hash_to_verify = user
        .as_ref()
        .map(|u| u.password_hash.as_str())
        .unwrap_or(DUMMY_PASSWORD_HASH);

    let password_valid = verify_password(password, hash_to_verify).unwrap_or(false);

    match (user, password_valid) {
        (Some(valid_user), true) => {
            diesel::update(users::table.find(valid_user.id))
                .set(users::last_login_at.eq(Some(Utc::now())))
                .execute(connection)
                .await
                .map_err(postgres_error)?;

            Ok(valid_user)
        }
        _ => {
            tracing::warn!(
                "Failed login attempt for email: {}",
                email.to_lowercase()
            );
            Err(AppError::InvalidCredentials)
        }
    }
}

pub async fn get_user_by_id(user_id: i32) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    users::table
        .find(user_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("User"))
}

pub async fn get_user_by_email(email: &str) -> Result<Option<User>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    users::table
        .filter(users::email.eq(&email.to_lowercase()))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)
}

pub async fn update_user(user_id: i32, update: UserUpdate) -> Result<User, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(users::table.find(user_id))
        .set(&update)
        .get_result::<User>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn change_password(
    user_id: i32,
    current_password: &str,
    new_password: &str,
    current_session_token: Option<Uuid>,
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
        .map_err(postgres_error)?;

    if let Err(error) = delete_all_user_sessions(user_id, current_session_token).await {
        tracing::warn!(
            "failed to invalidate other sessions after password change for user {}: {}",
            user_id,
            error
        );
    }

    Ok(())
}
