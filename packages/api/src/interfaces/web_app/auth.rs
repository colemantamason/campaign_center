use serde::{Deserialize, Serialize};

// request to register a new user
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

// request to login
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// auth response with user data
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user_id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub session_token: String,
}
