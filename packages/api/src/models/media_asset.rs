use crate::schema::media_assets;
use chrono::{DateTime, Utc};
use diesel::{pg::Pg as Postgres, prelude::*};

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = media_assets)]
#[diesel(check_for_backend(Postgres))]
pub struct MediaAsset {
    pub id: i32,
    pub uploaded_by: i32,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub storage_key: String,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = media_assets)]
pub struct NewMediaAsset {
    pub uploaded_by: i32,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size_bytes: i64,
    pub storage_key: String,
    pub alt_text: Option<String>,
}

impl NewMediaAsset {
    pub fn new(
        uploaded_by: i32,
        filename: String,
        original_filename: String,
        mime_type: String,
        file_size_bytes: i64,
        storage_key: String,
    ) -> Self {
        Self {
            uploaded_by,
            filename,
            original_filename,
            mime_type,
            file_size_bytes,
            storage_key,
            alt_text: None,
        }
    }

    pub fn set_alt_text(mut self, alt_text: String) -> Self {
        self.alt_text = Some(alt_text);
        self
    }
}
