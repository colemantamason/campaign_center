use crate::enums::ArticleType;
use crate::error::{postgres_error, AppError};
use crate::models::{ArticleCategory, ArticleCategoryUpdate, NewArticleCategory};
use crate::postgres::get_postgres_connection;
use crate::schema::article_categories;
use crate::services::{
    validate_optional_slug, validate_optional_string, validate_required_string,
    MAX_CATEGORY_NAME_LENGTH, MAX_CATEGORY_SLUG_LENGTH,
};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use slug::slugify;

pub async fn create_category(
    name: String,
    slug: Option<String>,
    description: Option<String>,
    article_type: ArticleType,
    sort_order: Option<i32>,
) -> Result<ArticleCategory, AppError> {
    validate_required_string("name", &name, MAX_CATEGORY_NAME_LENGTH)?;
    validate_optional_slug("slug", &slug, MAX_CATEGORY_SLUG_LENGTH)?;

    let slug = slug.unwrap_or_else(|| slugify(&name));

    let connection = &mut get_postgres_connection().await?;

    let existing: Option<ArticleCategory> = article_categories::table
        .filter(article_categories::slug.eq(&slug))
        .filter(article_categories::article_type.eq(article_type.as_str()))
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?;

    if existing.is_some() {
        return Err(AppError::already_exists("Category with this slug"));
    }

    let mut new_category = NewArticleCategory::new(name, slug, article_type);

    if let Some(desc) = description {
        new_category = new_category.set_description(desc);
    }
    if let Some(order) = sort_order {
        new_category = new_category.set_sort_order(order);
    }

    diesel::insert_into(article_categories::table)
        .values(&new_category)
        .get_result::<ArticleCategory>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn list_categories(article_type: ArticleType) -> Result<Vec<ArticleCategory>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    article_categories::table
        .filter(article_categories::article_type.eq(article_type.as_str()))
        .order(article_categories::sort_order.asc())
        .load::<ArticleCategory>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn update_category(
    category_id: i32,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    sort_order: Option<i32>,
) -> Result<ArticleCategory, AppError> {
    validate_optional_string("name", &name, MAX_CATEGORY_NAME_LENGTH)?;
    validate_optional_slug("slug", &slug, MAX_CATEGORY_SLUG_LENGTH)?;

    let connection = &mut get_postgres_connection().await?;

    let existing: ArticleCategory = article_categories::table
        .find(category_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Category"))?;

    if let Some(ref new_slug) = slug {
        let duplicate: Option<ArticleCategory> = article_categories::table
            .filter(article_categories::slug.eq(new_slug))
            .filter(article_categories::id.ne(category_id))
            .filter(article_categories::article_type.eq(&existing.article_type))
            .first(connection)
            .await
            .optional()
            .map_err(postgres_error)?;

        if duplicate.is_some() {
            return Err(AppError::already_exists("Category with this slug"));
        }
    }

    let update = ArticleCategoryUpdate {
        name,
        slug,
        description: description.map(Some),
        sort_order,
    };

    diesel::update(article_categories::table.find(category_id))
        .set(&update)
        .get_result::<ArticleCategory>(connection)
        .await
        .map_err(postgres_error)
}

pub async fn delete_category(category_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let deleted = diesel::delete(article_categories::table.find(category_id))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    if deleted == 0 {
        return Err(AppError::not_found("Category"));
    }

    Ok(())
}

pub async fn batch_reorder_categories(order: Vec<(i32, i32)>) -> Result<(), AppError> {
    if order.is_empty() {
        return Ok(());
    }

    let connection = &mut get_postgres_connection().await?;

    let category_ids: Vec<i32> = order.iter().map(|(id, _)| *id).collect();

    let distinct_types: Vec<String> = article_categories::table
        .filter(article_categories::id.eq_any(&category_ids))
        .select(article_categories::article_type)
        .distinct()
        .load(connection)
        .await
        .map_err(postgres_error)?;

    if distinct_types.is_empty() {
        return Err(AppError::not_found("Categories"));
    }

    if distinct_types.len() > 1 {
        return Err(AppError::validation(
            "order",
            "All categories must belong to the same article type",
        ));
    }

    connection
        .transaction::<_, AppError, _>(|connection| {
            Box::pin(async move {
                for (id, sort_order) in &order {
                    diesel::update(article_categories::table.find(*id))
                        .set(article_categories::sort_order.eq(*sort_order))
                        .execute(connection)
                        .await
                        .map_err(postgres_error)?;
                }
                Ok(())
            })
        })
        .await?;

    Ok(())
}
