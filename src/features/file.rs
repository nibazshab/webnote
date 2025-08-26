use axum::Json;
use axum::body::Body;
use axum::extract::{ConnectInfo, Multipart, Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use sqlx::{Decode, Error, FromRow, Row, Sqlite, SqlitePool, Type};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, path};
use tokio::time::{self, sleep};
use tokio_util;
use tracing::{error, info};

use crate::ext::assets_response;
use crate::utils;
use crate::var::data_dir;

include!(concat!(env!("OUT_DIR"), "/rust_embed_features_file.rs"));

#[cfg(debug_assertions)]
pub type FileAssets = DebugFileAssets;

#[cfg(not(debug_assertions))]
pub type FileAssets = ReleaseFileAssets;

#[derive(FromRow)]
struct File {
    key: i64,
    id: String,
    time: String,
}

static ATTACHMENT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| path::Path::new(&data_dir()).join("attachment"));

pub async fn file_remove(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
) -> impl IntoResponse {
    let token = match headers.get("token").and_then(|token| token.to_str().ok()) {
        Some(token) => token,
        None => return (StatusCode::FORBIDDEN, "Invalid, expecting token header.").into_response(),
    };

    let key = utils::hash(&id);
    let time = match File::read::<String>("time", key, &pool).await {
        Ok(time) => time,
        Err(_) => {
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    if token != time {
        return StatusCode::FORBIDDEN.into_response();
    };

    let obj = ATTACHMENT_PATH.join(key.to_string());

    if let Err(e) = tokio::fs::remove_file(obj).await {
        error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Err(e) = File::remove(key, &pool).await {
        error!("{e}");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    StatusCode::OK.into_response()
}

pub async fn file_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<SqlitePool>,
    headers: header::HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let req_obj = match multipart.next_field().await {
        Ok(Some(obj)) => obj,
        _ => return (StatusCode::BAD_REQUEST, "Invalid, expecting a file.").into_response(),
    };

    let file_name = req_obj.file_name().unwrap_or("unknown").to_string();

    let bytes = match req_obj.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let file = match File::write(file_name, &pool).await {
        Ok(file) => file,
        Err(e) => {
            error!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let fs_obj = ATTACHMENT_PATH.join(file.key.to_string());
    if let Err(e) = tokio::fs::write(&fs_obj, &bytes).await {
        error!("{e}");
        if let Err(e) = File::remove(file.key, &pool).await {
            error!("{e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|ua| ua.to_str().ok())
        .unwrap_or_default();

    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    let id = file.id;

    info!("[post] {id} - {addr} - {ua}");
    Json(json!({
        "link": format!("{host}/b/{id}"),
        "token": file.time,
    }))
    .into_response()
}

pub async fn file_download(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match id.as_str() {
        "script.js" => response_assets(id),
        "style.css" => response_assets(id),
        _ => response_files(id, &pool).await,
    }
}

async fn response_files(id: String, pool: &SqlitePool) -> Response {
    let key = utils::hash(&id);

    let name = match File::read::<String>("name", key, pool).await {
        Ok(name) => name,
        Err(_) => {
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    let obj = ATTACHMENT_PATH.join(key.to_string());
    if !obj.is_file() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let f = tokio::fs::File::open(obj.to_str().unwrap()).await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(f);
    let body = Body::from_stream(stream);

    let headers = [(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{name}\""),
    )];

    (headers, body).into_response()
}

fn response_assets(file: String) -> Response {
    match FileAssets::get(&file) {
        Some(obj) => assets_response(&file, Vec::from(obj.data)),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn file_index() -> impl IntoResponse {
    let file = "index.html".to_string();
    response_assets(file)
}

fn sys_millis_time() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

pub fn init_attachment() -> std::io::Result<()> {
    let attachment = ATTACHMENT_PATH.clone();

    if !attachment.exists() {
        fs::create_dir_all(attachment)?;
    }

    Ok(())
}

pub const CREATE_TABLE_FILE: &str = "
CREATE TABLE IF NOT EXISTS files (
    key INTEGER PRIMARY KEY,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    time TEXT NOT NULL
);";

impl File {
    async fn write(name: String, pool: &SqlitePool) -> Result<Self, Error> {
        let mut i = 0;

        loop {
            let id = utils::rand_string(8);
            let file = File {
                key: utils::hash(&id),
                id,
                time: sys_millis_time(),
            };

            match sqlx::query("INSERT INTO files (key, id, name, time) VALUES (?1, ?2, ?3, ?4)")
                .bind(file.key)
                .bind(&file.id)
                .bind(&name)
                .bind(&file.time)
                .execute(pool)
                .await
            {
                Ok(_) => {
                    return Ok(file);
                }
                Err(e) => {
                    if i >= 10 {
                        return Err(e);
                    }
                    i += 1;
                    sleep(time::Duration::from_millis(100)).await;
                }
            }
        }
    }

    async fn read<T>(column: &str, key: i64, pool: &SqlitePool) -> Result<T, Error>
    where
        T: for<'r> Decode<'r, Sqlite> + Type<Sqlite> + Send + Unpin,
    {
        let query_str = format!("SELECT {} FROM files WHERE key = ?", column);

        let rs = sqlx::query(&query_str).bind(key).fetch_one(pool).await?;

        Ok(rs.get(0))
    }

    async fn remove(key: i64, pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query("DELETE FROM files WHERE key = ?")
            .bind(key)
            .execute(pool)
            .await?;

        Ok(())
    }
}
