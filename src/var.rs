use askama::Template;

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
