use crate::enums::ArticleType;
use crate::schema::article_categories;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = article_categories)]
#[diesel(check_for_backend(Postgres))]
pub struct ArticleCategory {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub article_type: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ArticleCategory {
    pub fn get_article_type(&self) -> ArticleType {
        ArticleType::from_str(&self.article_type).unwrap_or(ArticleType::Blog)
    }
}

#[derive(Insertable)]
#[diesel(table_name = article_categories)]
pub struct NewArticleCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub article_type: String,
    pub sort_order: i32,
}

impl NewArticleCategory {
    pub fn new(name: String, slug: String, article_type: ArticleType) -> Self {
        Self {
            name,
            slug,
            description: None,
            article_type: article_type.as_str().to_string(),
            sort_order: 0,
        }
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn set_sort_order(mut self, sort_order: i32) -> Self {
        self.sort_order = sort_order;
        self
    }
}

#[derive(AsChangeset, Default)]
#[diesel(table_name = article_categories)]
pub struct ArticleCategoryUpdate {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
    pub sort_order: Option<i32>,
}
