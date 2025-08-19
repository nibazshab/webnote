use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use tokio::signal;

use crate::var::Assets;

pub async fn favicon() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "image/x-icon")], vec![]).into_response()
}

pub async fn assets(Path(file): Path<String>) -> impl IntoResponse {
    match Assets::get(&file) {
        Some(obj) => {
            let mime = mime_guess::from_path(&file).first_or_octet_stream();
            (
                [
                    (header::CONTENT_TYPE, mime.as_ref()),
                    (
                        header::CACHE_CONTROL,
                        format!("public, max-age={}", 60 * 60 * 24 * 7).as_str(),
                    ),
                ],
                obj.data,
            )
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to handle Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to handle signal")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
