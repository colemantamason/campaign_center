#[cfg(feature = "server")]
use crate::enums::{Platform, DEFAULT_SESSION_EXPIRY_SECONDS};
#[cfg(feature = "server")]
use axum::http::{header::SET_COOKIE, HeaderName, HeaderValue};
#[cfg(feature = "server")]
pub use dioxus::fullstack::{FullstackContext, HeaderMap};
use serde::{Deserialize, Serialize};

// web uses a cookie for session token
#[cfg(feature = "server")]
const SESSION_COOKIE_NAME: &str = "session_token";

// mobile uses a custom header for session token
#[cfg(feature = "server")]
pub const SESSION_TOKEN_HEADER: &str = "x-session-token";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithToken<T> {
    pub data: T,
    #[serde(skip)]
    pub cookie: Option<String>,
    #[serde(skip)]
    pub token: Option<String>,
}

impl<T> WithToken<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            cookie: None,
            token: None,
        }
    }
}

#[cfg(feature = "server")]
pub fn is_secure_request(headers: &HeaderMap) -> bool {
    if let Some(proto) = headers.get("x-forwarded-proto") {
        if let Ok(proto_string) = proto.to_str() {
            return proto_string.eq_ignore_ascii_case("https");
        }
    }

    // fallback if x-forwarded-proto is not set in production
    std::env::var("ENVIRONMENT")
        .map(|value| value != "development")
        .unwrap_or(true)
}

#[cfg(feature = "server")]
pub fn get_cookie_domain() -> Option<String> {
    std::env::var("COOKIE_DOMAIN").ok()
}

#[cfg(feature = "server")]
fn create_session_cookie(token: &str, secure: bool, domain: Option<&str>) -> String {
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
fn create_clear_cookie(domain: Option<&str>) -> String {
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

#[cfg(feature = "server")]
pub fn set_session_token_response(token: &str, platform: Platform, headers: &HeaderMap) {
    let Some(context) = FullstackContext::current() else {
        return;
    };

    match platform {
        Platform::Web => {
            let secure = is_secure_request(headers);
            let domain = get_cookie_domain();
            let cookie = create_session_cookie(token, secure, domain.as_deref());
            if let Ok(cookie_value) = cookie.parse::<HeaderValue>() {
                context.add_response_header(SET_COOKIE, cookie_value);
            }
        }
        Platform::Mobile => {
            if let Ok(token_value) = token.parse::<HeaderValue>() {
                context.add_response_header(
                    HeaderName::from_static(SESSION_TOKEN_HEADER),
                    token_value,
                );
            }
        }
    }
}

#[cfg(feature = "server")]
pub fn clear_session_token_response(platform: Platform) {
    let Some(context) = FullstackContext::current() else {
        return;
    };

    match platform {
        Platform::Web => {
            let domain = get_cookie_domain();
            let cookie = create_clear_cookie(domain.as_deref());
            if let Ok(cookie_value) = cookie.parse::<HeaderValue>() {
                context.add_response_header(SET_COOKIE, cookie_value);
            }
        }
        Platform::Mobile => {
            context.add_response_header(
                HeaderName::from_static(SESSION_TOKEN_HEADER),
                HeaderValue::from_static(""),
            );
        }
    }
}

#[cfg(feature = "server")]
pub fn get_session_token_from_headers(headers: &HeaderMap) -> Option<String> {
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

#[cfg(feature = "server")]
pub fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    // only trust forwarded headers in production when running behind nginx/load balancer
    let trust_proxy = std::env::var("ENVIRONMENT")
        .map(|value| value != "development")
        .unwrap_or(true);

    if trust_proxy {
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

        if let Some(real_ip) = headers.get("x-real-ip") {
            if let Ok(ip) = real_ip.to_str() {
                let ip = ip.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }

    // in development or when not behind a proxy, return None
    None
}

#[cfg(feature = "server")]
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|header| header.to_str().ok())
        .map(|string| string.to_string())
}
