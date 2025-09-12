use askama::Template;
use std::env;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct Note {
    pub id: String,
    pub content: String,
}

include!(concat!(env!("OUT_DIR"), "/rust_embed_assets.rs"));

#[cfg(debug_assertions)]
pub type Assets = DebugAssets;

#[cfg(not(debug_assertions))]
pub type Assets = ReleaseAssets;
