#[cfg(feature = "server")]
use crate::http::get_session_token_from_headers;
#[cfg(feature = "server")]
use crate::redis::{cache_session, get_cached_session, CachedSession};
#[cfg(feature = "server")]
use crate::services::validate_session as validate_session_service;
#[cfg(feature = "server")]
use axum::{
    extract::{FromRequestParts, Request},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use dioxus::prelude::ServerFnError;
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

    // try redis cache first
    if let Ok(Some(cached)) = get_cached_session(&token_string).await {
        return Some(ValidatedSession {
            session_id: cached.session_id,
            user_id: cached.user_id,
            active_organization_membership_id: cached.active_organization_membership_id,
            token: token_string,
        });
    }

    // fall back to postgres
    let session = validate_session_service(token).await.ok()?;

    let cached = CachedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
    };
    cache_session(&token_string, &cached).await.ok();

    Some(ValidatedSession {
        session_id: session.id,
        user_id: session.user_id,
        active_organization_membership_id: session.active_organization_membership_id,
        token: token_string,
    })
}
