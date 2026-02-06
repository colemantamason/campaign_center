use crate::interfaces::{
    ListPublicArticlesRequest, PublicArticleListResponse, PublicArticleResponse,
};
#[cfg(feature = "server")]
use crate::services::shared::{
    get_published_article_by_slug as get_published_article_by_slug_service,
    list_published_articles as list_published_articles_service,
};
use dioxus::prelude::*;

#[get("/api/articles/get")]
pub async fn get_published_article(slug: String) -> Result<PublicArticleResponse, ServerFnError> {
    get_published_article_by_slug_service(&slug)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))
}

#[post("/api/articles/list")]
pub async fn list_published_articles(
    request: ListPublicArticlesRequest,
) -> Result<PublicArticleListResponse, ServerFnError> {
    let page = request.page.unwrap_or(1).max(1);
    let per_page = request.per_page.unwrap_or(20).clamp(1, 100);

    list_published_articles_service(
        request.article_type,
        request.category_slug,
        request.tag_slug,
        page,
        per_page,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))
}
