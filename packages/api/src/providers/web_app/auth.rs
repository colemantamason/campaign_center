use crate::interfaces::{
    AuthResponse, LoginRequest, OrganizationInfo, OrganizationMembershipInfo, RegisterRequest,
    UserAccountResponse, WithCookie,
};
use crate::models::SubscriptionType;
use crate::postgres::initialize_postgres_pool;
use crate::redis::{
    cache_session, get_cached_session, initialize_redis_pool, invalidate_cached_session,
    CachedSession,
};
use crate::services::{
    authenticate_user, change_password as change_password_service, count_members, create_session,
    delete_session, get_session_from_headers, get_user_by_id, is_secure_request,
    list_user_organizations, register_user, validate_session as validate_session_service,
    WithCookieExt,
};
use dioxus::fullstack::HeaderMap;
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

fn initialize_databases() {
    initialize_postgres_pool().ok();
    initialize_redis_pool().ok();
}

#[post("/api/auth/register", headers: HeaderMap)]
pub async fn register(request: RegisterRequest) -> Result<WithCookie<AuthResponse>, ServerFnError> {
    initialize_databases();

    let user = register_user(
        request.email,
        request.password,
        request.first_name,
        request.last_name,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, None, None)
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

    // create auth response with session cookie set via http header
    let secure = is_secure_request(&headers);
    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: token.clone(),
    };

    Ok(WithCookie::with_session_cookie(
        auth_response,
        &token,
        secure,
    ))
}

#[post("/api/auth/login", headers: HeaderMap)]
pub async fn login(request: LoginRequest) -> Result<WithCookie<AuthResponse>, ServerFnError> {
    initialize_databases();

    let user = authenticate_user(&request.email, &request.password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, None, None)
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

    // create auth response with session cookie set via http header
    let secure = is_secure_request(&headers);
    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: token.clone(),
    };

    Ok(WithCookie::with_session_cookie(
        auth_response,
        &token,
        secure,
    ))
}

/// logout response data (success indicator)
#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

#[post("/api/auth/logout", headers: HeaderMap)]
pub async fn logout() -> Result<WithCookie<LogoutResponse>, ServerFnError> {
    initialize_databases();

    // get session token from cookie and invalidate it
    if let Some(token_string) = get_session_from_headers(&headers) {
        if let Ok(token) = Uuid::parse_str(&token_string) {
            // invalidate redis cache
            invalidate_cached_session(&token_string).await.ok();
            // delete the database session
            delete_session(token).await.ok();
        }
    }

    // return response with set-cookie header that clears the cookie
    Ok(WithCookie::clearing_cookie(LogoutResponse {
        success: true,
    }))
}

#[get("/api/auth/current_user", headers: HeaderMap)]
pub async fn get_current_user() -> Result<Option<UserAccountResponse>, ServerFnError> {
    initialize_databases();

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

        organization_memberships.insert(organization.id, membership_info);
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
    initialize_databases();

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

    Ok(Some(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        session_token: token_string,
    }))
}

#[post("/api/auth/change-password", headers: HeaderMap)]
pub async fn change_password(
    current_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    initialize_databases();

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
