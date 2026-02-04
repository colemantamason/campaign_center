use serde::{Deserialize, Serialize};

/// wrapper for server responses that need to set httponly cookies
/// the cookie is set via set-cookie header (server-side), not accessible to js
/// this protects against XSS attacks
///
/// for mobile apps, the token is also sent via X-Session-Token header
/// so native apps can store it in secure storage (keychain/encrypted prefs)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithCookie<T> {
    /// the actual response data
    pub data: T,
    /// cookie to set (only used on server side via IntoResponse, skipped in serialization)
    #[serde(skip)]
    pub cookie: Option<String>,
    /// raw token to send in X-Session-Token header for mobile apps
    #[serde(skip)]
    pub token: Option<String>,
}

impl<T> WithCookie<T> {
    // create a response without setting any cookie (used on client side for deserialization)
    pub fn without_cookie(data: T) -> Self {
        Self {
            data,
            cookie: None,
            token: None,
        }
    }
}
