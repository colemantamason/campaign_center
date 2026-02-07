use crate::error::AppError;
use crate::models::{ArticleTag, ArticleTagLink, NewArticleTag};
use crate::postgres::get_postgres_connection;
use crate::schema::{article_tags, articles_tags};
use crate::services::{
    validate_optional_slug, validate_required_string, MAX_TAG_NAME_LENGTH, MAX_TAG_SLUG_LENGTH,
};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use slug::slugify;

pub async fn create_tag(name: String, slug: Option<String>) -> Result<ArticleTag, AppError> {
    validate_required_string("name", &name, MAX_TAG_NAME_LENGTH)?;
    validate_optional_slug("slug", &slug, MAX_TAG_SLUG_LENGTH)?;

    let slug = slug.unwrap_or_else(|| slugify(&name));

    let connection = &mut get_postgres_connection().await?;

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

    let escaped = query
        .to_lowercase()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");

    let pattern = format!("%{}%", escaped);

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

    connection
        .transaction::<_, AppError, _>(|connection| {
            Box::pin(async move {
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
            })
        })
        .await?;

    Ok(())
}

pub async fn sync_article_tags(article_id: i32, tag_ids: &[i32]) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    connection
        .transaction::<_, AppError, _>(|connection| {
            let tag_ids = tag_ids.to_vec();
            Box::pin(async move {
                diesel::delete(
                    articles_tags::table.filter(articles_tags::article_id.eq(article_id)),
                )
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
            })
        })
        .await?;

    Ok(())
}
