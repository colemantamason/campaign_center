use crate::error::AppError;
use crate::models::{ArticleTag, ArticleTagLink, NewArticleTag};
use crate::postgres::get_postgres_connection;
use crate::schema::{article_tags, articles_tags};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use slug::slugify;

pub async fn create_tag(name: String, slug: Option<String>) -> Result<ArticleTag, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::validation("name", "Name is required"));
    }

    let slug = slug.unwrap_or_else(|| slugify(&name));

    let connection = &mut get_postgres_connection().await?;

    // check slug uniqueness
    let existing: Option<ArticleTag> = article_tags::table
        .filter(article_tags::slug.eq(&slug))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    if existing.is_some() {
        return Err(AppError::already_exists("Tag with this slug"));
    }

    let new_tag = NewArticleTag::new(name, slug);

    diesel::insert_into(article_tags::table)
        .values(&new_tag)
        .get_result::<ArticleTag>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn search_tags(query: String, limit: i64) -> Result<Vec<ArticleTag>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let pattern = format!("%{}%", query.to_lowercase());

    article_tags::table
        .filter(article_tags::name.ilike(&pattern))
        .order(article_tags::name.asc())
        .limit(limit)
        .load::<ArticleTag>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn delete_tag(tag_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    // remove all article-tag links first
    diesel::delete(articles_tags::table.filter(articles_tags::tag_id.eq(tag_id)))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let deleted = diesel::delete(article_tags::table.find(tag_id))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    if deleted == 0 {
        return Err(AppError::not_found("Tag"));
    }

    Ok(())
}

// sync the articles_tags join table for a given article
pub async fn sync_article_tags(article_id: i32, tag_ids: &[i32]) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    // remove all existing links for this article
    diesel::delete(articles_tags::table.filter(articles_tags::article_id.eq(article_id)))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    if !tag_ids.is_empty() {
        let links: Vec<ArticleTagLink> = tag_ids
            .iter()
            .map(|tag_id| ArticleTagLink::new(article_id, *tag_id))
            .collect();

        diesel::insert_into(articles_tags::table)
            .values(&links)
            .execute(connection)
            .await
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;
    }

    Ok(())
}
