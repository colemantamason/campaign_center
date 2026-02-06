use crate::enums::{ArticleStatus, ArticleType};
use crate::schema::articles;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use serde_json::Value as JsonValue;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(Postgres))]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub category_id: Option<i32>,
    pub article_type: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub cover_image_url: Option<String>,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_publish_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn get_article_type(&self) -> ArticleType {
        ArticleType::from_str(&self.article_type).unwrap_or(ArticleType::Blog)
    }

    pub fn get_status(&self) -> ArticleStatus {
        ArticleStatus::from_str(&self.status).unwrap_or(ArticleStatus::Draft)
    }

    pub fn is_published(&self) -> bool {
        self.get_status() == ArticleStatus::Published
    }
}

#[derive(Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub author_id: i32,
    pub category_id: Option<i32>,
    pub article_type: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub cover_image_url: Option<String>,
    pub status: String,
}

impl NewArticle {
    pub fn new(
        author_id: i32,
        article_type: ArticleType,
        title: String,
        slug: String,
    ) -> Self {
        Self {
            author_id,
            category_id: None,
            article_type: article_type.as_str().to_string(),
            title,
            slug,
            excerpt: None,
            content: JsonValue::Object(serde_json::Map::new()),
            cover_image_url: None,
            status: ArticleStatus::Draft.as_str().to_string(),
        }
    }

    pub fn set_category(mut self, category_id: i32) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn set_excerpt(mut self, excerpt: String) -> Self {
        self.excerpt = Some(excerpt);
        self
    }

    pub fn set_content(mut self, content: JsonValue) -> Self {
        self.content = content;
        self
    }

    pub fn set_cover_image_url(mut self, url: String) -> Self {
        self.cover_image_url = Some(url);
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = articles)]
pub struct ArticleUpdate {
    pub category_id: Option<Option<i32>>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub excerpt: Option<Option<String>>,
    pub content: Option<JsonValue>,
    pub cover_image_url: Option<Option<String>>,
    pub status: Option<String>,
    pub published_at: Option<Option<DateTime<Utc>>>,
    pub scheduled_publish_at: Option<Option<DateTime<Utc>>>,
    pub updated_at: Option<DateTime<Utc>>,
}
