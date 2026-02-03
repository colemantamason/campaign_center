use crate::error::AppError;
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool},
        AsyncDieselConnectionManager,
    },
    AsyncPgConnection,
};
use std::sync::OnceLock;

pub type DatabasePool = Pool<AsyncPgConnection>;
pub type PooledConnection = Object<AsyncPgConnection>;

static DATABASE_POOL: OnceLock<DatabasePool> = OnceLock::new();

// initialize the database connection pool
pub fn init_pool() -> Result<(), AppError> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::ConfigError("DATABASE_URL not set".to_string()))?;

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&database_url);

    let pool = Pool::builder(config)
        .max_size(10)
        .build()
        .map_err(|error| AppError::DatabaseError(error.to_string()))?;

    DATABASE_POOL
        .set(pool)
        .map_err(|_| AppError::ConfigError("Pool already initialized".to_string()))?;

    tracing::info!("Database connection pool initialized");
    Ok(())
}

// get a connection from the pool
pub async fn get_connection() -> Result<PooledConnection, AppError> {
    let pool = DATABASE_POOL
        .get()
        .ok_or_else(|| AppError::ConfigError("Database pool not initialized".to_string()))?;
    pool.get()
        .await
        .map_err(|error| AppError::DatabaseError(error.to_string()))
}
