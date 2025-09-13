mod cfg;
mod db;
mod ext;
mod features;
mod utils;
mod var;

use askama::Template;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum_extra::{TypedHeader, headers};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

use crate::cfg::port;
use crate::db::{init_schemas, pool};
use crate::ext::{ext_router, shutdown_signal};
use crate::var::Note;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    if let Err(e) = application().await {
        error!("{e}");
        std::process::exit(1);
    }
}

async fn application() -> Result<(), Box<dyn std::error::Error>> {
    println!("v{}", env!("CARGO_PKG_VERSION"));

    features::inits()?;

    let pool = pool().await?;
    init_schemas(&pool).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port()));
    println!("Server running on {addr}");

    let middleware = ServiceBuilder::new()
        .layer(DefaultBodyLimit::max(5 << 20))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(addr).await?;
    let router = router()
        .merge(ext_router())
        .merge(features::routers())
        .with_state(pool.clone())
        .layer(middleware)
        .into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    pool.close().await;

    Ok(())
}

fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/{id}", get(path_get).post(path_post))
        .route("/-/{id}", get(path_raw_get))
        .route("/", get(root_get).post(root_post))
}

async fn path_get(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> impl IntoResponse {
    let note = match select(id, &pool).await {
        Ok(note) => note,
        Err(e) => return e,
    };

    const CLI: [&str; 2] = ["curl", "wget"];
    let is_cli = CLI.iter().any(|agent| user_agent.as_str().contains(agent));

    if is_cli {
        (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            note.content,
        )
            .into_response()
    } else {
        let html = match note.render() {
            Ok(html) => html,
            Err(e) => {
                error!("{e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
        Html(html).into_response()
    }
}

async fn path_raw_get(Path(id): Path<String>, State(pool): State<SqlitePool>) -> impl IntoResponse {
    match select(id, &pool).await {
        Ok(note) => (
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            note.content,
        )
            .into_response(),
        Err(e) => e,
    }
}

async fn select(id: String, pool: &SqlitePool) -> Result<Note, Response> {
    if id.len() > 64 {
        return Err(Redirect::temporary(&utils::rand_string(4)).into_response());
    }

    match Note::read(id, pool).await {
        Ok(note) => Ok(note),
        Err(e) => {
            error!("{e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

async fn root_get() -> Redirect {
    Redirect::temporary(&utils::rand_string(4))
}

async fn path_post(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    bytes: Bytes,
) -> impl IntoResponse {
    if id.len() > 64 {
        return (
            StatusCode::BAD_REQUEST,
            "Invalid, expecting id length < 64.",
        )
            .into_response();
    }

    if let Err(e) = create_or_update(id.clone(), &pool, &bytes).await {
        return e;
    }

    info!("{id}");
    StatusCode::OK.into_response()
}

async fn root_post(
    State(pool): State<SqlitePool>,
    TypedHeader(host): TypedHeader<headers::Host>,
    bytes: Bytes,
) -> impl IntoResponse {
    let id = utils::rand_string(5);

    if let Err(e) = create_or_update(id.clone(), &pool, &bytes).await {
        return e;
    }

    info!("{id}");
    (StatusCode::OK, format!("{host}/-/{id}")).into_response()
}

async fn create_or_update(id: String, pool: &SqlitePool, bytes: &Bytes) -> Result<(), Response> {
    #[derive(Deserialize)]
    struct Payload {
        t: String,
    }

    let t = 'a: {
        if let Ok(form) = serde_urlencoded::from_bytes::<Payload>(bytes) {
            break 'a form;
        }

        if let Ok(t) = std::str::from_utf8(bytes) {
            break 'a Payload { t: t.to_string() };
        }

        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid, expecting text, encoding=utf-8.",
        )
            .into_response());
    };

    let note = Note {
        id: id.clone(),
        content: t.t,
    };

    match note.write(pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}
