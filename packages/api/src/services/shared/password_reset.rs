use crate::error::AppError;
use crate::models::{NewPasswordResetToken, PasswordResetToken};
use crate::postgres::get_postgres_connection;
use crate::schema::{password_reset_tokens, users};
use crate::services::{
    delete_all_user_sessions, get_user_by_email, hash_password, validate_password,
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn request_password_reset(email: &str) -> Result<Option<Uuid>, AppError> {
    let user = match get_user_by_email(email).await? {
        Some(user) => user,
        None => return Ok(None), // don't reveal whether email exists
    };

    let connection = &mut get_postgres_connection().await?;

    // invalidate any existing unused tokens for this user
    diesel::update(
        password_reset_tokens::table
            .filter(password_reset_tokens::user_id.eq(user.id))
            .filter(password_reset_tokens::used_at.is_null()),
    )
    .set(password_reset_tokens::used_at.eq(Some(Utc::now())))
    .execute(connection)
    .await
    .map_err(|error| AppError::ExternalServiceError {
        service: "Postgres".to_string(),
        message: error.to_string(),
    })?;

    let new_token = NewPasswordResetToken::new(user.id);

    let token_uuid = new_token.token;

    diesel::insert_into(password_reset_tokens::table)
        .values(&new_token)
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(Some(token_uuid))
}

pub async fn reset_password(token: Uuid, new_password: &str) -> Result<(), AppError> {
    validate_password(new_password)?;

    let connection = &mut get_postgres_connection().await?;

    let reset_token: PasswordResetToken = password_reset_tokens::table
        .filter(password_reset_tokens::token.eq(token))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::validation("token", "Invalid or expired reset token"))?;

    if !reset_token.is_valid() {
        return Err(AppError::validation(
            "token",
            "Invalid or expired reset token",
        ));
    }

    let new_hash = hash_password(new_password)?;

    diesel::update(users::table.find(reset_token.user_id))
        .set(users::password_hash.eq(new_hash))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    diesel::update(password_reset_tokens::table.find(reset_token.id))
        .set(password_reset_tokens::used_at.eq(Some(Utc::now())))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    // invalidate all sessions after password reset
    delete_all_user_sessions(reset_token.user_id).await.ok();

    Ok(())
}

pub async fn validate_reset_token(token: Uuid) -> Result<bool, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let reset_token: Option<PasswordResetToken> = password_reset_tokens::table
        .filter(password_reset_tokens::token.eq(token))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(reset_token.map(|t| t.is_valid()).unwrap_or(false))
}

pub async fn cleanup_expired_reset_tokens() -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let count = diesel::delete(
        password_reset_tokens::table.filter(
            password_reset_tokens::expires_at
                .lt(Utc::now())
                .or(password_reset_tokens::used_at.is_not_null()),
        ),
    )
    .execute(connection)
    .await
    .map_err(|error| AppError::ExternalServiceError {
        service: "Postgres".to_string(),
        message: error.to_string(),
    })?;

    if count > 0 {
        tracing::info!("Cleaned up {} expired/used password reset tokens", count);
    }

    Ok(count as i32)
}
