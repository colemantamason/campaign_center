use crate::error::AppError;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::InternalError(format!("Password hashing failed: {}", error)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|error| AppError::InternalError(format!("Invalid password hash: {}", error)))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::validation(
            "password",
            "Password must be at least 8 characters",
        ));
    }

    if password.len() > 128 {
        return Err(AppError::validation(
            "password",
            "Password must be less than 128 characters",
        ));
    }

    let has_letter = password.chars().any(|character| character.is_alphabetic());
    let has_number = password.chars().any(|character| character.is_numeric());

    if !has_letter || !has_number {
        return Err(AppError::validation(
            "password",
            "Password must contain at least one letter and one number",
        ));
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), AppError> {
    if email.is_empty() {
        return Err(AppError::validation("email", "Email is required"));
    }

    if email.len() > 128 {
        return Err(AppError::validation(
            "email",
            "Email must be less than 128 characters",
        ));
    }

    // check for text before and after an '@' symbol
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(AppError::validation("email", "Invalid email format"));
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return Err(AppError::validation("email", "Invalid email format"));
    }

    if !domain.contains('.') {
        return Err(AppError::validation("email", "Invalid email domain"));
    }

    Ok(())
}
