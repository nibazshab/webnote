use axum::Router;
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use mime_guess::MimeGuess;
use sqlx::SqlitePool;
use std::borrow::Cow;

use crate::var::Assets;

pub fn ext_router() -> Router<SqlitePool> {
    Router::new()
        .route("/assets/{file}", get(assets))
        .route("/favicon.ico", get(favicon))
}

async fn favicon() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/x-icon")], vec![]).into_response()
}

async fn assets(Path(id): Path<String>) -> impl IntoResponse {
    match Assets::get(&id) {
        Some(obj) => assets_response(&id, obj.data),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub fn assets_response(filename: &str, data: Cow<'static, [u8]>) -> Response {
    let content_type = MimeGuess::from_path(filename).first_or_octet_stream();
    let cache_control = format!("public, max-age={}", 60 * 60 * 24 * 7);

    let bytes = match data {
        Cow::Borrowed(slice) => Bytes::from_static(slice),
        Cow::Owned(vec) => Bytes::from(vec),
    };

    (
        [
            (header::CONTENT_TYPE, content_type.as_ref()),
            (header::CACHE_CONTROL, cache_control.as_str()),
        ],
        bytes,
    )
        .into_response()
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl+c");
    };
    let terminate = terminate();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(unix)]
async fn terminate() {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to listen for signal")
        .recv()
        .await;
}

#[cfg(not(unix))]
async fn terminate() -> ! {
    std::future::pending().await
}
