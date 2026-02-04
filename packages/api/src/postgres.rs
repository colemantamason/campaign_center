use crate::error::AppError;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use std::sync::OnceLock;

static POSTGRES_POOL: OnceLock<Pool<AsyncPgConnection>> = OnceLock::new();

pub fn initialize_postgres_pool() -> Result<(), AppError> {
    let postgres_url = std::env::var("POSTGRES_URL")
        .map_err(|_| AppError::ConfigError("POSTGRES_URL not set".to_string()))?;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&postgres_url);

    let pool = Pool::builder(config)
        .max_size(10)
        .build()
        .map_err(|error| AppError::ConfigError(format!("Postgres pool error: {}", error)))?;

    POSTGRES_POOL
        .set(pool)
        .map_err(|_| AppError::ConfigError("Postgres pool already initialized".to_string()))?;

    tracing::info!("Postgres connection pool initialized");

    Ok(())
}

pub async fn get_postgres_connection() -> Result<Object<AsyncPgConnection>, AppError> {
    let pool = POSTGRES_POOL
        .get()
        .ok_or_else(|| AppError::ConfigError("Postgres pool not initialized".to_string()))?;

    pool.get()
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}
