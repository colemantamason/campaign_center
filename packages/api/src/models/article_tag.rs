use crate::schema::{article_tags, articles_tags};
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = article_tags)]
#[diesel(check_for_backend(Postgres))]
pub struct ArticleTag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = article_tags)]
pub struct NewArticleTag {
    pub name: String,
    pub slug: String,
}

impl NewArticleTag {
    pub fn new(name: String, slug: String) -> Self {
        Self { name, slug }
    }
}

// join table linking articles to tags (many-to-many)
#[derive(Identifiable, Insertable, Queryable, Selectable)]
#[diesel(table_name = articles_tags)]
#[diesel(primary_key(article_id, tag_id))]
#[diesel(check_for_backend(Postgres))]
pub struct ArticleTagLink {
    pub article_id: i32,
    pub tag_id: i32,
}

impl ArticleTagLink {
    pub fn new(article_id: i32, tag_id: i32) -> Self {
        Self { article_id, tag_id }
    }
}
