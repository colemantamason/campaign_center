use crate::error::{minio_error, AppError};
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::Builder as S3ConfigBuilder, presigning::PresigningConfig, primitives::ByteStream,
    Client,
};
use std::{env, sync::OnceLock, time::Duration};

const MINIO_MEDIA_URL_EXPIRY_SECONDS: u64 = 3600;
const MINIO_MEDIA_BUCKET: &str = "media";

static MINIO_CLIENT: OnceLock<Client> = OnceLock::new();
static MINIO_ENDPOINT_URL: OnceLock<String> = OnceLock::new();
static MINIO_PUBLIC_URL_CACHED: OnceLock<String> = OnceLock::new();

pub fn is_minio_initialized() -> bool {
    MINIO_CLIENT.get().is_some()
}

pub fn initialize_minio_client() -> Result<(), AppError> {
    let endpoint = env::var("MINIO_ENDPOINT")
        .map_err(|_| AppError::ConfigError("MINIO_ENDPOINT not set".to_string()))?;

    let public_url = env::var("MINIO_PUBLIC_URL")
        .map_err(|_| AppError::ConfigError("MINIO_PUBLIC_URL not set".to_string()))?;

    let access_key = env::var("MINIO_ACCESS_KEY")
        .map_err(|_| AppError::ConfigError("MINIO_ACCESS_KEY not set".to_string()))?;

    let secret_key = env::var("MINIO_SECRET_KEY")
        .map_err(|_| AppError::ConfigError("MINIO_SECRET_KEY not set".to_string()))?;

    let credentials = Credentials::new(&access_key, &secret_key, None, None, "minio-static");

    let config = S3ConfigBuilder::new()
        .endpoint_url(&endpoint)
        .region(Region::new("us-east-1"))
        .credentials_provider(credentials)
        .force_path_style(true)
        .behavior_version_latest()
        .build();

    let client = Client::from_conf(config);

    MINIO_CLIENT
        .set(client)
        .map_err(|_| AppError::ConfigError("MinIO client already initialized".to_string()))?;

    MINIO_ENDPOINT_URL
        .set(endpoint)
        .map_err(|_| AppError::ConfigError("MinIO endpoint URL already set".to_string()))?;

    MINIO_PUBLIC_URL_CACHED
        .set(public_url)
        .map_err(|_| AppError::ConfigError("MinIO public URL already set".to_string()))?;

    tracing::info!("MinIO client initialized");

    Ok(())
}

pub fn get_minio_client() -> Result<&'static Client, AppError> {
    MINIO_CLIENT
        .get()
        .ok_or_else(|| AppError::ConfigError("MinIO client not initialized".to_string()))
}

fn get_minio_endpoint() -> Result<&'static str, AppError> {
    MINIO_ENDPOINT_URL
        .get()
        .map(|string| string.as_str())
        .ok_or_else(|| AppError::ConfigError("MinIO endpoint URL not initialized".to_string()))
}

fn get_minio_public_url() -> Result<&'static str, AppError> {
    MINIO_PUBLIC_URL_CACHED
        .get()
        .map(|string| string.as_str())
        .ok_or_else(|| AppError::ConfigError("MinIO public URL not initialized".to_string()))
}

pub async fn minio_upload_object(
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, AppError> {
    let client = get_minio_client()?;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .content_type(content_type)
        .send()
        .await
        .map_err(minio_error)?;

    Ok(key.to_string())
}

pub async fn minio_delete_object(bucket: &str, key: &str) -> Result<(), AppError> {
    let client = get_minio_client()?;

    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(minio_error)?;

    Ok(())
}

pub async fn get_minio_presigned_url(
    bucket: &str,
    key: &str,
    expires_in: Duration,
) -> Result<String, AppError> {
    let client = get_minio_client()?;

    let presigning_config = PresigningConfig::builder()
        .expires_in(expires_in)
        .build()
        .map_err(|error| AppError::InternalError(format!("Presigning config error: {}", error)))?;

    let presigned = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning_config)
        .await
        .map_err(minio_error)?;

    // replace the internal minio endpoint with the public-facing URL
    let url = presigned.uri().to_string();
    let endpoint = get_minio_endpoint()?;
    let public_url = get_minio_public_url()?;

    Ok(url.replace(endpoint, public_url))
}

pub async fn object_exists(bucket: &str, key: &str) -> Result<bool, AppError> {
    let client = get_minio_client()?;

    match client.head_object().bucket(bucket).key(key).send().await {
        Ok(_) => Ok(true),
        Err(error) => {
            let service_error = error.into_service_error();
            if service_error.is_not_found() {
                Ok(false)
            } else {
                Err(AppError::ExternalServiceError {
                    service: "MinIO".to_string(),
                    message: service_error.to_string(),
                })
            }
        }
    }
}

pub async fn minio_upload_media(
    key: &str,
    data: Vec<u8>,
    content_type: &str,
) -> Result<String, AppError> {
    minio_upload_object(MINIO_MEDIA_BUCKET, key, data, content_type).await
}

pub async fn minio_delete_media(key: &str) -> Result<(), AppError> {
    minio_delete_object(MINIO_MEDIA_BUCKET, key).await
}

pub async fn get_minio_media_url(key: &str) -> Result<String, AppError> {
    get_minio_presigned_url(
        MINIO_MEDIA_BUCKET,
        key,
        Duration::from_secs(MINIO_MEDIA_URL_EXPIRY_SECONDS),
    )
    .await
}
