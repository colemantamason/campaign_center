use crate::enums::{ArticleStatus, ArticleType};
use crate::interfaces::{ArticleAuthorInfo, ArticleCategoryInfo, ArticleTagInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Deserialize, Serialize)]
pub struct CreateArticleRequest {
    pub article_type: ArticleType,
    pub title: String,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<JsonValue>,
    pub cover_image_url: Option<String>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<JsonValue>,
    pub cover_image_url: Option<String>,
    pub category_id: Option<i32>,
    pub tag_ids: Option<Vec<i32>>,
    pub scheduled_publish_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct ListArticlesRequest {
    pub article_type: Option<ArticleType>,
    pub status: Option<ArticleStatus>,
    pub category_id: Option<i32>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleResponse {
    pub id: i32,
    pub article_type: ArticleType,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub cover_image_url: Option<String>,
    pub status: ArticleStatus,
    pub author: ArticleAuthorInfo,
    pub category: Option<ArticleCategoryInfo>,
    pub tags: Vec<ArticleTagInfo>,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_publish_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleRevisionResponse {
    pub id: i32,
    pub revision_number: i32,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub published_by: ArticleAuthorInfo,
    pub created_at: DateTime<Utc>,
}
