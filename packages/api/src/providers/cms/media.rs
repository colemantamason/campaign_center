use crate::http::AuthSession;
use crate::interfaces::{
    ListMediaRequest, MediaAssetResponse, MediaListResponse, UploadMediaRequest,
};
#[cfg(feature = "server")]
use crate::minio::get_minio_media_url;
#[cfg(feature = "server")]
use crate::services::cms::media::{
    delete_media as delete_media_service, list_media as list_media_service,
    upload_media as upload_media_service,
};
use dioxus::prelude::*;

#[post("/api/cms/media/upload", auth: AuthSession)]
pub async fn upload_media(
    request: UploadMediaRequest,
) -> Result<MediaAssetResponse, ServerFnError> {
    let session = auth.require_staff()?;

    // TODO: actual file bytes will come via multipart upload or a two-step presigned URL flow;
    // for now the service creates the DB record and placeholder upload
    let asset = upload_media_service(
        session.user_id,
        request.original_filename,
        request.mime_type,
        request.file_size_bytes,
        vec![], // placeholder — real upload to be wired with multipart
        request.alt_text,
    )
    .await
    .map_err(|error| ServerFnError::new(error.to_string()))?;

    let url = get_minio_media_url(&asset.storage_key)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(MediaAssetResponse {
        id: asset.id,
        filename: asset.filename,
        original_filename: asset.original_filename,
        mime_type: asset.mime_type,
        file_size_bytes: asset.file_size_bytes,
        url,
        alt_text: asset.alt_text,
        created_at: asset.created_at,
    })
}

#[post("/api/cms/media/list", auth: AuthSession)]
pub async fn list_media(
    request: ListMediaRequest,
) -> Result<MediaListResponse, ServerFnError> {
    let _session = auth.require_staff()?;

    let page = request.page.unwrap_or(1).max(1);
    let per_page = request.per_page.unwrap_or(20).clamp(1, 100);

    let (assets, total) = list_media_service(page, per_page)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let mut responses = Vec::with_capacity(assets.len());
    for asset in assets {
        let url = get_minio_media_url(&asset.storage_key)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        responses.push(MediaAssetResponse {
            id: asset.id,
            filename: asset.filename,
            original_filename: asset.original_filename,
            mime_type: asset.mime_type,
            file_size_bytes: asset.file_size_bytes,
            url,
            alt_text: asset.alt_text,
            created_at: asset.created_at,
        });
    }

    Ok(MediaListResponse {
        assets: responses,
        total,
        page,
        per_page,
    })
}

#[post("/api/cms/media/delete", auth: AuthSession)]
pub async fn delete_media(asset_id: i32) -> Result<(), ServerFnError> {
    let _session = auth.require_staff()?;

    delete_media_service(asset_id)
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
