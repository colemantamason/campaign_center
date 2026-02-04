pub use crate::interfaces::WithCookie;
use axum::response::{IntoResponse, Response};
pub use dioxus::fullstack::HeaderMap;
use serde::Serialize;

// web browser has session token sent via Set-Cookie header (saved in browser cookies)
const SESSION_COOKIE_NAME: &str = "session_token";
// mobile apps have sessions token sent via custom X-Session-Token header (stored in secure native storage)
const SESSION_TOKEN_HEADER: &str = "x-session-token";
// default session max-age in seconds (7 days)
const SESSION_MAX_AGE: i64 = 604800;

// TODO: consider SameSite=None; Secure if using cross-site requests
// TODO: consider adding Domain= if using subdomains
// TODO: figure out if development uses HTTPS and can remove Secure flag
pub fn create_session_cookie(token: &str, secure: bool) -> String {
    let secure_flag = if secure { "; Secure" } else { "" };
    format!(
        "{}={}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}{}",
        SESSION_COOKIE_NAME, token, SESSION_MAX_AGE, secure_flag
    )
}

pub fn create_clear_cookie() -> String {
    format!(
        "{}=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0",
        SESSION_COOKIE_NAME
    )
}

// parse the session token from a HeaderMap (extracted via hoisted extractor)
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
    std::env::var("SECURE_COOKIES")
        .map(|value| value == "true" || value == "1")
        .unwrap_or(false)
}

// extension trait to add cookie-setting methods to WithCookie
pub trait WithCookieExt<T> {
    /// create a response with a session cookie (for web) and token header (for mobile)
    fn with_session_cookie(data: T, token: &str, secure: bool) -> WithCookie<T>;

    /// create a response that clears the session cookie
    fn clearing_cookie(data: T) -> WithCookie<T>;
}

impl<T> WithCookieExt<T> for WithCookie<T> {
    fn with_session_cookie(data: T, token: &str, secure: bool) -> WithCookie<T> {
        WithCookie {
            data,
            cookie: Some(create_session_cookie(token, secure)),
            token: Some(token.to_string()),
        }
    }

    fn clearing_cookie(data: T) -> WithCookie<T> {
        WithCookie {
            data,
            cookie: Some(create_clear_cookie()),
            token: None,
        }
    }
}

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
