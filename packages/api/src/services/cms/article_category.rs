use crate::enums::ArticleType;
use crate::error::AppError;
use crate::models::{ArticleCategory, ArticleCategoryUpdate, NewArticleCategory};
use crate::postgres::get_postgres_connection;
use crate::schema::article_categories;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use slug::slugify;

pub async fn create_category(
    name: String,
    slug: Option<String>,
    description: Option<String>,
    article_type: ArticleType,
    sort_order: Option<i32>,
) -> Result<ArticleCategory, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::validation("name", "Name is required"));
    }

    let slug = slug.unwrap_or_else(|| slugify(&name));

    let connection = &mut get_postgres_connection().await?;

    let existing: Option<ArticleCategory> = article_categories::table
        .filter(article_categories::slug.eq(&slug))
        .filter(article_categories::article_type.eq(article_type.as_str()))
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn list_categories(article_type: ArticleType) -> Result<Vec<ArticleCategory>, AppError> {
    let connection = &mut get_postgres_connection().await?;

    article_categories::table
        .filter(article_categories::article_type.eq(article_type.as_str()))
        .order(article_categories::sort_order.asc())
        .load::<ArticleCategory>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn update_category(
    category_id: i32,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    sort_order: Option<i32>,
) -> Result<ArticleCategory, AppError> {
    let connection = &mut get_postgres_connection().await?;

    let existing: ArticleCategory = article_categories::table
        .find(category_id)
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::not_found("Category"))?;

    if let Some(ref new_slug) = slug {
        let duplicate: Option<ArticleCategory> = article_categories::table
            .filter(article_categories::slug.eq(new_slug))
            .filter(article_categories::id.ne(category_id))
            .filter(article_categories::article_type.eq(&existing.article_type))
            .first(connection)
            .await
            .optional()
            .map_err(|error| AppError::ExternalServiceError {
                service: "Postgres".to_string(),
                message: error.to_string(),
            })?;

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
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn delete_category(category_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let deleted = diesel::delete(article_categories::table.find(category_id))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    if deleted == 0 {
        return Err(AppError::not_found("Category"));
    }

    Ok(())
}

// uses raw SQL because diesel's DSL doesn't support UPDATE ... FROM (VALUES ...) for per-row different values
pub async fn batch_reorder_categories(order: Vec<(i32, i32)>) -> Result<(), AppError> {
    if order.is_empty() {
        return Ok(());
    }

    let connection = &mut get_postgres_connection().await?;

    let values: Vec<String> = order
        .iter()
        .map(|(id, sort_order)| format!("({}, {})", id, sort_order))
        .collect();

    let query = format!(
        "UPDATE article_categories AS ac SET sort_order = v.new_order FROM (VALUES {}) AS v(id, new_order) WHERE ac.id = v.id",
        values.join(", ")
    );

    diesel::sql_query(query)
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}
