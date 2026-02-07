use crate::interfaces::{
    RequestPasswordResetRequest, RequestPasswordResetResponse, ResetPasswordRequest,
    ResetPasswordResponse, ValidateResetTokenRequest, ValidateResetTokenResponse,
};
#[cfg(feature = "server")]
use crate::services::{
    request_password_reset as request_password_reset_service,
    reset_password as reset_password_service,
    validate_reset_token as validate_reset_token_service,
};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use uuid::Uuid;

#[post("/api/auth/request-password-reset")]
pub async fn request_password_reset(
    request: RequestPasswordResetRequest,
) -> Result<RequestPasswordResetResponse, ServerFnError> {
    let result = request_password_reset_service(&request.email)
        .await?;

    // TODO: if result is Some(token), send the reset email with the token link via AWS SES.
    // For now, log the token in development for testing.
    if let Some(token) = result {
        tracing::info!(
            "Password reset token generated for {}: {} (email sending not yet implemented)",
            request.email,
            token
        );
    }

    // always return success to prevent user enumeration
    Ok(RequestPasswordResetResponse { success: true })
}

#[post("/api/auth/reset-password")]
pub async fn reset_password(
    request: ResetPasswordRequest,
) -> Result<ResetPasswordResponse, ServerFnError> {
    let token = Uuid::parse_str(&request.token)
        .map_err(|_| ServerFnError::new("Invalid token format"))?;

    reset_password_service(token, &request.new_password)
        .await?;

    Ok(ResetPasswordResponse { success: true })
}

#[post("/api/auth/validate-reset-token")]
pub async fn validate_reset_token(
    request: ValidateResetTokenRequest,
) -> Result<ValidateResetTokenResponse, ServerFnError> {
    let token = Uuid::parse_str(&request.token)
        .map_err(|_| ServerFnError::new("Invalid token format"))?;

    let valid = validate_reset_token_service(token)
        .await?;

    Ok(ValidateResetTokenResponse { valid })
}
