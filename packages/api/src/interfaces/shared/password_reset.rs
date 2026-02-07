use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestPasswordResetRequest {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct RequestPasswordResetResponse {
    // always true — does not reveal whether the email exists
    pub success: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResetPasswordResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ValidateResetTokenRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ValidateResetTokenResponse {
    pub valid: bool,
}
