use crate::enums::{Platform, SESSION_EXPIRY_SECONDS, SLIDING_SESSION_THRESHOLD_SECONDS};
use crate::error::AppError;
use crate::models::{NewSession, Session, SessionUpdate};
use crate::postgres::get_postgres_connection;
use crate::redis::invalidate_redis_cached_session;
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn get_session_by_token(token: Uuid) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let session: Session = sessions::table
        .filter(sessions::token.eq(token))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn set_active_organization(
    session_id: i32,
    organization_id: Option<i32>,
) -> Result<Session, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(sessions::table.find(session_id))
        .set(SessionUpdate {
            active_organization_membership_id: Some(organization_id),
            last_accessed_at: Some(Utc::now()),
            ..Default::default()
        })
        .get_result::<Session>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn delete_session(token: Uuid) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::delete(sessions::table.filter(sessions::token.eq(token)))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}

pub async fn delete_all_user_sessions(user_id: i32) -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let tokens: Vec<Uuid> = sessions::table
        .filter(sessions::user_id.eq(user_id))
        .select(sessions::token)
        .load::<Uuid>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    // invalidate each session in redis cache and postgres
    for token in &tokens {
        if let Err(error) = invalidate_redis_cached_session(&token.to_string()).await {
            tracing::warn!(
                "failed to invalidate redis cache for session {} during delete_all_user_sessions: {}",
                token,
                error
            );
        }
    }

    let count = diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id)))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

// TODO: schedule this to run periodically
pub async fn cleanup_expired_sessions() -> Result<i32, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let count = diesel::delete(sessions::table.filter(sessions::expires_at.lt(Utc::now())))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    // invalidate each session in redis cache and postgres
    for token in &tokens {
        if let Err(error) = invalidate_redis_cached_session(&token.to_string()).await {
            tracing::warn!(
                "failed to invalidate redis cache for session {} during delete_user_sessions_by_platform: {}",
                token,
                error
            );
        }
    }

    let count = diesel::delete(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::platform.eq(platform_string)),
    )
    .execute(connection)
    .await
    .map_err(|error| AppError::ExternalServiceError {
        service: "Postgres".to_string(),
        message: error.to_string(),
    })?;

    Ok(count as i32)
}
