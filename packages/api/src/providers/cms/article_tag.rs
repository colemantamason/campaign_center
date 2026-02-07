use crate::http::AuthSession;
use crate::interfaces::{CreateTagRequest, SearchTagsRequest, TagResponse};
#[cfg(feature = "server")]
use crate::services::cms::article_tag::{
    create_tag as create_tag_service, delete_tag as delete_tag_service,
    search_tags as search_tags_service,
};
use dioxus::prelude::*;

#[post("/api/cms/tags", auth: AuthSession)]
pub async fn create_tag(request: CreateTagRequest) -> Result<TagResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let tag = create_tag_service(request.name, request.slug)
        .await?;

    Ok(build_tag_response(tag))
}

#[post("/api/cms/tags/search", auth: AuthSession)]
pub async fn search_tags(
    request: SearchTagsRequest,
) -> Result<Vec<TagResponse>, ServerFnError> {
    let _session = auth.require_staff()?;

    let limit = request.limit.unwrap_or(20).clamp(1, 100);

    let tags = search_tags_service(request.query, limit)
        .await?;

    Ok(tags.into_iter().map(build_tag_response).collect())
}

#[post("/api/cms/tags/delete", auth: AuthSession)]
pub async fn delete_tag(tag_id: i32) -> Result<(), ServerFnError> {
    let _session = auth.require_staff()?;

    delete_tag_service(tag_id)
        .await?;

    Ok(())
}

#[cfg(feature = "server")]
fn build_tag_response(tag: crate::models::ArticleTag) -> TagResponse {
    TagResponse {
        id: tag.id,
        name: tag.name,
        slug: tag.slug,
        created_at: tag.created_at,
    }
}
