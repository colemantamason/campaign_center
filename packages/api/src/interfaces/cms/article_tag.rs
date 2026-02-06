use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchTagsRequest {
    pub query: String,
    pub limit: Option<i64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}
