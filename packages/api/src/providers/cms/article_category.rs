use crate::enums::ArticleType;
use crate::http::AuthSession;
use crate::interfaces::{
    CategoryResponse, CreateCategoryRequest, ReorderCategoriesRequest, UpdateCategoryRequest,
};
use crate::models::ArticleCategory;
#[cfg(feature = "server")]
use crate::services::cms::article_category::{
    batch_reorder_categories as batch_reorder_categories_service,
    create_category as create_category_service, delete_category as delete_category_service,
    list_categories as list_categories_service, update_category as update_category_service,
};
use dioxus::prelude::*;

#[post("/api/cms/categories", auth: AuthSession)]
pub async fn create_category(
    request: CreateCategoryRequest,
) -> Result<CategoryResponse, ServerFnError> {
    let _session = auth.require_auth()?;

    let category = create_category_service(
        request.name,
        request.slug,
        request.description,
        request.article_type,
        request.sort_order,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(build_category_response(category))
}

#[get("/api/cms/categories/list", auth: AuthSession)]
pub async fn list_categories(
    article_type: ArticleType,
) -> Result<Vec<CategoryResponse>, ServerFnError> {
    let _session = auth.require_auth()?;

    let categories = list_categories_service(article_type)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(categories
        .into_iter()
        .map(build_category_response)
        .collect())
}

#[post("/api/cms/categories/update", auth: AuthSession)]
pub async fn update_category(
    category_id: i32,
    request: UpdateCategoryRequest,
) -> Result<CategoryResponse, ServerFnError> {
    let _session = auth.require_auth()?;

    let category = update_category_service(
        category_id,
        request.name,
        request.slug,
        request.description,
        request.sort_order,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(build_category_response(category))
}

#[post("/api/cms/categories/delete", auth: AuthSession)]
pub async fn delete_category(category_id: i32) -> Result<(), ServerFnError> {
    let _session = auth.require_auth()?;

    delete_category_service(category_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[post("/api/cms/categories/reorder", auth: AuthSession)]
pub async fn reorder_categories(request: ReorderCategoriesRequest) -> Result<(), ServerFnError> {
    let _session = auth.require_auth()?;

    batch_reorder_categories_service(request.order)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
fn build_category_response(category: ArticleCategory) -> CategoryResponse {
    let article_type = category.get_article_type();

    CategoryResponse {
        id: category.id,
        name: category.name,
        slug: category.slug,
        description: category.description,
        article_type,
        sort_order: category.sort_order,
        created_at: category.created_at,
        updated_at: category.updated_at,
    }
}
