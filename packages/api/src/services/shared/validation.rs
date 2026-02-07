use crate::error::AppError;

// these are based on the database schema (VARCHAR lengths)
pub const MAX_ORGANIZATION_NAME_LENGTH: usize = 255;
pub const MAX_ORGANIZATION_SLUG_LENGTH: usize = 100;
pub const MAX_USER_NAME_LENGTH: usize = 100;
pub const MAX_ARTICLE_TITLE_LENGTH: usize = 500;
pub const MAX_ARTICLE_SLUG_LENGTH: usize = 500;
pub const MAX_CATEGORY_NAME_LENGTH: usize = 100;
pub const MAX_CATEGORY_SLUG_LENGTH: usize = 100;
pub const MAX_TAG_NAME_LENGTH: usize = 100;
pub const MAX_TAG_SLUG_LENGTH: usize = 100;
pub const MAX_FILENAME_LENGTH: usize = 255;

pub const MAX_MEDIA_FILE_SIZE_BYTES: i64 = 50 * 1024 * 1024;
pub const ALLOWED_MEDIA_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "image/svg+xml",
    "image/avif",
    "application/pdf",
    "video/mp4",
    "video/webm",
];

pub fn validate_max_length(field: &str, value: &str, max_length: usize) -> Result<(), AppError> {
    if value.len() > max_length {
        return Err(AppError::validation(
            field,
            format!("Must be {} characters or fewer", max_length),
        ));
    }
    Ok(())
}

pub fn validate_required_string(
    field: &str,
    value: &str,
    max_length: usize,
) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::validation(
            field,
            format!("{} is required", field),
        ));
    }
    validate_max_length(field, value, max_length)
}

pub fn validate_optional_string(
    field: &str,
    value: &Option<String>,
    max_length: usize,
) -> Result<(), AppError> {
    if let Some(ref v) = value {
        validate_max_length(field, v, max_length)?;
    }
    Ok(())
}

pub fn validate_slug(field: &str, value: &str, max_length: usize) -> Result<(), AppError> {
    validate_max_length(field, value, max_length)?;

    if !value
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(AppError::validation(
            field,
            "Slug must contain only lowercase letters, numbers, and hyphens",
        ));
    }

    Ok(())
}

pub fn validate_optional_slug(
    field: &str,
    value: &Option<String>,
    max_length: usize,
) -> Result<(), AppError> {
    if let Some(ref v) = value {
        validate_slug(field, v, max_length)?;
    }
    Ok(())
}

pub fn validate_media_file(
    filename: &str,
    mime_type: &str,
    file_size_bytes: i64,
) -> Result<(), AppError> {
    if file_size_bytes <= 0 {
        return Err(AppError::validation(
            "file_size_bytes",
            "File size must be greater than 0",
        ));
    }

    if file_size_bytes > MAX_MEDIA_FILE_SIZE_BYTES {
        let max_mb = MAX_MEDIA_FILE_SIZE_BYTES / (1024 * 1024);
        return Err(AppError::validation(
            "file_size_bytes",
            format!("File size exceeds maximum of {} MB", max_mb),
        ));
    }

    if !ALLOWED_MEDIA_MIME_TYPES.contains(&mime_type) {
        return Err(AppError::validation(
            "mime_type",
            format!(
                "File type '{}' is not allowed. Allowed types: {}",
                mime_type,
                ALLOWED_MEDIA_MIME_TYPES.join(", ")
            ),
        ));
    }

    // basic extension sanity check — prevent uploading files with dangerous extensions
    let extension = filename.rsplit('.').next().unwrap_or("").to_lowercase();

    let dangerous_extensions = [
        "exe", "bat", "cmd", "sh", "ps1", "msi", "dll", "so", "dylib",
    ];

    if dangerous_extensions.contains(&extension.as_str()) {
        return Err(AppError::validation(
            "original_filename",
            format!("File extension '.{}' is not allowed", extension),
        ));
    }

    Ok(())
}
