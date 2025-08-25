use askama::Template;
use std::env;

include!(concat!(env!("OUT_DIR"), "/rust_embed_assets.rs"));

#[derive(Template)]
#[template(path = "index.html")]
pub struct Note {
    pub id: String,
    pub content: String,
}

#[cfg(debug_assertions)]
pub type Assets = DebugAssets;

#[cfg(not(debug_assertions))]
pub type Assets = ReleaseAssets;

pub fn data_dir() -> String {
    env::var("DATA_DIR").ok().unwrap_or_else(|| {
        let mut path = env::current_exe().unwrap();
        path.pop();
        path.display().to_string()
    })
}

pub fn port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}
