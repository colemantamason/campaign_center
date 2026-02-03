use crate::database::get_connection;
use crate::error::AppError;
use crate::models::{NewSession, Session, SessionUpdate};
use crate::schema::sessions;
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

// default session expiry in hours (7 days)
const DEFAULT_SESSION_EXPIRY_HOURS: i64 = 168;

pub async fn create_session(
    user_id: i32,
    user_agent: Option<String>,
    ip_address: Option<String>,
) -> Result<Session, AppError> {
    let connection = &mut get_connection().await?;

    let expiry_hours = std::env::var("SESSION_EXPIRY_HOURS")
        .ok()
        .and_then(|string| string.parse().ok())
        .unwrap_or(DEFAULT_SESSION_EXPIRY_HOURS);

    let mut new_session = NewSession::new(user_id, expiry_hours);

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
        .map_err(|error| AppError::DatabaseError(error.to_string()))
}

pub async fn get_session_by_token(token: Uuid) -> Result<Session, AppError> {
    let connection = &mut get_connection().await?;

    let session: Session = sessions::table
        .filter(sessions::token.eq(token))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::DatabaseError(error.to_string()))?
        .ok_or(AppError::NotAuthenticated)?;

    if !session.is_valid() {
        // delete expired session
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

    // update last accessed time
    let connection = &mut get_connection().await?;
    diesel::update(sessions::table.find(session.id))
        .set(sessions::last_accessed_at.eq(Utc::now()))
        .execute(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

    Ok(session)
}

pub async fn set_active_organization(
    session_id: i32,
    organization_id: Option<i32>,
) -> Result<Session, AppError> {
    let connection = &mut get_connection().await?;

    diesel::update(sessions::table.find(session_id))
        .set(SessionUpdate {
            active_organization_membership_id: Some(organization_id),
            last_accessed_at: Some(Utc::now()),
            ..Default::default()
        })
        .get_result::<Session>(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))
}

pub async fn delete_session(token: Uuid) -> Result<(), AppError> {
    let connection = &mut get_connection().await?;

    diesel::delete(sessions::table.filter(sessions::token.eq(token)))
        .execute(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

    Ok(())
}

pub async fn delete_all_user_sessions(user_id: i32) -> Result<i32, AppError> {
    let connection = &mut get_connection().await?;

    let count = diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id)))
        .execute(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

    Ok(count as i32)
}

pub async fn list_user_sessions(user_id: i32) -> Result<Vec<Session>, AppError> {
    let connection = &mut get_connection().await?;

    sessions::table
        .filter(sessions::user_id.eq(user_id))
        .filter(sessions::expires_at.gt(Utc::now()))
        .order(sessions::last_accessed_at.desc())
        .load::<Session>(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))
}

// TODO: schedule this to run periodically
pub async fn cleanup_expired_sessions() -> Result<i32, AppError> {
    let connection = &mut get_connection().await?;

    let count = diesel::delete(sessions::table.filter(sessions::expires_at.lt(Utc::now())))
        .execute(connection)
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

    if count > 0 {
        tracing::info!("Cleaned up {} expired sessions", count);
    }

    Ok(count as i32)
}
