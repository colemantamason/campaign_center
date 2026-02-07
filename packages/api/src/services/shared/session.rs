use crate::enums::{Platform, SESSION_EXPIRY_SECONDS, SLIDING_SESSION_THRESHOLD_SECONDS};
use crate::error::{postgres_error, AppError};
use crate::models::{NewSession, Session, SessionUpdate};
use crate::postgres::get_postgres_connection;
use crate::redis::batch_invalidate_redis_cached_sessions;
use crate::schema::sessions;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn create_session(
    user_id: i32,
    platform: Platform,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let mut new_session = NewSession::new(user_id, SESSION_EXPIRY_SECONDS as i64, platform);

    if let Some(ua) = user_agent {
        new_session = new_session.set_user_agent(ua);
    }

    // silently ignore invalid IPs
    if let Some(ip_string) = ip_address {
        if let Ok(ip) = ip_string.parse::<std::net::IpAddr>() {
            new_session = new_session.set_ip_address(ipnetwork::IpNetwork::from(ip));
        }
    }

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .get_result::<Session>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn get_session_by_token(token: Uuid) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let session: Session = sessions::table
        .filter(sessions::token.eq(token))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or(AppError::NotAuthenticated)?;

    if !session.is_valid() {
        diesel::delete(sessions::table.find(session.id))
            .execute(connection)
            .await
            .ok();
        return Err(AppError::SessionExpired);
    }

    Ok(session)
}

pub async fn validate_session(token: Uuid) -> Result<Session, AppError> {
    let session = get_session_by_token(token).await?;

    if should_extend_session(&session) {
        return extend_session_expiry(session.id).await;
    }

    // otherwise just update last_accessed_at
    let connection = &mut get_postgres_connection().await?;
    diesel::update(sessions::table.find(session.id))
        .set(sessions::last_accessed_at.eq(Utc::now()))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    Ok(session)
}

pub fn should_extend_session(session: &Session) -> bool {
    let elapsed = Utc::now() - session.last_accessed_at;
    elapsed.num_seconds() >= SLIDING_SESSION_THRESHOLD_SECONDS as i64
}

pub async fn extend_session_expiry(session_id: i32) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let now = Utc::now();
    let new_expires_at = now + Duration::seconds(SESSION_EXPIRY_SECONDS as i64);

    diesel::update(sessions::table.find(session_id))
        .set(SessionUpdate {
            last_accessed_at: Some(now),
            expires_at: Some(new_expires_at),
            ..Default::default()
        })
        .get_result::<Session>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn set_active_organization(
    session_id: i32,
    membership_id: Option<i32>,
) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(sessions::table.find(session_id))
        .set(SessionUpdate {
            active_organization_membership_id: Some(membership_id),
            last_accessed_at: Some(Utc::now()),
            ..Default::default()
        })
        .get_result::<Session>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn delete_session(token: Uuid) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::delete(sessions::table.filter(sessions::token.eq(token)))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    Ok(())
}

pub async fn delete_all_user_sessions(
    user_id: i32,
    exclude_token: Option<Uuid>,
) -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let mut token_query = sessions::table
        .filter(sessions::user_id.eq(user_id))
        .into_boxed();

    if let Some(ref token) = exclude_token {
        token_query = token_query.filter(sessions::token.ne(*token));
    }

    let tokens: Vec<Uuid> = token_query
        .select(sessions::token)
        .load::<Uuid>(connection)
        .await
        .map_err(postgres_error)?;

    let token_strings: Vec<String> = tokens.iter().map(|token| token.to_string()).collect();
    if let Err(error) = batch_invalidate_redis_cached_sessions(&token_strings).await {
        tracing::warn!(
            "failed to batch invalidate redis cache during delete_all_user_sessions: {}",
            error
        );
    }

    if tokens.is_empty() {
        return Ok(0);
    }

    let count = diesel::delete(sessions::table.filter(sessions::token.eq_any(&tokens)))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    Ok(count as i32)
}

pub async fn list_user_sessions(user_id: i32) -> Result<Vec<Session>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    sessions::table
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::expires_at.gt(Utc::now()))
        .order(sessions::last_accessed_at.desc())
        .load::<Session>(connection)
        .await
        .map_err(postgres_error)
}

// TODO: schedule this to run periodically
pub async fn cleanup_expired_sessions() -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let count = diesel::delete(sessions::table.filter(sessions::expires_at.lt(Utc::now())))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    if count > 0 {
        tracing::info!("Cleaned up {} expired sessions", count);
    }

    Ok(count as i32)
}

pub async fn delete_user_sessions_by_platform(
    user_id: i32,
    platform: Platform,
) -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;
    let platform_string = platform.as_str();

    let tokens: Vec<Uuid> = sessions::table
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::platform.eq(platform_string))
        .select(sessions::token)
        .load::<Uuid>(connection)
        .await
        .map_err(postgres_error)?;

    let token_strings: Vec<String> = tokens.iter().map(|token| token.to_string()).collect();
    if let Err(error) = batch_invalidate_redis_cached_sessions(&token_strings).await {
        tracing::warn!(
            "failed to batch invalidate redis cache during delete_user_sessions_by_platform: {}",
            error
        );
    }

    let count = diesel::delete(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::platform.eq(platform_string)),
    )
    .execute(connection)
    .await
    .map_err(postgres_error)?;

    Ok(count as i32)
}
