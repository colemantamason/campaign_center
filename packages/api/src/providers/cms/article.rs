use crate::http::AuthSession;
use crate::interfaces::{
    ArticleListResponse, ArticleResponse, ArticleRevisionResponse, CreateArticleRequest,
    ListArticlesRequest, UpdateArticleRequest,
};
#[cfg(feature = "server")]
use crate::services::cms::{
    article::{
        auto_save_article, batch_get_author_infos, batch_get_category_infos, batch_get_tag_infos,
        create_article as create_article_service, delete_article as delete_article_service,
        get_article as get_article_service, get_article_author_info, get_article_category_info,
        get_article_tag_infos, list_articles as list_articles_service,
        publish_article as publish_article_service, update_article as update_article_service,
    },
    article_revision::list_revisions,
};
use dioxus::prelude::*;

#[post("/api/cms/articles", auth: AuthSession)]
pub async fn create_article(
    request: CreateArticleRequest,
) -> Result<ArticleResponse, ServerFnError> {
    let session = auth.require_staff()?;

    let article = create_article_service(
        session.user_id,
        request.article_type,
        request.title,
        request.slug,
        request.excerpt,
        request.content,
        request.cover_image_url,
        request.category_id,
        request.tag_ids,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    build_article_response(article).await
}

#[post("/api/cms/articles/update", auth: AuthSession)]
pub async fn update_article(
    article_id: i32,
    request: UpdateArticleRequest,
) -> Result<ArticleResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let article = update_article_service(
        article_id,
        request.title,
        request.slug,
        request.excerpt,
        request.content,
        request.cover_image_url,
        request.category_id,
        request.tag_ids,
        request.scheduled_publish_at,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    build_article_response(article).await
}

#[post("/api/cms/articles/publish", auth: AuthSession)]
pub async fn publish_article(article_id: i32) -> Result<ArticleResponse, ServerFnError> {
    let session = auth.require_staff()?;

    let article = publish_article_service(article_id, session.user_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    build_article_response(article).await
}

#[get("/api/cms/articles/get", auth: AuthSession)]
pub async fn get_article(article_id: i32) -> Result<ArticleResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let article = get_article_service(article_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    build_article_response(article).await
}

#[post("/api/cms/articles/list", auth: AuthSession)]
pub async fn list_articles(
    request: ListArticlesRequest,
) -> Result<ArticleListResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let page = request.page.unwrap_or(1).max(1);
    let per_page = request.per_page.unwrap_or(20).clamp(1, 100);

    let (articles, total) = list_articles_service(
        request.article_type,
        request.status,
        request.category_id,
        page,
        per_page,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    let responses = batch_build_article_responses(articles)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(ArticleListResponse {
        articles: responses,
        total,
        page,
        per_page,
    })
}

#[post("/api/cms/articles/delete", auth: AuthSession)]
pub async fn delete_article(article_id: i32) -> Result<(), ServerFnError> {
    let _session = auth.require_staff()?;

    delete_article_service(article_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[post("/api/cms/articles/auto-save", auth: AuthSession)]
pub async fn auto_save(article_id: i32, content: serde_json::Value) -> Result<(), ServerFnError> {
    let _session = auth.require_staff()?;

    auto_save_article(article_id, content)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[get("/api/cms/articles/revisions", auth: AuthSession)]
pub async fn list_article_revisions(
    article_id: i32,
) -> Result<Vec<ArticleRevisionResponse>, ServerFnError> {
    let _session = auth.require_staff()?;

    let revisions = list_revisions(article_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    if revisions.is_empty() {
        return Ok(vec![]);
    }

    let publisher_ids: Vec<i32> = revisions
        .iter()
        .map(|revision| revision.published_by)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let authors = batch_get_author_infos(&publisher_ids)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let mut responses = Vec::with_capacity(revisions.len());

    for revision in revisions {
        let published_by = authors
            .get(&revision.published_by)
            .cloned()
            .ok_or_else(|| ServerFnError::new("Revision author not found"))?;

        responses.push(ArticleRevisionResponse {
            id: revision.id,
            revision_number: revision.revision_number,
            title: revision.title,
            excerpt: revision.excerpt,
            content: revision.content,
            published_by,
            created_at: revision.created_at,
        });
    }

    Ok(responses)
}

#[post("/api/cms/articles/revisions/restore", auth: AuthSession)]
pub async fn restore_revision(revision_id: i32) -> Result<ArticleResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let article = crate::services::cms::article_revision::restore_revision(revision_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    build_article_response(article).await
}

#[cfg(feature = "server")]
async fn build_article_response(
    article: crate::models::Article,
) -> Result<ArticleResponse, ServerFnError> {
    let author = get_article_author_info(article.author_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let category = if let Some(cat_id) = article.category_id {
        Some(
            get_article_category_info(cat_id)
                .await
                .map_err(|error| ServerFnError::new(error.to_string()))?,
        )
    } else {
        None
    };

    let tags = get_article_tag_infos(article.id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let article_type = article.get_article_type();
    let status = article.get_status();

    Ok(ArticleResponse {
        id: article.id,
        article_type,
        title: article.title,
        slug: article.slug,
        excerpt: article.excerpt,
        content: article.content,
        cover_image_url: article.cover_image_url,
        status,
        author,
        category,
        tags,
        published_at: article.published_at,
        scheduled_publish_at: article.scheduled_publish_at,
        created_at: article.created_at,
        updated_at: article.updated_at,
    })
}

#[cfg(feature = "server")]
async fn batch_build_article_responses(
    articles: Vec<crate::models::Article>,
) -> Result<Vec<ArticleResponse>, crate::error::AppError> {
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
            .ok_or_else(|| crate::error::AppError::not_found("Author"))?;

        let category = article
            .category_id
            .and_then(|cat_id| categories.get(&cat_id).cloned());

        let article_tags = tags.get(&article.id).cloned().unwrap_or_default();

        let article_type = article.get_article_type();
        let status = article.get_status();

        responses.push(ArticleResponse {
            id: article.id,
            article_type,
            title: article.title,
            slug: article.slug,
            excerpt: article.excerpt,
            content: article.content,
            cover_image_url: article.cover_image_url,
            status,
            author,
            category,
            tags: article_tags,
            published_at: article.published_at,
            scheduled_publish_at: article.scheduled_publish_at,
            created_at: article.created_at,
            updated_at: article.updated_at,
        });
    }

    Ok(responses)
}
