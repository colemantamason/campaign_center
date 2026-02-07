use crate::error::{postgres_error, AppError};
use crate::minio::{minio_delete_media, minio_upload_media};
use crate::models::{MediaAsset, NewMediaAsset};
use crate::postgres::get_postgres_connection;
use crate::schema::media_assets;
use crate::services::{validate_media_file, validate_required_string, MAX_FILENAME_LENGTH};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

const MINIO_DELETE_MAX_RETRIES: u32 = 3;

pub async fn upload_media(
    uploaded_by: i32,
    original_filename: String,
    mime_type: String,
    file_size_bytes: i64,
    data: Vec<u8>,
    alt_text: Option<String>,
) -> Result<MediaAsset, AppError> {
    validate_required_string("original_filename", &original_filename, MAX_FILENAME_LENGTH)?;
    validate_media_file(&original_filename, &mime_type, file_size_bytes)?;

    // generate a unique storage key preserving file extension
    let extension = original_filename.rsplit('.').next().unwrap_or("bin");
    let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
    let storage_key = format!("uploads/{}", unique_filename);

    minio_upload_media(&storage_key, data, &mime_type).await?;

    let connection = &mut get_postgres_connection().await?;

    let mut new_asset = NewMediaAsset::new(
        uploaded_by,
        unique_filename,
        original_filename,
        mime_type,
        file_size_bytes,
        storage_key.clone(),
    );

    if let Some(alt) = alt_text {
        new_asset = new_asset.set_alt_text(alt);
    }

    let result = diesel::insert_into(media_assets::table)
        .values(&new_asset)
        .get_result::<MediaAsset>(connection)
        .await
        .map_err(postgres_error);

    if result.is_err() {
        if let Err(delete_error) = minio_delete_media(&storage_key).await {
            tracing::error!(
                "Failed to clean up orphaned MinIO object '{}' after DB insert failure: {}",
                storage_key,
                delete_error
            );
        }
    }

    result
}

pub async fn list_media(page: i64, per_page: i64) -> Result<(Vec<MediaAsset>, i64), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let total: i64 = media_assets::table
        .count()
        .get_result(connection)
        .await
        .map_err(postgres_error)?;

    let offset = (page - 1) * per_page;

    let assets: Vec<MediaAsset> = media_assets::table
        .order(media_assets::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(connection)
        .await
        .map_err(postgres_error)?;

    Ok((assets, total))
}

pub async fn delete_media(asset_id: i32) -> Result<(), AppError> {
    let connection = &mut get_postgres_connection().await?;

    let asset: MediaAsset = media_assets::table
        .find(asset_id)
        .first(connection)
        .await
        .optional()
        .map_err(postgres_error)?
        .ok_or_else(|| AppError::not_found("Media asset"))?;
    diesel::delete(media_assets::table.find(asset_id))
        .execute(connection)
        .await
        .map_err(postgres_error)?;

    let storage_key = asset.storage_key;
    let mut last_error = None;

    for attempt in 0..MINIO_DELETE_MAX_RETRIES {
        match minio_delete_media(&storage_key).await {
            Ok(()) => return Ok(()),
            Err(error) => {
                last_error = Some(error);
                if attempt + 1 < MINIO_DELETE_MAX_RETRIES {
                    tracing::warn!(
                        "MinIO delete attempt {} failed for '{}', retrying...",
                        attempt + 1,
                        storage_key
                    );
                }
            }
        }
    }

    if let Some(error) = last_error {
        tracing::error!(
            "Failed to delete MinIO object '{}' after {} retries (DB record already deleted, orphaned file): {}",
            storage_key,
            MINIO_DELETE_MAX_RETRIES,
            error
        );
    }

    Ok(())
}
