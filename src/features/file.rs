use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::ext::assets_response;

#[derive(rust_embed::RustEmbed)]
#[folder = "templates/features/file/"]
struct FileAssets;

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
