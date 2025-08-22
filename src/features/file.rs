use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::ext::assets_response;

include!(concat!(env!("OUT_DIR"), "/rust_embed_features_file.rs"));

#[cfg(debug_assertions)]
pub type FileAssets = DebugFileAssets;

#[cfg(not(debug_assertions))]
pub type FileAssets = ReleaseFileAssets;

pub async fn file_index() -> impl IntoResponse {
    let file = Path("index.html".to_string());
    file_assets(file).await
}

pub async fn file_assets(Path(file): Path<String>) -> impl IntoResponse {
    match FileAssets::get(&file) {
        Some(obj) => assets_response(&file, Vec::from(obj.data)),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub const CREATE_TABLE_FILE: &str = "
CREATE TABLE IF NOT EXISTS files (
    key INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    date DATE NOT NULL,
    token TEXT NOT NULL
);";
