use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};
use minify_js::TopLevelMode;

fn compression(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let source_path = entry.path();
        let target_path = dst.join(entry.file_name());

        if ty.is_dir() {
            compression(&source_path, &target_path)?;
        } else {
            match source_path.extension().and_then(|s| s.to_str()) {
                Some("js") => {
                    let js_code = fs::read(&source_path)?;
                    let session = minify_js::Session::new();
                    let mut minified_js = Vec::new();
                    minify_js::minify(
                        &session,
                        TopLevelMode::Module,
                        js_code.as_slice(),
                        &mut minified_js,
                    )
                    .unwrap();
                    fs::write(&target_path, &minified_js)?;
                }
                Some("css") => {
                    let css_code = fs::read_to_string(&source_path)?;
                    let mut stylesheet =
                        StyleSheet::parse(&css_code, ParserOptions::default()).unwrap();
                    stylesheet.minify(MinifyOptions::default()).unwrap();
                    let compressed_css = stylesheet.to_css(Default::default()).unwrap();
                    fs::write(&target_path, compressed_css.code.as_bytes())?;
                }
                _ => {
                    fs::copy(&source_path, &target_path)?;
                }
            }
        }
    }
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=templates/assets/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();

    let obj = if profile == "release" {
        let source_dir = PathBuf::from("templates/assets");
        let target_dir = PathBuf::from(&out_dir).join("assets");

        compression(&source_dir, &target_dir).unwrap();

        format!(
            r#"
            #[derive(rust_embed::RustEmbed)]
            #[folder = "{}/assets"]
            pub struct ReleaseAssets;
            "#,
            out_dir.replace('\\', "/")
        )
    } else {
        r#"
            #[derive(rust_embed::RustEmbed)]
            #[folder = "templates/assets/"]
            pub struct DebugAssets;
            "#
        .to_string()
    };

    let mut f = File::create(Path::new(&out_dir).join("rust_embed_assets.rs")).unwrap();
    writeln!(f, "{obj}").unwrap();
}
