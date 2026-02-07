use crate::enums::{ArticleStatus, ArticleType};
use crate::error::{postgres_error, AppError};
use crate::models::{Article, ArticleUpdate, NewArticle, NewArticleRevision};
use crate::postgres::get_postgres_connection;
use crate::redis::invalidate_redis_cached_article;
use crate::schema::{
    article_categories, article_revisions, article_tags, articles, articles_tags, users,
};
use crate::services::cms::article_tag::sync_article_tags;
use crate::services::{
    validate_optional_slug, validate_optional_string, validate_required_string,
    MAX_ARTICLE_SLUG_LENGTH, MAX_ARTICLE_TITLE_LENGTH,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use slug::slugify;
use std::collections::HashMap;

pub async fn create_article(
    author_id: i32,
    article_type: ArticleType,
    title: String,
    slug: Option<String>,
    excerpt: Option<String>,
    content: Option<serde_json::Value>,
    cover_image_url: Option<String>,
    category_id: Option<i32>,
    tag_ids: Option<Vec<i32>>,
) -> Result<Article, AppError> {
    validate_required_string("title", &title, MAX_ARTICLE_TITLE_LENGTH)?;
    validate_optional_slug("slug", &slug, MAX_ARTICLE_SLUG_LENGTH)?;

    let slug = slug.unwrap_or_else(|| slugify(&title));

    let connection = &mut get_postgres_connection().await?;

    let existing: Option<Article> = articles::table
        .filter(articles::slug.eq(&slug))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

    if existing.is_some() {
        return Err(AppError::already_exists("Article with this slug"));
    }

    if let Some(cat_id) = category_id {
        let category_type: String = article_categories::table
            .find(cat_id)
            .select(article_categories::article_type)
            .first(connection)
            .await
            .optional()
            .map_err(postgres_error)?
            .ok_or_else(|| AppError::not_found("Category"))?;

        if category_type != article_type.as_str() {
            return Err(AppError::validation(
                "category_id",
                "Category does not match article type",
            ));
        }
    }

    let mut new_article = NewArticle::new(author_id, article_type, title, slug);

    if let Some(cat_id) = category_id {
        new_article = new_article.set_category(cat_id);
    }
    if let Some(excerpt_text) = excerpt {
        new_article = new_article.set_excerpt(excerpt_text);
    }
    if let Some(content_json) = content {
        new_article = new_article.set_content(content_json);
    }
    if let Some(url) = cover_image_url {
        new_article = new_article.set_cover_image_url(url);
    }

    let article: Article = diesel::insert_into(articles::table)
        .values(&new_article)
        .get_result(connection)
        .await
        .map_err(postgres_error)?;

    // link tags if provided
    if let Some(ids) = tag_ids {
        sync_article_tags(article.id, &ids).await?;
    }

    Ok(article)
}

pub async fn update_article(
    article_id: i32,
    title: Option<String>,
    slug: Option<String>,
    excerpt: Option<String>,
    content: Option<serde_json::Value>,
    cover_image_url: Option<String>,
    category_id: Option<i32>,
    tag_ids: Option<Vec<i32>>,
    scheduled_publish_at: Option<DateTime<Utc>>,
) -> Result<Article, AppError> {
    validate_optional_string("title", &title, MAX_ARTICLE_TITLE_LENGTH)?;
    validate_optional_slug("slug", &slug, MAX_ARTICLE_SLUG_LENGTH)?;

    let connection = &mut get_postgres_connection().await?;

    let existing: Article = articles::table
        .find(article_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article"))?;

    // if slug is changing, check uniqueness
    if let Some(ref new_slug) = slug {
        let duplicate: Option<Article> = articles::table
            .filter(articles::slug.eq(new_slug))
            .filter(articles::id.ne(article_id))
            .first(connection)
            .await
            .optional()
            .map_err(postgres_error)?;

        if duplicate.is_some() {
            return Err(AppError::already_exists("Article with this slug"));
        }
    }

    let update = ArticleUpdate {
        title,
        slug: slug.clone(),
        excerpt: excerpt.map(Some),
        content,
        cover_image_url: cover_image_url.map(Some),
        category_id: category_id.map(Some),
        scheduled_publish_at: scheduled_publish_at.map(Some),
        updated_at: Some(Utc::now()),
        ..Default::default()
    };

    let article: Article = diesel::update(articles::table.find(article_id))
        .set(&update)
        .get_result(connection)
        .await
        .map_err(postgres_error)?;

    // sync tags if provided
    if let Some(ids) = tag_ids {
        sync_article_tags(article_id, &ids).await?;
    }

    // invalidate cache if slug changed and article was published
    if slug.is_some() && existing.is_published() {
        invalidate_redis_cached_article(&existing.slug).await.ok();
        invalidate_redis_cached_article(&article.slug).await.ok();
    }

    Ok(article)
}

pub async fn publish_article(article_id: i32, published_by: i32) -> Result<Article, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let updated_article = connection
        .transaction::<_, AppError, _>(|connection| {
            Box::pin(async move {
                let article: Article = articles::table
                    .find(article_id)
                    .first(connection)
                    .await
                    .optional()
                    .map_err(postgres_error)?
                    .ok_or_else(|| AppError::not_found("Article"))?;

                let max_revision: Option<i32> = article_revisions::table
                    .filter(article_revisions::article_id.eq(article_id))
                    .select(diesel::dsl::max(article_revisions::revision_number))
                    .first(connection)
                    .await
                    .map_err(postgres_error)?;

                let next_revision = max_revision.unwrap_or(0) + 1;

                let new_revision = NewArticleRevision::new(
                    article_id,
                    article.title.clone(),
                    article.content.clone(),
                    next_revision,
                    published_by,
                );
                let new_revision = if let Some(ref excerpt) = article.excerpt {
                    new_revision.set_excerpt(excerpt.clone())
                } else {
                    new_revision
                };

                diesel::insert_into(article_revisions::table)
                    .values(&new_revision)
                    .execute(connection)
                    .await
                    .map_err(postgres_error)?;

                let now = Utc::now();

                let update = ArticleUpdate {
                    status: Some(ArticleStatus::Published.as_str().to_string()),
                    published_at: if article.published_at.is_none() {
                        Some(Some(now))
                    } else {
                        None
                    },
                    updated_at: Some(now),
                    ..Default::default()
                };

                let updated: Article = diesel::update(articles::table.find(article_id))
                    .set(&update)
                    .get_result(connection)
                    .await
                    .map_err(postgres_error)?;

                Ok(updated)
            })
        })
        .await?;

    invalidate_redis_cached_article(&updated_article.slug)
        .await
        .ok();

    Ok(updated_article)
}

pub async fn get_article(article_id: i32) -> Result<Article, AppError> {
    let connection = &mut get_postgres_connection().await?;

    articles::table
        .find(article_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article"))
}

pub async fn get_article_by_slug(slug: &str) -> Result<Article, AppError> {
    let connection = &mut get_postgres_connection().await?;

    articles::table
        .filter(articles::slug.eq(slug))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article"))
}

pub async fn list_articles(
    article_type: Option<ArticleType>,
    status: Option<ArticleStatus>,
    category_id: Option<i32>,
    page: i64,
    per_page: i64,
) -> Result<(Vec<Article>, i64), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let mut query = articles::table.into_boxed();
    let mut count_query = articles::table.into_boxed();

    if let Some(article_type) = article_type {
        let type_str = article_type.as_str().to_string();
        query = query.filter(articles::article_type.eq(type_str.clone()));
        count_query = count_query.filter(articles::article_type.eq(type_str));
    }

    if let Some(status) = status {
        let status_str = status.as_str().to_string();
        query = query.filter(articles::status.eq(status_str.clone()));
        count_query = count_query.filter(articles::status.eq(status_str));
    }

    if let Some(cat_id) = category_id {
        query = query.filter(articles::category_id.eq(cat_id));
        count_query = count_query.filter(articles::category_id.eq(cat_id));
    }

    let total: i64 = count_query
        .count()
        .get_result(connection)
        .await
        .map_err(postgres_error)?;

    let offset = (page - 1) * per_page;

    let articles_list: Vec<Article> = query
        .order(articles::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok((articles_list, total))
}

pub async fn delete_article(article_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let article: Article = articles::table
        .find(article_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Article"))?;

    connection
        .transaction::<_, AppError, _>(|connection| {
            Box::pin(async move {
                diesel::delete(
                    articles_tags::table.filter(articles_tags::article_id.eq(article_id)),
                )
                .execute(connection)
                .await
                .map_err(postgres_error)?;

                diesel::delete(
                    article_revisions::table.filter(article_revisions::article_id.eq(article_id)),
                )
                .execute(connection)
                .await
                .map_err(postgres_error)?;

                diesel::delete(articles::table.find(article_id))
                    .execute(connection)
                    .await
                    .map_err(postgres_error)?;

                Ok(())
            })
        })
        .await?;

    if article.is_published() {
        invalidate_redis_cached_article(&article.slug).await.ok();
    }

    Ok(())
}

pub async fn auto_save_article(
    article_id: i32,
    content: serde_json::Value,
) -> Result<Article, AppError> {
    let connection = &mut get_postgres_connection().await?;

    diesel::update(articles::table.find(article_id))
        .set((
            articles::content.eq(&content),
            articles::updated_at.eq(Utc::now()),
        ))
        .get_result::<Article>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn get_article_author_info(
    author_id: i32,
) -> Result<crate::interfaces::ArticleAuthorInfo, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let (first_name, last_name, avatar_url): (String, String, Option<String>) = users::table
        .find(author_id)
        .select((users::first_name, users::last_name, users::avatar_url))
        .first(connection)
        .await
        .map_err(postgres_error)?;

    Ok(crate::interfaces::ArticleAuthorInfo {
        id: author_id,
        first_name,
        last_name,
        avatar_url,
    })
}

pub async fn get_article_category_info(
    category_id: i32,
) -> Result<crate::interfaces::ArticleCategoryInfo, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let (name, slug): (String, String) = article_categories::table
        .find(category_id)
        .select((article_categories::name, article_categories::slug))
        .first(connection)
        .await
        .map_err(postgres_error)?;

    Ok(crate::interfaces::ArticleCategoryInfo {
        id: category_id,
        name,
        slug,
    })
}

pub async fn get_article_tag_infos(
    article_id: i32,
) -> Result<Vec<crate::interfaces::ArticleTagInfo>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let tag_ids: Vec<i32> = articles_tags::table
        .filter(articles_tags::article_id.eq(article_id))
        .select(articles_tags::tag_id)
        .load(connection)
        .await
        .map_err(postgres_error)?;

    if tag_ids.is_empty() {
        return Ok(vec![]);
    }

    let tags: Vec<(i32, String, String)> = article_tags::table
        .filter(article_tags::id.eq_any(&tag_ids))
        .select((article_tags::id, article_tags::name, article_tags::slug))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok(tags
        .into_iter()
        .map(|(id, name, slug)| crate::interfaces::ArticleTagInfo { id, name, slug })
        .collect())
}

pub async fn batch_get_author_infos(
    author_ids: &[i32],
) -> Result<HashMap<i32, crate::interfaces::ArticleAuthorInfo>, AppError> {
    if author_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let connection = &mut get_postgres_connection().await?;

    let rows: Vec<(i32, String, String, Option<String>)> = users::table
        .filter(users::id.eq_any(author_ids))
        .select((
            users::id,
            users::first_name,
            users::last_name,
            users::avatar_url,
        ))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, first_name, last_name, avatar_url)| {
            (
                id,
                crate::interfaces::ArticleAuthorInfo {
                    id,
                    first_name,
                    last_name,
                    avatar_url,
                },
            )
        })
        .collect())
}

pub async fn batch_get_category_infos(
    category_ids: &[i32],
) -> Result<HashMap<i32, crate::interfaces::ArticleCategoryInfo>, AppError> {
    if category_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let connection = &mut get_postgres_connection().await?;

    let rows: Vec<(i32, String, String)> = article_categories::table
        .filter(article_categories::id.eq_any(category_ids))
        .select((
            article_categories::id,
            article_categories::name,
            article_categories::slug,
        ))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, name, slug)| {
            (
                id,
                crate::interfaces::ArticleCategoryInfo { id, name, slug },
            )
        })
        .collect())
}

pub async fn batch_get_tag_infos(
    article_ids: &[i32],
) -> Result<HashMap<i32, Vec<crate::interfaces::ArticleTagInfo>>, AppError> {
    if article_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let connection = &mut get_postgres_connection().await?;

    let links: Vec<(i32, i32)> = articles_tags::table
        .filter(articles_tags::article_id.eq_any(article_ids))
        .select((articles_tags::article_id, articles_tags::tag_id))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    if links.is_empty() {
        return Ok(HashMap::new());
    }

    let unique_tag_ids: Vec<i32> = links
        .iter()
        .map(|(_, tag_id)| *tag_id)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let tags: Vec<(i32, String, String)> = article_tags::table
        .filter(article_tags::id.eq_any(&unique_tag_ids))
        .select((article_tags::id, article_tags::name, article_tags::slug))
        .load(connection)
        .await
        .map_err(postgres_error)?;

    let tag_map: HashMap<i32, crate::interfaces::ArticleTagInfo> = tags
        .into_iter()
        .map(|(id, name, slug)| (id, crate::interfaces::ArticleTagInfo { id, name, slug }))
        .collect();

    let mut result: HashMap<i32, Vec<crate::interfaces::ArticleTagInfo>> = HashMap::new();

    for (article_id, tag_id) in links {
        if let Some(tag_info) = tag_map.get(&tag_id) {
            result.entry(article_id).or_default().push(tag_info.clone());
        }
    }

    Ok(result)
}
