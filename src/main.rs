mod db;
mod ext;
mod utils;
mod var;

use askama::Template;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, DefaultBodyLimit, Multipart, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use base64::{Engine, engine::general_purpose};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::{error, info};

use crate::db::init_database;
use crate::ext::{assets, favicon, shutdown_signal};
use crate::var::Note;

#[tokio::main]
async fn main() {
    println!("v{}", env!("CARGO_PKG_VERSION"));
    tracing_subscriber::fmt().with_target(false).init();

    let pool = init_database().await.unwrap();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    let app = Router::new()
        .route("/{id}", get(path_get).post(path_post))
        .route("/-/{id}", get(path_raw_get))
        .route("/", get(root_get).post(root_post))
        .route("/assets/{file}", get(assets))
        .route("/favicon.ico", get(favicon))
        .with_state(pool.clone())
        .layer(ServiceBuilder::new().layer(DefaultBodyLimit::max(5 << 20)));

    println!("Server running on {addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    pool.close().await;
}

async fn path_get(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    if id.len() > 64 {
        return Redirect::temporary(&utils::rand_string(4)).into_response();
    }

    let mut note = Note {
        id: id.clone(),
        content: "".to_string(),
    };

    if let Err(e) = note.read(&pool).await {
        error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or_default();

    const CLI: [&str; 2] = ["curl", "wget"];
    let is_cli = CLI.iter().any(|agent| ua.contains(agent));

    if is_cli {
        (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            note.content,
        )
            .into_response()
    } else {
        let html = note.render().unwrap();
        Html(html).into_response()
    }
}

async fn path_post(
    Path(id): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
    bytes: Bytes,
) -> impl IntoResponse {
    if id.len() > 64 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid, expecting id length < 64.",
        )
            .into_response();
    }

    #[derive(Deserialize)]
    struct Payload {
        t: String,
    }

    let t = 'a: {
        if let Ok(form) = serde_urlencoded::from_bytes::<Payload>(&bytes) {
            break 'a form;
        }

        if let Ok(t) = String::from_utf8(bytes.to_vec()) {
            break 'a Payload { t };
        }

        return (
            StatusCode::BAD_REQUEST,
            "Invalid, expecting a form with 't' field.",
        )
            .into_response();
    };

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or_default();

    let note = Note {
        id: id.clone(),
        content: t.t,
    };

    match note.write(&pool).await {
        Ok(_) => {
            info!("[post] {id} - {addr} - {ua}");
            StatusCode::OK.into_response()
        }
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn root_get() -> Redirect {
    Redirect::temporary(&utils::rand_string(4))
}

async fn root_post(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let obj = if let Ok(Some(obj)) = multipart.next_field().await {
        obj
    } else {
        return (StatusCode::BAD_REQUEST, "Invalid, expecting a file.").into_response();
    };

    if !obj
        .content_type()
        .map(|s| s.to_string())
        .unwrap_or_default()
        .starts_with("text/")
    {
        return (StatusCode::BAD_REQUEST, "Invalid, expecting text file.").into_response();
    }

    let bytes = if let Ok(bytes) = obj.bytes().await {
        bytes
    } else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or_default();

    let id = utils::rand_string(5);

    let note = 'a: {
        if let Ok(t) = String::from_utf8(bytes.to_vec()) {
            break 'a Note {
                id: id.clone(),
                content: t,
            };
        }

        let b64 = general_purpose::STANDARD.encode(&bytes);
        break 'a Note {
            id: id.clone(),
            content: b64,
        };
    };

    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    match note.write(&pool).await {
        Ok(_) => {
            info!("[post] {id} - {addr} - {ua}");
            (StatusCode::OK, format!("{host}/-/{id}")).into_response()
        }
        Err(e) => {
            error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn path_raw_get(Path(id): Path<String>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    if id.len() > 64 {
        return Redirect::temporary(&utils::rand_string(4)).into_response();
    }

    let mut note = Note {
        id: id.clone(),
        content: "".to_string(),
    };

    if let Err(e) = note.read(&pool).await {
        error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        note.content,
    )
        .into_response()
}
