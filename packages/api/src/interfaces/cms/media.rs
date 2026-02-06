use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UploadMediaRequest {
    pub original_filename: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub alt_text: Option<String>,
    // TODO: actual file data will be handled via multipart upload or presigned URL
}

#[derive(Deserialize, Serialize)]
pub struct ListMediaRequest {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MediaAssetResponse {
    pub id: i32,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub url: String,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MediaListResponse {
    pub assets: Vec<MediaAssetResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
