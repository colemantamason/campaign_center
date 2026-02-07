use crate::http::AuthSession;
use crate::interfaces::{
    ArticleListResponse, ArticleResponse, ArticleRevisionResponse, CreateArticleRequest,
    ListArticlesRequest, PaginationParams, UpdateArticleRequest,
};
#[cfg(feature = "server")]
use crate::models::Article;
#[cfg(feature = "server")]
use crate::services::cms::{
    article::{
        auto_save_article, batch_build_article_responses, batch_get_author_infos,
        build_article_response, create_article as create_article_service,
        delete_article as delete_article_service, get_article as get_article_service,
        list_articles as list_articles_service, publish_article as publish_article_service,
        update_article as update_article_service,
    },
    article_revision::list_revisions,
};
use dioxus::prelude::*;

#[cfg(feature = "server")]
async fn require_article_ownership(
    article_id: i32,
    user_id: i32,
) -> Result<Article, ServerFnError> {
    let article = get_article_service(article_id).await?;
    if article.author_id != user_id {
        return Err(ServerFnError::new("You can only modify your own articles"));
    }
    Ok(article)
}

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
    .await?;

    Ok(build_article_response(article).await?)
}

#[post("/api/cms/articles/update", auth: AuthSession)]
pub async fn update_article(
    article_id: i32,
    request: UpdateArticleRequest,
) -> Result<ArticleResponse, ServerFnError> {
    let session = auth.require_staff()?;
    require_article_ownership(article_id, session.user_id).await?;

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
    .await?;

    Ok(build_article_response(article).await?)
}

#[post("/api/cms/articles/publish", auth: AuthSession)]
pub async fn publish_article(article_id: i32) -> Result<ArticleResponse, ServerFnError> {
    let session = auth.require_staff()?;
    require_article_ownership(article_id, session.user_id).await?;

    let article = publish_article_service(article_id, session.user_id).await?;

    Ok(build_article_response(article).await?)
}

#[get("/api/cms/articles/get", auth: AuthSession)]
pub async fn get_article(article_id: i32) -> Result<ArticleResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let article = get_article_service(article_id).await?;

    Ok(build_article_response(article).await?)
}

#[post("/api/cms/articles/list", auth: AuthSession)]
pub async fn list_articles(
    request: ListArticlesRequest,
) -> Result<ArticleListResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let (page, per_page) = PaginationParams::resolve(request.page, request.per_page);

    let (articles, total) = list_articles_service(
        request.article_type,
        request.status,
        request.category_id,
        page,
        per_page,
    )
    .await?;

    let responses = batch_build_article_responses(articles).await?;

    Ok(ArticleListResponse {
        articles: responses,
        total,
        page,
        per_page,
    })
}

#[post("/api/cms/articles/delete", auth: AuthSession)]
pub async fn delete_article(article_id: i32) -> Result<(), ServerFnError> {
    let session = auth.require_staff()?;
    require_article_ownership(article_id, session.user_id).await?;

    delete_article_service(article_id).await?;

    Ok(())
}

#[post("/api/cms/articles/auto-save", auth: AuthSession)]
pub async fn auto_save(article_id: i32, content: serde_json::Value) -> Result<(), ServerFnError> {
    let session = auth.require_staff()?;
    require_article_ownership(article_id, session.user_id).await?;

    auto_save_article(article_id, content).await?;

    Ok(())
}

#[get("/api/cms/articles/revisions", auth: AuthSession)]
pub async fn list_article_revisions(
    article_id: i32,
) -> Result<Vec<ArticleRevisionResponse>, ServerFnError> {
    let _session = auth.require_staff()?;

    let revisions = list_revisions(article_id).await?;

    if revisions.is_empty() {
        return Ok(vec![]);
    }

    let publisher_ids: Vec<i32> = revisions
        .iter()
        .map(|revision| revision.published_by)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let authors = batch_get_author_infos(&publisher_ids).await?;

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
    let session = auth.require_staff()?;

    let revision = crate::services::cms::article_revision::get_revision(revision_id).await?;
    require_article_ownership(revision.article_id, session.user_id).await?;

    let article = crate::services::cms::article_revision::restore_revision(revision_id).await?;

    Ok(build_article_response(article).await?)
}
