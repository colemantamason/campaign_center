use crate::database;
use crate::interfaces::web_app::{AuthResponse, LoginRequest, RegisterRequest};
use crate::services::{
    authenticate_user, change_password as change_password_service, create_session, delete_session,
    get_user_by_id, register_user, validate_session as validate_session_service,
};
use dioxus::prelude::*;
use uuid::Uuid;

#[server]
pub async fn register(req: RegisterRequest) -> Result<AuthResponse, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let user = register_user(req.email, req.password, req.first_name, req.last_name)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, None, None)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: session.token.to_string(),
    })
}

#[server]
pub async fn login(req: LoginRequest) -> Result<AuthResponse, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let user = authenticate_user(&req.email, &req.password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, None, None)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: session.token.to_string(),
    })
}

#[server]
pub async fn logout(session_token: String) -> Result<(), ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    delete_session(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[server]
pub async fn validate_session(session_token: String) -> Result<AuthResponse, ServerFnError> {
    // initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session_service(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let user = get_user_by_id(session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: session.token.to_string(),
    })
}

#[server]
pub async fn change_password(
    session_token: String,
    current_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    // Initialize database connection if needed
    database::init_pool().ok();

    let token =
        Uuid::parse_str(&session_token).map_err(|_| ServerFnError::new("Invalid session token"))?;

    let session = validate_session_service(token)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    change_password_service(session.user_id, &current_password, &new_password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
