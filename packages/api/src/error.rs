#[cfg(feature = "server")]
use diesel::result::{DatabaseErrorKind as DieselDatabaseErrorKind, Error as DieselError};
#[cfg(feature = "server")]
use dioxus::prelude::ServerFnError as DioxusServerFnError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Error, Serialize)]
pub enum AppError {
    // Authentication errors
    #[error("Not authenticated")]
    NotAuthenticated,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Session expired")]
    SessionExpired,

    // Authorization errors
    #[error("Not authorized: {0}")]
    NotAuthorized(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    // Validation errors
    #[error("Validation error: {field} - {message}")]
    ValidationError { field: String, message: String },

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    // Resource errors
    #[error("{entity} not found")]
    NotFound { entity: String },

    #[error("{entity} already exists")]
    AlreadyExists { entity: String },

    // Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    // External service errors
    #[error("External service error ({service}): {message}")]
    ExternalServiceError { service: String, message: String },

    // Generic internal error
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl AppError {
    /// Create a validation error for a specific field
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a not found error for an entity
    pub fn not_found(entity: impl Into<String>) -> Self {
        Self::NotFound {
            entity: entity.into(),
        }
    }

    /// Create an already exists error
    pub fn already_exists(entity: impl Into<String>) -> Self {
        Self::AlreadyExists {
            entity: entity.into(),
        }
    }

    /// Create an external service error
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalServiceError {
            service: service.into(),
            message: message.into(),
        }
    }
}

// convert AppError to DioxusServerFnError for server functions
#[cfg(feature = "server")]
impl From<AppError> for DioxusServerFnError {
    fn from(err: AppError) -> Self {
        let json = serde_json::to_string(&err).unwrap_or_else(|_| err.to_string());
        DioxusServerFnError::new(json)
    }
}

// convert DieselError to AppError
#[cfg(feature = "server")]
impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => AppError::NotFound {
                entity: "Record".to_string(),
            },
            DieselError::DatabaseError(DieselDatabaseErrorKind::UniqueViolation, info) => {
                AppError::AlreadyExists {
                    entity: info.message().to_string(),
                }
            }
            _ => AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: err.to_string(),
            },
        }
    }
}
