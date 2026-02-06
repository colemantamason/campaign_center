use crate::enums::{SESSION_EXPIRY_SECONDS, SLIDING_SESSION_THRESHOLD_SECONDS};
#[cfg(feature = "server")]
use crate::http::get_session_token_from_headers;
#[cfg(feature = "server")]
use crate::redis::{
    get_redis_cached_session, get_redis_session_expiry, redis_cache_session, CachedSession,
};
#[cfg(feature = "server")]
use crate::services::{
    extend_session_expiry as extend_session_expiry_service,
    validate_session as validate_session_service,
};
#[cfg(feature = "server")]
use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use dioxus::prelude::*;
use std::env;
#[cfg(feature = "server")]
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ValidatedSession {
    pub session_id: i32,
    pub user_id: i32,
    pub active_organization_membership_id: Option<i32>,
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct AuthSession {
    pub current: Option<ValidatedSession>,
}

impl AuthSession {
    pub fn require_auth(self) -> Result<ValidatedSession, ServerFnError> {
        self.current
            .ok_or_else(|| ServerFnError::new("Not authenticated"))
    }
}

#[cfg(feature = "server")]
impl<S: Send + Sync> FromRequestParts<S> for AuthSession {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(AuthSession {
            current: parts.extensions.get::<ValidatedSession>().cloned(),
        })
    }
}

#[cfg(feature = "server")]
pub async fn session_middleware(request: Request, next: Next) -> Response {
    let session = resolve_session(request.headers()).await;

    let mut request = request;
    if let Some(session) = session {
        request.extensions_mut().insert(session);
    }

    next.run(request).await
}

#[cfg(feature = "server")]
async fn resolve_session(headers: &axum::http::HeaderMap) -> Option<ValidatedSession> {
    let token_string = get_session_token_from_headers(headers)?;
    let token = Uuid::parse_str(&token_string).ok()?;

    if let Ok(Some(cached)) = get_redis_cached_session(&token_string).await {
        let validated = ValidatedSession {
            session_id: cached.session_id,
            user_id: cached.user_id,
            active_organization_membership_id: cached.active_organization_membership_id,
            token: token_string.clone(),
        };

        let expiry_seconds = env::var("SESSION_EXPIRY_SECONDS")
            .ok()
            .and_then(|string| string.parse().ok())
            .unwrap_or(SESSION_EXPIRY_SECONDS);
        let extend_when_expiry_below = expiry_seconds - SLIDING_SESSION_THRESHOLD_SECONDS;

        let should_extend = get_redis_session_expiry(&token_string)
            .await
            .ok()
            .map(|expiry| expiry < extend_when_expiry_below)
            .unwrap_or(false);

        if should_extend {
            let session_id = cached.session_id;
            let cached_clone = cached.clone();
            let token_for_task = token_string.clone();
            let new_expiry = expiry_seconds as u64;

            // spawn the postgres update + redis re-cache so we don't block the response
            spawn(async move {
                if let Err(error) = extend_session_expiry_service(session_id).await {
                    tracing::warn!(
                        "failed to extend session expiry for session {}: {}",
                        session_id,
                        error
                    );
                    return;
                }
                if let Err(error) =
                    redis_cache_session(&token_for_task, &cached_clone, Some(new_expiry)).await
                {
                    tracing::warn!(
                        "failed to re-cache session {} after sliding extend: {}",
                        session_id,
                        error
                    );
                }
            });
        }

        return Some(validated);
    }

    // fall back to postgres
    let session = validate_session_service(token).await.ok()?;

    let new_expiry = (session.expires_at - Utc::now()).num_seconds().max(0) as u64;

    let cached = CachedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
    };
    redis_cache_session(&token_string, &cached, Some(new_expiry))
        .await
        .ok();

    Some(ValidatedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
        token: token_string,
    })
}
