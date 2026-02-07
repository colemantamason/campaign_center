use crate::enums::SubscriptionType;
#[cfg(feature = "server")]
use crate::http::{
    clear_session_token_response, extract_client_ip, extract_user_agent, set_session_token_response,
};
use crate::http::{AuthSession, WithToken};
use crate::interfaces::{
    AuthResponse, LoginRequest, LogoutRequest, LogoutResponse, OrganizationInfo,
    OrganizationMembershipInfo, RegisterRequest, UserAccountResponse,
};
#[cfg(feature = "server")]
use crate::redis::{invalidate_redis_cached_session, redis_cache_session, CachedSession};
#[cfg(feature = "server")]
use crate::services::{
    authenticate_user, batch_count_members, change_password as change_password_service,
    create_session, delete_session, get_user_by_id, list_user_organizations, register_user,
};
use dioxus::fullstack::HeaderMap;
use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "server")]
use tracing;
use uuid::Uuid;

#[post("/api/auth/register", headers: HeaderMap)]
pub async fn register(request: RegisterRequest) -> Result<WithToken<AuthResponse>, ServerFnError> {
    let user_agent = extract_user_agent(&headers);
    let ip_address = extract_client_ip(&headers);

    let user = register_user(
        request.email,
        request.password,
        request.first_name,
        request.last_name,
        false, // is_staff can only bet set manually in the database, not through registration
    )
    .await?;

    let session = create_session(user.id, request.platform, user_agent, ip_address)
        .await?;

    let token = session.token.to_string();

    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: None,
        is_staff: user.is_staff,
    };

    if let Err(error) = redis_cache_session(&token, &cached, None).await {
        tracing::warn!(
            "failed to cache session in Redis during register: {}",
            error
        );
    }

    set_session_token_response(&token, request.platform, &headers);

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        is_staff: user.is_staff,
    };

    Ok(WithToken::new(auth_response))
}

#[post("/api/auth/login", headers: HeaderMap)]
pub async fn login(request: LoginRequest) -> Result<WithToken<AuthResponse>, ServerFnError> {
    let user_agent = extract_user_agent(&headers);
    let ip_address = extract_client_ip(&headers);

    let user = authenticate_user(&request.email, &request.password)
        .await?;

    let session = create_session(user.id, request.platform, user_agent, ip_address)
        .await?;

    let token = session.token.to_string();

    let cached = CachedSession {
        session_id: session.id,
        user_id: user.id,
        active_organization_membership_id: session.active_organization_membership_id,
        is_staff: user.is_staff,
    };

    if let Err(error) = redis_cache_session(&token, &cached, None).await {
        tracing::warn!("failed to cache session in Redis during login: {}", error);
    }

    set_session_token_response(&token, request.platform, &headers);

    let auth_response = AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        is_staff: user.is_staff,
    };

    Ok(WithToken::new(auth_response))
}

#[post("/api/auth/logout", auth: AuthSession)]
pub async fn logout(request: LogoutRequest) -> Result<WithToken<LogoutResponse>, ServerFnError> {
    if let Some(ref session) = auth.current {
        // invalidate redis cache first (before postgres, to ensure consistency)
        if let Err(error) = invalidate_redis_cached_session(&session.token).await {
            tracing::warn!(
                "failed to invalidate redis session cache during logout: {}",
                error
            );
        }

        if let Ok(token_uuid) = Uuid::parse_str(&session.token) {
            if let Err(error) = delete_session(token_uuid).await {
                tracing::warn!("failed to delete postgres session during logout: {}", error);
            }
        }
    }

    clear_session_token_response(request.platform);

    Ok(WithToken::new(LogoutResponse { success: true }))
}

#[get("/api/auth/current_user", auth: AuthSession)]
pub async fn get_current_user() -> Result<Option<UserAccountResponse>, ServerFnError> {
    let Some(session) = auth.current else {
        return Ok(None);
    };

    let user = get_user_by_id(session.user_id)
        .await?;

    let organizations = list_user_organizations(session.user_id)
        .await?;

    let org_ids: Vec<i32> = organizations.iter().map(|(org, _)| org.id).collect();
    let member_counts = batch_count_members(&org_ids)
        .await?;

    let mut organization_memberships = HashMap::new();

    for (organization, member) in organizations {
        let member_count = member_counts.get(&organization.id).copied().unwrap_or(0);

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
                avatar_url: organization.avatar_url.clone(),
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
        avatar_url: user.avatar_url,
        active_organization_membership_id: session.active_organization_membership_id,
        organization_memberships,
    }))
}

#[post("/api/auth/validate", auth: AuthSession)]
pub async fn validate_session() -> Result<Option<AuthResponse>, ServerFnError> {
    let Some(session) = auth.current else {
        return Ok(None);
    };

    let user = get_user_by_id(session.user_id)
        .await?;

    Ok(Some(AuthResponse {
        user_id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        is_staff: user.is_staff,
    }))
}

#[post("/api/auth/change-password", auth: AuthSession)]
pub async fn change_password(
    current_password: String,
    new_password: String,
) -> Result<(), ServerFnError> {
    let session = auth.require_auth()?;

    let current_token = Uuid::parse_str(&session.token).ok();

    change_password_service(
        session.user_id,
        &current_password,
        &new_password,
        current_token,
    )
    .await?;

    Ok(())
}
