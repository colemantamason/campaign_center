use crate::error::AppError;
use crate::minio::{minio_delete_media, minio_upload_media};
use crate::models::{MediaAsset, NewMediaAsset};
use crate::postgres::get_postgres_connection;
use crate::schema::media_assets;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

pub async fn upload_media(
    uploaded_by: i32,
    original_filename: String,
    mime_type: String,
    file_size_bytes: i64,
    data: Vec<u8>,
    alt_text: Option<String>,
) -> Result<MediaAsset, AppError> {
    if original_filename.trim().is_empty() {
        return Err(AppError::validation(
            "original_filename",
            "Filename is required",
        ));
    }

    // generate a unique storage key preserving file extension
    let extension = original_filename.rsplit('.').next().unwrap_or("bin");
    let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
    let storage_key = format!("uploads/{}", unique_filename);

    // upload to minio
    minio_upload_media(&storage_key, data, &mime_type).await?;

    // create database record
    let connection = &mut get_postgres_connection().await?;

    let mut new_asset = NewMediaAsset::new(
        uploaded_by,
        unique_filename,
        original_filename,
        mime_type,
        file_size_bytes,
        storage_key,
    );

    if let Some(alt) = alt_text {
        new_asset = new_asset.set_alt_text(alt);
    }

    diesel::insert_into(media_assets::table)
        .values(&new_asset)
        .get_result::<MediaAsset>(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })
}

pub async fn list_media(page: i64, per_page: i64) -> Result<(Vec<MediaAsset>, i64), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let total: i64 = media_assets::table
        .count()
        .get_result(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    let offset = (page - 1) * per_page;

    let assets: Vec<MediaAsset> = media_assets::table
        .order(media_assets::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok((assets, total))
}

pub async fn delete_media(asset_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let asset: MediaAsset = media_assets::table
        .find(asset_id)
        .first(connection)
        .await
        .optional()
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?
        .ok_or_else(|| AppError::not_found("Media asset"))?;

    // delete from minio
    minio_delete_media(&asset.storage_key).await?;

    // delete from database
    diesel::delete(media_assets::table.find(asset_id))
        .execute(connection)
        .await
        .map_err(|error| AppError::ExternalServiceError {
            service: "Postgres".to_string(),
            message: error.to_string(),
        })?;

    Ok(())
}
