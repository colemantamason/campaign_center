use crate::enums::{ArticleStatus, ArticleType};
use crate::error::AppError;
use crate::interfaces::{
    ArticleAuthorInfo, ArticleCategoryInfo, ArticleTagInfo, PublicArticleListResponse,
    PublicArticleResponse,
};
use crate::models::Article;
use crate::postgres::get_postgres_connection;
use crate::redis::{get_redis_cached_article_by_slug, redis_cache_article_by_slug};
use crate::schema::{article_categories, article_tags, articles, articles_tags, users};
use crate::services::cms::article::{
    batch_get_author_infos, batch_get_category_infos, batch_get_tag_infos,
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn get_published_article_by_slug(slug: &str) -> Result<PublicArticleResponse, AppError> {
    if let Ok(Some(cached_json)) = get_redis_cached_article_by_slug(slug).await {
        if let Ok(response) = serde_json::from_str::<PublicArticleResponse>(&cached_json) {
            return Ok(response);
        }
    }

    let connection = &mut get_postgres_connection().await?;

    let article: Article = articles::table
        .filter(articles::slug.eq(slug))
        .filter(articles::status.eq(ArticleStatus::Published.as_str()))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::not_found("Article"))?;

    let response = build_public_article_response(&article).await?;

    if let Ok(json) = serde_json::to_string(&response) {
        redis_cache_article_by_slug(slug, &json).await.ok();
    }

    Ok(response)
}

pub async fn list_published_articles(
    article_type: ArticleType,
    category_slug: Option<String>,
    tag_slug: Option<String>,
    page: i64,
    per_page: i64,
) -> Result<PublicArticleListResponse, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let published_status = ArticleStatus::Published.as_str().to_string();
    let type_str = article_type.as_str().to_string();

    let category_id = if let Some(ref cat_slug) = category_slug {
        let cat: Option<(i32,)> = article_categories::table
            .filter(article_categories::slug.eq(cat_slug))
            .filter(article_categories::article_type.eq(&type_str))
            .select((article_categories::id,))
            .first(connection)
            .await
            .optional()
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;
        cat.map(|(id,)| id)
    } else {
        None
    };

    let tag_article_ids = if let Some(ref t_slug) = tag_slug {
        let tag: Option<(i32,)> = article_tags::table
            .filter(article_tags::slug.eq(t_slug))
            .select((article_tags::id,))
            .first(connection)
            .await
            .optional()
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;

        if let Some((tag_id,)) = tag {
            let ids: Vec<i32> = articles_tags::table
                .filter(articles_tags::tag_id.eq(tag_id))
                .select(articles_tags::article_id)
                .load(connection)
                .await
                .map_err(|error| AppError::ExternalServiceError {
                    service: "Postgres".to_string(),
                    message: error.to_string(),
                })?;
            Some(ids)
        } else {
            Some(vec![])
        }
    } else {
        None
    };

    let mut query = articles::table.into_boxed();
    let mut count_query = articles::table.into_boxed();

    query = query
        .filter(articles::status.eq(&published_status))
        .filter(articles::article_type.eq(&type_str));
    count_query = count_query
        .filter(articles::status.eq(&published_status))
        .filter(articles::article_type.eq(&type_str));

    if let Some(cat_id) = category_id {
        query = query.filter(articles::category_id.eq(cat_id));
        count_query = count_query.filter(articles::category_id.eq(cat_id));
    }

    if let Some(ref ids) = tag_article_ids {
        query = query.filter(articles::id.eq_any(ids));
        count_query = count_query.filter(articles::id.eq_any(ids));
    }

    let total: i64 = count_query
        .count()
        .get_result(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let offset = (page - 1) * per_page;

    let articles_list: Vec<Article> = query
        .order(articles::published_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let responses = batch_build_public_article_responses(&articles_list).await?;

    Ok(PublicArticleListResponse {
        articles: responses,
        total,
        page,
        per_page,
    })
}

async fn build_public_article_response(
    article: &Article,
) -> Result<PublicArticleResponse, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let (first_name, last_name, avatar_url): (String, String, Option<String>) = users::table
        .find(article.author_id)
        .select((users::first_name, users::last_name, users::avatar_url))
        .first(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let author = ArticleAuthorInfo {
        id: article.author_id,
        first_name,
        last_name,
        avatar_url,
    };

    let category = if let Some(cat_id) = article.category_id {
        let (name, slug): (String, String) = article_categories::table
            .find(cat_id)
            .select((article_categories::name, article_categories::slug))
            .first(connection)
            .await
            .optional()
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?
            .unwrap_or_else(|| ("Unknown".to_string(), "unknown".to_string()));

        Some(ArticleCategoryInfo {
            id: cat_id,
            name,
            slug,
        })
    } else {
        None
    };
    let tag_ids: Vec<i32> = articles_tags::table
        .filter(articles_tags::article_id.eq(article.id))
        .select(articles_tags::tag_id)
        .load(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let tags = if tag_ids.is_empty() {
        vec![]
    } else {
        let tag_rows: Vec<(i32, String, String)> = article_tags::table
            .filter(article_tags::id.eq_any(&tag_ids))
            .select((article_tags::id, article_tags::name, article_tags::slug))
            .load(connection)
            .await
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;

        tag_rows
            .into_iter()
            .map(|(id, name, slug)| ArticleTagInfo { id, name, slug })
            .collect()
    };

    Ok(PublicArticleResponse {
        id: article.id,
        article_type: article.get_article_type(),
        title: article.title.clone(),
        slug: article.slug.clone(),
        excerpt: article.excerpt.clone(),
        content: article.content.clone(),
        cover_image_url: article.cover_image_url.clone(),
        author,
        category,
        tags,
        published_at: article.published_at.unwrap_or_else(|| {
            tracing::warn!(
                "published article id={} has NULL published_at, using current time",
                article.id
            );
            Utc::now()
        }),
    })
}

async fn batch_build_public_article_responses(
    articles: &[Article],
) -> Result<Vec<PublicArticleResponse>, AppError> {
    if articles.is_empty() {
        return Ok(vec![]);
    }

    let author_ids: Vec<i32> = articles
        .iter()
        .map(|article| article.author_id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let category_ids: Vec<i32> = articles
        .iter()
        .filter_map(|article| article.category_id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let article_ids: Vec<i32> = articles.iter().map(|article| article.id).collect();

    let authors = batch_get_author_infos(&author_ids).await?;
    let categories = batch_get_category_infos(&category_ids).await?;
    let tags = batch_get_tag_infos(&article_ids).await?;

    let mut responses = Vec::with_capacity(articles.len());

    for article in articles {
        let author = authors
            .get(&article.author_id)
            .cloned()
            .ok_or_else(|| AppError::not_found("Author"))?;

        let category = article
            .category_id
            .and_then(|cat_id| categories.get(&cat_id).cloned());

        let article_tags = tags.get(&article.id).cloned().unwrap_or_default();

        responses.push(PublicArticleResponse {
            id: article.id,
            article_type: article.get_article_type(),
            title: article.title.clone(),
            slug: article.slug.clone(),
            excerpt: article.excerpt.clone(),
            content: article.content.clone(),
            cover_image_url: article.cover_image_url.clone(),
            author,
            category,
            tags: article_tags,
            published_at: article.published_at.unwrap_or_else(|| {
                tracing::warn!(
                    "published article id={} has NULL published_at, using current time",
                    article.id
                );
                Utc::now()
            }),
        });
    }

    Ok(responses)
}
