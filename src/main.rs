use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use askama::Template;
use clap::Parser;
use db::AppState;
use env_logger::Builder;
use log::LevelFilter;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::path::Path;

mod db;
mod uid;

const DEFAULT_PORT: u16 = 10003;
const DEFAULT_UID_LENGTH: usize = 4;
const MAX_UID_LENGTH: usize = 16;
const CACHE_DURATION_SECONDS: u64 = 60 * 60 * 24;
const MAX_PAYLOAD_SIZE: usize = 50 << 20;
const CLI_USER_AGENTS: [&str; 2] = ["curl", "wget"];

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
    #[arg(short = 'P', long, default_value_t = DEFAULT_PORT, value_name = "PORT")]
    port: u16,

    #[arg(short = 'D', long, value_parser = validate_directory, value_name = "DIR")]
    db_dir: Option<String>,
}

#[get("/")]
async fn redirect_path() -> impl Responder {
    generate_path()
}

#[get("/{uid}")]
async fn get_note(
    req: HttpRequest,
    uid: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let uid = uid.into_inner();

    if uid.len() > MAX_UID_LENGTH {
        return generate_path();
    }

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_lowercase();

    let is_cli_tool = CLI_USER_AGENTS
        .iter()
        .any(|agent| user_agent.contains(agent));

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
            .body(
                HtmlResponse { uid, content }
                    .render()
                    .expect("Template rendering failed"),
            )
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
    data: web::Data<AppState>,
) -> impl Responder {
    let uid = uid.into_inner();

    if uid.len() > MAX_UID_LENGTH {
        return HttpResponse::BadRequest().body(format!("UID length exceeds {}", MAX_UID_LENGTH));
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

    log::info!("{} | {}", uid, client_ip);

    match data.save_content(&uid, &form.t) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/assets/{filename:.*}")]
async fn static_files(path: web::Path<String>) -> actix_web::HttpResponse {
    let filename = path.into_inner();

    match Assets::get(&filename) {
        Some(file) => {
            let mime = mime_guess::from_path(&filename).first_or_octet_stream();

            actix_web::HttpResponse::Ok()
                .content_type(mime.as_ref())
                .append_header((
                    "cache-control",
                    format!("public, max-age={}", CACHE_DURATION_SECONDS),
                ))
                .body(file.data.to_vec())
        }
        None => actix_web::HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let db_dir = cli.db_dir.unwrap_or_else(|| {
        let exe_path = std::env::current_exe().unwrap_or_else(|e| {
            log::error!("{}", e);
            std::process::exit(1);
        });
        exe_path
            .parent()
            .unwrap_or_else(|| {
                log::error!("{}", "Path error");
                std::process::exit(1);
            })
            .to_string_lossy()
            .into_owned()
    });

    Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_module("actix_server", LevelFilter::Warn)
        .filter_module("actix_web", LevelFilter::Warn)
        .init();

    log::info!("Webnote starting on http://0.0.0.0:{}", cli.port);

    let state = match AppState::new(&db_dir) {
        Ok(app_state) => web::Data::new(app_state),
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::PayloadConfig::default().limit(MAX_PAYLOAD_SIZE))
            .app_data(web::JsonConfig::default().limit(MAX_PAYLOAD_SIZE))
            .app_data(web::FormConfig::default().limit(MAX_PAYLOAD_SIZE))
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
    let random_uid = uid::rand_string(DEFAULT_UID_LENGTH);

    HttpResponse::Found()
        .append_header(("Location", format!("/{}", random_uid)))
        .finish()
}

fn validate_directory(s: &str) -> Result<String, String> {
    let path = Path::new(s);
    path.is_dir()
        .then(|| s.to_string())
        .ok_or_else(|| "必须是一个有效的目录".into())
}
