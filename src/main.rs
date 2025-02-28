use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;
use clap::Parser;
use db::AppState;
use env_logger::Builder;
use log::{LevelFilter, info};
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::{path::Path, time::Duration};

mod db;
mod uid;

#[derive(Template)]
#[template(path = "index.html")]
struct HtmlResponse {
    uid: String,
    content: String,
}

#[derive(RustEmbed)]
#[folder = "templates/assets/"]
struct Assets;

#[derive(Deserialize)]
struct FormData {
    t: String,
}

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(short = 'P', long, default_value_t = 10003, value_name = "PORT")]
    port: u16,

    #[arg(short = 'D', long, value_parser = validate_directory, default_value = ".", value_name = "DIR")]
    db_dir: String,
}

#[get("/")]
async fn redirect_path() -> impl Responder {
    generate_path()
}

#[get("/{uid}")]
async fn get_note(
    req: HttpRequest,
    uid: web::Path<String>,
    data: web::Data<db::AppState>,
) -> impl Responder {
    let uid = uid.into_inner();

    if uid.len() > 16 {
        return generate_path();
    }

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    let is_cli_tool = user_agent.contains("curl") || user_agent.contains("wget");

    let content = match data.get_content(&uid) {
        Ok(content) => content,
        Err(rusqlite::Error::QueryReturnedNoRows) => String::new(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    if is_cli_tool {
        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(content)
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(HtmlResponse { uid, content }.render().unwrap())
    }
}

#[post("/")]
async fn invalid_path() -> impl Responder {
    HttpResponse::BadRequest().finish()
}

#[post("/{uid}")]
async fn save_note(
    req: HttpRequest,
    uid: web::Path<String>,
    form: web::Form<FormData>,
    data: web::Data<db::AppState>,
) -> impl Responder {
    let uid = uid.into_inner();

    if uid.len() > 16 {
        return HttpResponse::BadRequest().body("UID <= 16");
    }

    let client_ip = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split(',').next().map(|ip| ip.trim().to_string()))
        .unwrap_or_else(|| {
            req.peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        });

    info!("{} | {}", uid, client_ip);

    match data.save_content(&uid, &form.t) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/assets/{filename:.*}")]
async fn static_files(path: web::Path<String>) -> actix_web::HttpResponse {
    let filename = path.into_inner();

    match crate::Assets::get(&filename) {
        Some(file) => {
            let mime = mime_guess::from_path(&filename).first_or_octet_stream();

            let cache_duration = Duration::from_secs(86400);

            actix_web::HttpResponse::Ok()
                .append_header(("content-type", mime.as_ref()))
                .append_header((
                    "cache-control",
                    format!("public, max-age={}", cache_duration.as_secs()),
                ))
                .body(file.data.to_vec())
        }
        None => actix_web::HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("actix_server", LevelFilter::Warn)
        .filter_module("actix_web", LevelFilter::Warn)
        .init();

    info!("Webnote starting on http://0.0.0.0:{}", cli.port);

    let state = web::Data::new(AppState::new(&cli.db_dir));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::PayloadConfig::default().limit(52_428_800))
            .app_data(web::JsonConfig::default().limit(52_428_800))
            .app_data(web::FormConfig::default().limit(52_428_800))
            .service(redirect_path)
            .service(get_note)
            .service(invalid_path)
            .service(save_note)
            .service(static_files)
    })
    .bind(("0.0.0.0", cli.port))?
    .run()
    .await
}

fn generate_path() -> HttpResponse {
    let random_uid = uid::rand_string(4);

    HttpResponse::Found()
        .append_header(("Location", format!("/{}", random_uid)))
        .finish()
}

fn validate_directory(s: &str) -> Result<String, String> {
    let path = Path::new(s);

    if path.is_dir() {
        Ok(s.to_string())
    } else {
        Err("必须是一个有效的目录".into())
    }
}
