pub mod enums;
pub mod error;
pub mod http;
pub mod interfaces;
#[cfg(feature = "server")]
pub mod minio;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod postgres;
pub mod providers;
#[cfg(feature = "server")]
pub mod redis;
#[cfg(feature = "server")]
pub mod schema;
#[cfg(feature = "server")]
pub mod services;
pub mod state;

#[cfg(feature = "server")]
use crate::error::AppError;
#[cfg(feature = "server")]
use crate::minio::{initialize_minio_client, is_minio_initialized};
#[cfg(feature = "server")]
use crate::postgres::{initialize_postgres_pool, is_postgres_initialized};
#[cfg(feature = "server")]
use crate::redis::{initialize_redis_pool, is_redis_initialized};

#[cfg(feature = "server")]
pub fn initialize_services() -> Result<(), AppError> {
    if !is_postgres_initialized() {
        initialize_postgres_pool()?;
    }
    if !is_redis_initialized() {
        initialize_redis_pool()?;
    }
    if !is_minio_initialized() {
        initialize_minio_client()?;
    }
    Ok(())
}
