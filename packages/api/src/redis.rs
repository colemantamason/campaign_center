use crate::enums::{ARTICLE_CACHE_EXPIRY_SECONDS, SESSION_EXPIRY_SECONDS};
use crate::error::AppError;
use deadpool_redis::{redis, redis::AsyncCommands, Config, Connection, Pool, Runtime::Tokio1};
use serde::{Deserialize, Serialize};
use std::{env, sync::OnceLock};

static REDIS_POOL: OnceLock<Pool> = OnceLock::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CachedSession {
    pub session_id: i32,
    pub user_id: i32,
    pub active_organization_membership_id: Option<i32>,
    pub is_staff: bool,
}

pub fn is_redis_initialized() -> bool {
    REDIS_POOL.get().is_some()
}

pub fn initialize_redis_pool() -> Result<(), AppError> {
    let redis_url = env::var("REDIS_URL")
        .map_err(|_| AppError::ConfigError("REDIS_URL not set".to_string()))?;

    let config = Config::from_url(&redis_url);

    let pool = config
        .create_pool(Some(Tokio1))
        .map_err(|error| AppError::ConfigError(format!("Redis pool error: {}", error)))?;

    REDIS_POOL
        .set(pool)
        .map_err(|_| AppError::ConfigError("Redis pool already initialized".to_string()))?;

    tracing::info!("Redis connection pool initialized");

    Ok(())
}

async fn get_redis_connection() -> Result<Connection, AppError> {
    let pool = REDIS_POOL
        .get()
        .ok_or_else(|| AppError::ConfigError("Redis pool not initialized".to_string()))?;

    pool.get()
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })
}

pub async fn redis_cache_session(
    token: &str,
    session: &CachedSession,
    expiry: Option<u64>,
) -> Result<(), AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("session:{}", token);

    let value = serde_json::to_string(session)
        .map_err(|error| AppError::InternalError(error.to_string()))?;

    connection
        .set_ex::<&str, &str, ()>(&key, &value, expiry.unwrap_or(SESSION_EXPIRY_SECONDS))
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}

pub async fn get_redis_cached_session(token: &str) -> Result<Option<CachedSession>, AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("session:{}", token);

    let value: Option<String> =
        connection
            .get(&key)
            .await
            .map_err(|error| AppError::ExternalServiceError {
                service: "Redis".to_string(),
                message: error.to_string(),
            })?;

    match value {
        Some(json) => {
            let session: CachedSession = serde_json::from_str(&json)
                .map_err(|error| AppError::InternalError(error.to_string()))?;
            Ok(Some(session))
        }
        None => Ok(None),
    }
}

pub async fn invalidate_redis_cached_session(token: &str) -> Result<(), AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("session:{}", token);

    connection
        .del::<&str, ()>(&key)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}

pub async fn get_redis_session_expiry(token: &str) -> Result<u64, AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("session:{}", token);

    let expiry: i64 = redis::cmd("TTL")
        .arg(&key)
        .query_async(&mut *connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })?;

    // TTL returns -2 if key doesn't exist, -1 if no expiry set
    Ok(expiry.max(0) as u64)
}

pub async fn update_redis_cached_session_active_organization_membership_id(
    token: &str,
    organization_id: Option<i32>,
) -> Result<(), AppError> {
    if let Some(mut session) = get_redis_cached_session(token).await? {
        session.active_organization_membership_id = organization_id;
        let remaining_expiry = get_redis_session_expiry(token).await.ok();
        redis_cache_session(token, &session, remaining_expiry).await?;
    }

    Ok(())
}

pub async fn redis_cache_article_by_slug(slug: &str, json: &str) -> Result<(), AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("article:{}", slug);

    connection
        .set_ex::<&str, &str, ()>(&key, json, ARTICLE_CACHE_EXPIRY_SECONDS)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}

pub async fn get_redis_cached_article_by_slug(slug: &str) -> Result<Option<String>, AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("article:{}", slug);

    let value: Option<String> =
        connection
            .get(&key)
            .await
            .map_err(|error| AppError::ExternalServiceError {
                service: "Redis".to_string(),
                message: error.to_string(),
            })?;

    Ok(value)
}

pub async fn invalidate_redis_cached_article(slug: &str) -> Result<(), AppError> {
    let mut connection = get_redis_connection().await?;

    let key = format!("article:{}", slug);

    connection
        .del::<&str, ()>(&key)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Redis".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}
