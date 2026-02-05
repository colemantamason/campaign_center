use crate::enums::SubscriptionType;
use crate::error::AppError;
use crate::http::WithCookie;
#[cfg(feature = "server")]
use crate::http::{
    create_clear_cookie, create_session_cookie, extract_client_ip, extract_user_agent,
    get_cookie_domain, get_session_from_headers, is_secure_request, SESSION_TOKEN_HEADER,
};
use crate::interfaces::{
    AuthResponse, LoginRequest, OrganizationInfo, OrganizationMembershipInfo, RegisterRequest,
    UserAccountResponse,
};
#[cfg(feature = "server")]
use crate::postgres::{initialize_postgres_pool, is_postgres_initialized};
#[cfg(feature = "server")]
use crate::redis::{
    cache_session, get_cached_session, initialize_redis_pool, invalidate_cached_session,
    is_redis_initialized, CachedSession,
};
#[cfg(feature = "server")]
use crate::services::{
    authenticate_user, change_password as change_password_service, count_members, create_session,
    delete_session, get_user_by_id, list_user_organizations, register_user,
    validate_session as validate_session_service,
};
#[cfg(feature = "server")]
use axum::http::{header::SET_COOKIE, HeaderName, HeaderValue};
use dioxus::fullstack::HeaderMap;
use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "server")]
use tracing;
use uuid::Uuid;

#[cfg(feature = "server")]
fn initialize_databases() -> Result<(), AppError> {
    if !is_postgres_initialized() {
        initialize_postgres_pool()?;
    }
    if !is_redis_initialized() {
        initialize_redis_pool()?;
    }
    Ok(())
}

#[post("/api/auth/register", headers: HeaderMap)]
pub async fn register(request: RegisterRequest) -> Result<WithCookie<AuthResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // extract user agent and ip from headers
    let user_agent = extract_user_agent(&headers);
    let ip_address = extract_client_ip(&headers);

    let user = register_user(
        request.email,
        request.password,
        request.first_name,
        request.last_name,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, user_agent, ip_address)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let token = session.token.to_string();

    // cache the session in redis
    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: None,
    };
    cache_session(&token, &cached).await.ok();

    // set session token via dioxus fullstack context
    // session token is NOT included in the JSON response body (security: prevents XSS token theft)
    // web browsers receive the token via HttpOnly Set-Cookie header
    // mobile apps receive the token via X-Session-Token header (stored in secure native storage)
    let secure = is_secure_request(&headers);
    let domain = get_cookie_domain();
    let cookie = create_session_cookie(&token, secure, domain.as_deref());

    if let Some(context) = FullstackContext::current() {
        // set cookie for web browsers
        if let Ok(cookie_value) = cookie.parse::<HeaderValue>() {
            context.add_response_header(SET_COOKIE, cookie_value);
        }
        // set token header for mobile apps
        if let Ok(token_value) = token.parse::<HeaderValue>() {
            context.add_response_header(
                axum::http::HeaderName::from_static(SESSION_TOKEN_HEADER),
                token_value,
            );
        }
    }

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    };

    Ok(WithCookie::without_cookie(auth_response))
}

#[post("/api/auth/login", headers: HeaderMap)]
pub async fn login(request: LoginRequest) -> Result<WithCookie<AuthResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // extract user agent and ip from headers
    let user_agent = extract_user_agent(&headers);
    let ip_address = extract_client_ip(&headers);

    let user = authenticate_user(&request.email, &request.password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, user_agent, ip_address)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let token = session.token.to_string();

    // cache the session in redis
    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: session.active_organization_membership_id,
    };
    cache_session(&token, &cached).await.ok();

    // set session token via dioxus fullstack context
    // session token is NOT included in the JSON response body (security: prevents XSS token theft)
    // web browsers receive the token via HttpOnly Set-Cookie header
    // mobile apps receive the token via X-Session-Token header (stored in secure native storage)
    let secure = is_secure_request(&headers);
    let domain = get_cookie_domain();
    let cookie = create_session_cookie(&token, secure, domain.as_deref());

    if let Some(context) = FullstackContext::current() {
        // set cookie for web browsers
        if let Ok(cookie_value) = cookie.parse::<HeaderValue>() {
            context.add_response_header(SET_COOKIE, cookie_value);
        }
        // set token header for mobile apps
        if let Ok(token_value) = token.parse::<HeaderValue>() {
            context.add_response_header(HeaderName::from_static(SESSION_TOKEN_HEADER), token_value);
        }
    }

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    };

    Ok(WithCookie::without_cookie(auth_response))
}

/// logout response data (success indicator)
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

#[post("/api/auth/logout", headers: HeaderMap)]
pub async fn logout() -> Result<WithCookie<LogoutResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // get session token from cookie and invalidate it
    if let Some(token_string) = get_session_from_headers(&headers) {
        if let Ok(token) = Uuid::parse_str(&token_string) {
            // invalidate redis cache first (before postgres, to ensure consistency)
            if let Err(error) = invalidate_cached_session(&token_string).await {
                tracing::warn!(
                    "failed to invalidate redis session cache during logout: {}",
                    error
                );
            }
            // delete the database session
            if let Err(error) = delete_session(token).await {
                tracing::warn!("failed to delete postgres session during logout: {}", error);
            }
        }
    }

    // clear session via dioxus fullstack context
    // web browsers: clear cookie
    // mobile apps: send empty token header (client should delete from secure storage)
    let domain = get_cookie_domain();
    let cookie = create_clear_cookie(domain.as_deref());

    if let Some(context) = FullstackContext::current() {
        // clear cookie for web browsers
        if let Ok(cookie_value) = cookie.parse::<HeaderValue>() {
            context.add_response_header(SET_COOKIE, cookie_value);
        }
        // send empty token header for mobile apps (signals logout)
        context.add_response_header(
            HeaderName::from_static(SESSION_TOKEN_HEADER),
            HeaderValue::from_static(""),
        );
    }

    Ok(WithCookie::without_cookie(LogoutResponse { success: true }))
}

#[get("/api/auth/current_user", headers: HeaderMap)]
pub async fn get_current_user() -> Result<Option<UserAccountResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // get session token from cookie
    let token_string = match get_session_from_headers(&headers) {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let token = match Uuid::parse_str(&token_string) {
        Ok(token) => token,
        Err(_) => return Ok(None),
    };

    // try to get session from redis cache first
    let (user_id, active_org_id) = match get_cached_session(&token_string).await {
        Ok(Some(cached)) => (cached.user_id, cached.active_organization_membership_id),
        _ => {
            // fall back to database
            let session = match validate_session_service(token).await {
                Ok(session) => session,
                Err(_) => return Ok(None),
            };

            // cache the session for future requests
            let cached = CachedSession {
                session_id: session.id,
                user_id: session.user_id,
                active_organization_membership_id: session.active_organization_membership_id,
            };
            cache_session(&token_string, &cached).await.ok();

            (session.user_id, session.active_organization_membership_id)
        }
    };

    let user = get_user_by_id(user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // get user's organizations
    let organizations = list_user_organizations(user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let mut organization_memberships = HashMap::new();

    for (organization, member) in organizations {
        let member_count = count_members(organization.id).await.unwrap_or(0);

        // get organization permissions from subscriptions
        let mut permissions = HashMap::new();
        for subscription in &organization.subscriptions {
            if let Some(sub_str) = subscription {
                if let Some(sub_type) = SubscriptionType::from_str(sub_str) {
                    permissions.insert(sub_type, true);
                }
            }
        }

        let membership_info = OrganizationMembershipInfo {
            id: member.id,
            organization_id: organization.id,
            organization: OrganizationInfo {
                id: organization.id,
                name: organization.name.clone(),
                avatar_url: None,
                member_count,
            },
            user_role: member.get_role(),
            permissions,
        };

        // Key by membership_id (not organization.id) to match active_organization_membership_id
        organization_memberships.insert(member.id, membership_info);
    }

    Ok(Some(UserAccountResponse {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        avatar_url: None,
        active_organization_membership_id: active_org_id,
        organization_memberships,
    }))
}

#[post("/api/auth/validate", headers: HeaderMap)]
pub async fn validate_session() -> Result<Option<AuthResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // get session token from cookie
    let token_string = match get_session_from_headers(&headers) {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let token = match Uuid::parse_str(&token_string) {
        Ok(token) => token,
        Err(_) => return Ok(None),
    };

    // try redis cache first
    let user_id = match get_cached_session(&token_string).await {
        Ok(Some(cached)) => cached.user_id,
        _ => {
            // fall back to database validation
            let session = match validate_session_service(token).await {
                Ok(session) => session,
                Err(_) => return Ok(None),
            };

            // cache the session
            let cached = CachedSession {
                session_id: session.id,
                user_id: session.user_id,
                active_organization_membership_id: session.active_organization_membership_id,
            };
            cache_session(&token_string, &cached).await.ok();

            session.user_id
        }
    };

    let user = get_user_by_id(user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    // note: session token is not included in the response body for security
    // clients should use the token from the cookie or X-Session-Token header
    Ok(Some(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    }))
}

#[post("/api/auth/change-password", headers: HeaderMap)]
pub async fn change_password(
    current_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    // get session token from cookie
    let token_string = get_session_from_headers(&headers)
        .ok_or_else(|| ServerFnError::new("Not authenticated"))?;

    let token =
        Uuid::parse_str(&token_string).map_err(|_| ServerFnError::new("Invalid session token"))?;

    // get user_id from cache or database
    let user_id = match get_cached_session(&token_string).await {
        Ok(Some(cached)) => cached.user_id,
        _ => {
            let session = validate_session_service(token)
                .await
                .map_err(|error| ServerFnError::new(error.to_string()))?;
            session.user_id
        }
    };

    change_password_service(user_id, &current_password, &new_password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
