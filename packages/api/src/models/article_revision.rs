use crate::schema::article_revisions;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};
use serde_json::Value as JsonValue;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = article_revisions)]
#[diesel(check_for_backend(Postgres))]
pub struct ArticleRevision {
    pub id: i32,
    pub article_id: i32,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub revision_number: i32,
    pub published_by: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = article_revisions)]
pub struct NewArticleRevision {
    pub article_id: i32,
    pub title: String,
    pub excerpt: Option<String>,
    pub content: JsonValue,
    pub revision_number: i32,
    pub published_by: i32,
}

impl NewArticleRevision {
    pub fn new(
        article_id: i32,
        title: String,
        content: JsonValue,
        revision_number: i32,
        published_by: i32,
    ) -> Self {
        Self {
            article_id,
            title,
            excerpt: None,
            content,
            revision_number,
            published_by,
        }
    }

    pub fn set_excerpt(mut self, excerpt: String) -> Self {
        self.excerpt = Some(excerpt);
        self
    }
}
