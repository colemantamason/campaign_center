use crate::enums::ArticleType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleAuthorInfo {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleCategoryInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ArticleTagInfo {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PublicArticleResponse {
    pub id: i32,
    pub article_type: ArticleType,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub cover_image_url: Option<String>,
    pub author: ArticleAuthorInfo,
    pub category: Option<ArticleCategoryInfo>,
    pub tags: Vec<ArticleTagInfo>,
    pub published_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct PublicArticleListResponse {
    pub articles: Vec<PublicArticleResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Deserialize, Serialize)]
pub struct ListPublicArticlesRequest {
    pub article_type: ArticleType,
    pub category_slug: Option<String>,
    pub tag_slug: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
