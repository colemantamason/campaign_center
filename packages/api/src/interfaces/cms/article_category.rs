use crate::enums::ArticleType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub article_type: ArticleType,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct ReorderCategoriesRequest {
    // vec of (category_id, new_sort_order) pairs
    pub order: Vec<(i32, i32)>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub article_type: ArticleType,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
