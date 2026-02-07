use crate::error::{postgres_error, AppError};
use crate::models::{Article, ArticleRevision, ArticleUpdate};
use crate::postgres::get_postgres_connection;
use crate::redis::invalidate_redis_cached_article;
use crate::schema::{article_revisions, articles};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn list_revisions(article_id: i32) -> Result<Vec<ArticleRevision>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    article_revisions::table
        .filter(article_revisions::article_id.eq(article_id))
        .order(article_revisions::revision_number.desc())
        .load::<ArticleRevision>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn get_revision(revision_id: i32) -> Result<ArticleRevision, AppError> {
    let connection = &mut get_postgres_connection().await?;

    article_revisions::table
        .find(revision_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article revision"))
}

pub async fn restore_revision(revision_id: i32) -> Result<Article, AppError> {
    let revision = get_revision(revision_id).await?;

    let connection = &mut get_postgres_connection().await?;

    let existing: Article = articles::table
        .find(revision.article_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article"))?;

    let was_published = existing.is_published();
    let slug = existing.slug.clone();

    let update = ArticleUpdate {
        title: Some(revision.title),
        excerpt: Some(revision.excerpt),
        content: Some(revision.content),
        status: Some(crate::enums::ArticleStatus::Draft.as_str().to_string()),
        updated_at: Some(Utc::now()),
        ..Default::default()
    };

    let article = diesel::update(articles::table.find(revision.article_id))
        .set(&update)
        .get_result::<Article>(connection)
        .await
        .map_err(postgres_error)?;

    if was_published {
        invalidate_redis_cached_article(&slug).await.ok();
    }

    Ok(article)
}
