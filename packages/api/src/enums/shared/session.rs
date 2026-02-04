// shared session expiry constant (7 days in seconds)
// used by both redis cache TTL and postgres session expiry
pub const DEFAULT_SESSION_EXPIRY_SECONDS: i64 = 604800;
