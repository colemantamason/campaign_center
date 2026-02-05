use crate::enums::SubscriptionType;
use crate::http::WithToken;
#[cfg(feature = "server")]
use crate::http::{
    clear_session_token_response, extract_client_ip, extract_user_agent,
    get_session_token_from_headers, set_session_token_response,
};
#[cfg(feature = "server")]
use crate::initialize_databases;
use crate::interfaces::{
    AuthResponse, LoginRequest, LogoutRequest, LogoutResponse, OrganizationInfo,
    OrganizationMembershipInfo, RegisterRequest, UserAccountResponse,
};
#[cfg(feature = "server")]
use crate::redis::{cache_session, get_cached_session, invalidate_cached_session, CachedSession};
#[cfg(feature = "server")]
use crate::services::{
    authenticate_user, change_password as change_password_service, count_members, create_session,
    delete_session, get_user_by_id, list_user_organizations, register_user,
    validate_session as validate_session_service,
};
use dioxus::fullstack::HeaderMap;
use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "server")]
use tracing;
use uuid::Uuid;

#[post("/api/auth/register", headers: HeaderMap)]
pub async fn register(request: RegisterRequest) -> Result<WithToken<AuthResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

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

    let session = create_session(user.id, request.platform, user_agent, ip_address)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let token = session.token.to_string();

    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: None,
    };

    cache_session(&token, &cached).await.ok();

    set_session_token_response(&token, request.platform, &headers);

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    };

    Ok(WithToken::new(auth_response))
}

#[post("/api/auth/login", headers: HeaderMap)]
pub async fn login(request: LoginRequest) -> Result<WithToken<AuthResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let user_agent = extract_user_agent(&headers);
    let ip_address = extract_client_ip(&headers);

    let user = authenticate_user(&request.email, &request.password)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let session = create_session(user.id, request.platform, user_agent, ip_address)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let token = session.token.to_string();

    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: session.active_organization_membership_id,
    };

    cache_session(&token, &cached).await.ok();

    set_session_token_response(&token, request.platform, &headers);

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    };

    Ok(WithToken::new(auth_response))
}

#[post("/api/auth/logout", headers: HeaderMap)]
pub async fn logout(request: LogoutRequest) -> Result<WithToken<LogoutResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let token_string = get_session_token_from_headers(&headers);

    if let Some(ref token_str) = token_string {
        match Uuid::parse_str(token_str) {
            Ok(token) => {
                // invalidate redis cache first (before postgres, to ensure consistency)
                if let Err(error) = invalidate_cached_session(token_str).await {
                    tracing::warn!(
                        "failed to invalidate redis session cache during logout: {}",
                        error
                    );
                }

                if let Err(error) = delete_session(token).await {
                    tracing::warn!("failed to delete postgres session during logout: {}", error);
                }
            }
            Err(_) => {
                tracing::warn!("invalid session token format during logout");
            }
        }
    }

    clear_session_token_response(request.platform);

    Ok(WithToken::new(LogoutResponse { success: true }))
}

#[get("/api/auth/current_user", headers: HeaderMap)]
pub async fn get_current_user() -> Result<Option<UserAccountResponse>, ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let token_string = match get_session_token_from_headers(&headers) {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let token = match Uuid::parse_str(&token_string) {
        Ok(token) => token,
        Err(_) => return Ok(None),
    };

    // get user_id from cache or database
    let (user_id, active_org_id) = match get_cached_session(&token_string).await {
        Ok(Some(cached)) => (cached.user_id, cached.active_organization_membership_id),
        _ => {
            let session = match validate_session_service(token).await {
                Ok(session) => session,
                Err(_) => return Ok(None),
            };

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

    let organizations = list_user_organizations(user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let mut organization_memberships = HashMap::new();

    for (organization, member) in organizations {
        let member_count = count_members(organization.id).await.unwrap_or(0);

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

    let token_string = match get_session_token_from_headers(&headers) {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };

    let token = match Uuid::parse_str(&token_string) {
        Ok(token) => token,
        Err(_) => return Ok(None),
    };

    // get user_id from cache or database
    let user_id = match get_cached_session(&token_string).await {
        Ok(Some(cached)) => cached.user_id,
        _ => {
            let session = match validate_session_service(token).await {
                Ok(session) => session,
                Err(_) => return Ok(None),
            };

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
    }))
}

#[post("/api/auth/change-password", headers: HeaderMap)]
pub async fn change_password(
    current_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    initialize_databases().map_err(|error| ServerFnError::new(error.to_string()))?;

    let token_string = get_session_token_from_headers(&headers)
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
