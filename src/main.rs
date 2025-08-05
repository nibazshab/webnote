mod utils;

use askama::Template;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, DefaultBodyLimit, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use rust_embed::RustEmbed;
use serde::Deserialize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Error, FromRow, Row, SqlitePool};
use std::net::SocketAddr;
use std::str::FromStr;
use std::{env, path};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing::{error, info};

#[derive(FromRow, Template)]
#[template(path = "index.html")]
struct Note {
    id: String,
    content: String,
}

#[derive(RustEmbed)]
#[folder = "templates/assets/"]
struct Assets;

#[tokio::main]
async fn main() {
    println!("Version {}", env!("CARGO_PKG_VERSION"));

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
        .route("/", get(root_get).post(root_post))
        .route("/assets/{file}", get(assets))
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(DefaultBodyLimit::max(5 << 20)));

    println!("Server running on {}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn init_database() -> Result<SqlitePool, Error> {
    let dir = env::var("DATA_DIR").ok().unwrap_or_else(|| {
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.display().to_string()
    });

    let db_url = path::Path::new(format!("sqlite:{}", dir).as_str())
        .join("note.db")
        .display()
        .to_string();

    let options = SqliteConnectOptions::from_str(&*db_url)?
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .create_if_missing(true);

    println!("Connecting to {}", db_url);
    let pool = SqlitePool::connect_with(options).await?;

    create_table(&pool).await?;
    Ok(pool)
}

async fn path_get(
    Path(id): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    if id.len() > 64 {
        return Redirect::temporary(&*utils::rand_string(4)).into_response();
    }

    let mut note = Note {
        id: id.clone(),
        content: "".to_string(),
    };

    if let Err(e) = note.select(&pool).await {
        error!("{} - {} - {}", id, e, addr);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or_default();

    const CLI: [&str; 2] = ["curl", "wget"];
    let is_cli = CLI.iter().any(|agent| ua.contains(agent));

    info!("[get] {} - {} - {}", id, addr, ua);
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

    match note.upsert(&pool).await {
        Ok(_) => {
            info!("[post] {} - {} - {}", id, addr, ua);
            StatusCode::OK.into_response()
        }
        Err(e) => {
            error!("{} - {} - {}", id, e, addr);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn root_get() -> Redirect {
    Redirect::temporary(&*utils::rand_string(4))
}

async fn root_post() -> StatusCode {
    StatusCode::BAD_REQUEST
}

impl Note {
    async fn upsert(&self, pool: &SqlitePool) -> Result<(), Error> {
        let key = utils::hash(&self.id);

        sqlx::query(
            "
INSERT INTO notes (key, id, content) VALUES (?1, ?2, ?3)
ON CONFLICT(key) DO UPDATE SET
    content = excluded.content
",
        )
        .bind(key)
        .bind(&self.id)
        .bind(&self.content)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn select(&mut self, pool: &SqlitePool) -> Result<(), Error> {
        let key = utils::hash(&self.id);

        if let Some(rs) = sqlx::query("SELECT content FROM notes WHERE key = ?")
            .bind(key)
            .fetch_optional(pool)
            .await?
        {
            self.content = rs.get("content")
        }

        Ok(())
    }
}

async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        "
CREATE TABLE IF NOT EXISTS notes (
    key INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    content TEXT NOT NULL
)",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn assets(Path(file): Path<String>) -> impl IntoResponse {
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
