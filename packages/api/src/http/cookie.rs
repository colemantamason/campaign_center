#[cfg(feature = "server")]
use crate::enums::DEFAULT_SESSION_EXPIRY_SECONDS;
#[cfg(feature = "server")]
use axum::response::{IntoResponse, Response};
#[cfg(feature = "server")]
pub use dioxus::fullstack::HeaderMap;
use serde::{Deserialize, Serialize};

// web browser has session token sent via Set-Cookie header (saved in browser cookies)
#[cfg(feature = "server")]
const SESSION_COOKIE_NAME: &str = "session_token";
// mobile apps have sessions token sent via custom X-Session-Token header (stored in secure native storage)
#[cfg(feature = "server")]
pub const SESSION_TOKEN_HEADER: &str = "x-session-token";

// wrapper for server responses that need to set 'httponly' cookies, protects against XSS attacks
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithCookie<T> {
    // the actual response data
    pub data: T,
    // cookie to set (only used on server side via IntoResponse, skipped in serialization)
    #[serde(skip)]
    pub cookie: Option<String>,
    // raw token to send in X-Session-Token header for mobile apps
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

#[cfg(feature = "server")]
pub fn create_session_cookie(token: &str, secure: bool, domain: Option<&str>) -> String {
    let mut parts = vec![
        format!("{}={}", SESSION_COOKIE_NAME, token),
        "Path=/".to_string(),
        "HttpOnly".to_string(),
        "SameSite=Lax".to_string(),
        format!("Max-Age={}", DEFAULT_SESSION_EXPIRY_SECONDS),
    ];

    if secure {
        parts.push("Secure".to_string());
    }

    if let Some(domain) = domain {
        if !domain.is_empty() {
            parts.push(format!("Domain={}", domain));
        }
    }

    parts.join("; ")
}

#[cfg(feature = "server")]
pub fn create_clear_cookie(domain: Option<&str>) -> String {
    let mut parts = vec![
        format!("{}=", SESSION_COOKIE_NAME),
        "Path=/".to_string(),
        "HttpOnly".to_string(),
        "SameSite=Lax".to_string(),
        "Max-Age=0".to_string(),
    ];

    if let Some(domain) = domain {
        if !domain.is_empty() {
            parts.push(format!("Domain={}", domain));
        }
    }

    parts.join("; ")
}

// parse the session token from a HeaderMap (extracted via hoisted extractor)
#[cfg(feature = "server")]
pub fn get_session_from_headers(headers: &HeaderMap) -> Option<String> {
    // check for session cookie (web app)
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_string) = cookie_header.to_str() {
            for cookie in cookie_string.split(';') {
                let cookie = cookie.trim();
                if let Some(value) = cookie.strip_prefix(&format!("{}=", SESSION_COOKIE_NAME)) {
                    let value = value.trim();
                    if !value.is_empty() {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }

    // check for x-session-token header (mobile app)
    if let Some(token_header) = headers.get(SESSION_TOKEN_HEADER) {
        if let Ok(token_string) = token_header.to_str() {
            let token = token_string.trim();
            if !token.is_empty() {
                return Some(token.to_string());
            }
        }
    }

    None
}

// check if the request is using https (for setting secure cookies)
#[cfg(feature = "server")]
pub fn is_secure_request(headers: &HeaderMap) -> bool {
    // check x-forwarded-proto header (common in reverse proxy setups)
    if let Some(proto) = headers.get("x-forwarded-proto") {
        if let Ok(proto_string) = proto.to_str() {
            return proto_string.eq_ignore_ascii_case("https");
        }
    }

    // check origin header
    if let Some(origin) = headers.get("origin") {
        if let Ok(origin_string) = origin.to_str() {
            return origin_string.starts_with("https://");
        }
    }

    // default to checking environment
    std::env::var("ENVIRONMENT")
        .map(|value| value == "production")
        .unwrap_or(false)
}

// get cookie domain from environment variable
#[cfg(feature = "server")]
pub fn get_cookie_domain() -> Option<String> {
    std::env::var("COOKIE_DOMAIN").ok()
}

// extract client ip address from headers (handles proxies)
#[cfg(feature = "server")]
pub fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    // check x-forwarded-for header (common in reverse proxy setups)
    // format: "client, proxy1, proxy2" - first IP is the original client
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded_for.to_str() {
            if let Some(first_ip) = value.split(',').next() {
                let ip = first_ip.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }

    // check x-real-ip header (nginx)
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            let ip = ip.trim();
            if !ip.is_empty() {
                return Some(ip.to_string());
            }
        }
    }

    None
}

// extract user-agent from headers
#[cfg(feature = "server")]
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|header| header.to_str().ok())
        .map(|string| string.to_string())
}

// extension trait to add cookie-setting methods to WithCookie
#[cfg(feature = "server")]
pub trait WithCookieExt<T> {
    /// create a response with a session cookie (for web) and token header (for mobile)
    fn with_session_cookie(data: T, token: &str, secure: bool) -> WithCookie<T>;

    /// create a response that clears the session cookie
    fn clearing_cookie(data: T) -> WithCookie<T>;
}

#[cfg(feature = "server")]
impl<T> WithCookieExt<T> for WithCookie<T> {
    fn with_session_cookie(data: T, token: &str, secure: bool) -> WithCookie<T> {
        let domain = get_cookie_domain();
        WithCookie {
            data,
            cookie: Some(create_session_cookie(token, secure, domain.as_deref())),
            token: Some(token.to_string()),
        }
    }

    fn clearing_cookie(data: T) -> WithCookie<T> {
        let domain = get_cookie_domain();
        WithCookie {
            data,
            cookie: Some(create_clear_cookie(domain.as_deref())),
            token: None,
        }
    }
}

#[cfg(feature = "server")]
impl<T: Serialize> IntoResponse for WithCookie<T> {
    fn into_response(self) -> Response {
        let json_body = match serde_json::to_string(&self.data) {
            Ok(body) => body,
            Err(_) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Serialization error",
                )
                    .into_response()
            }
        };

        let mut builder = Response::builder()
            .status(axum::http::StatusCode::OK)
            .header(axum::http::header::CONTENT_TYPE, "application/json");

        // set the cookie header for web browsers (httponly, not accessible to js)
        if let Some(cookie) = &self.cookie {
            builder = builder.header(axum::http::header::SET_COOKIE, cookie.as_str());
        }

        // set the token header for mobile apps (stored in secure native storage)
        if let Some(token) = &self.token {
            builder = builder.header(SESSION_TOKEN_HEADER, token.as_str());
        }

        builder
            .body(axum::body::Body::from(json_body))
            .unwrap_or_else(|_| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Response build error",
                )
                    .into_response()
            })
    }
}
