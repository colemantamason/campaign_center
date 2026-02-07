use crate::interfaces::{
    ListPublicArticlesRequest, PaginationParams, PublicArticleListResponse, PublicArticleResponse,
};
#[cfg(feature = "server")]
use crate::services::shared::{
    get_published_article_by_slug as get_published_article_by_slug_service,
    list_published_articles as list_published_articles_service,
};
use dioxus::prelude::*;

#[get("/api/articles/get")]
pub async fn get_published_article(slug: String) -> Result<PublicArticleResponse, ServerFnError> {
    Ok(get_published_article_by_slug_service(&slug)
        .await?)
}

#[post("/api/articles/list")]
pub async fn list_published_articles(
    request: ListPublicArticlesRequest,
) -> Result<PublicArticleListResponse, ServerFnError> {
    let (page, per_page) = PaginationParams::resolve(request.page, request.per_page);

    Ok(list_published_articles_service(
        request.article_type,
        request.category_slug,
        request.tag_slug,
        page,
        per_page,
    )
    .await?)
}
