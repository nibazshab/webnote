use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use lightningcss::stylesheet::{MinifyOptions, ParserOptions, StyleSheet};
use minify_js::{minify, Session, TopLevelMode};

fn compression(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;

        let src = entry.path();
        let dst = dst.join(entry.file_name());

        if entry.file_type()?.is_dir() {
            compression(&src, &dst)?;
            continue;
        }

        match src.extension().and_then(|s| s.to_str()) {
            Some("js") => {
                let con = fs::read(&src)?;

                let s = Session::new();
                let mut obj = Vec::new();
                minify(&s, TopLevelMode::Module, con.as_slice(), &mut obj).unwrap();

                fs::write(&dst, &obj)?;
            }
            Some("css") => {
                let con = fs::read_to_string(&src)?;

                let mut s = StyleSheet::parse(&con, ParserOptions::default()).unwrap();
                s.minify(MinifyOptions::default()).unwrap();
                let obj = s.to_css(Default::default()).unwrap().code;

                fs::write(&dst, &obj)?;
            }
            _ => {
                fs::copy(&src, &dst)?;
            }
        }
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=templates/assets/");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    let obj = if env::var("PROFILE").unwrap() == "release" {
        let src = PathBuf::from("templates/assets");
        let dst = PathBuf::from(&out_dir).join("assets");

        compression(&src, &dst).unwrap();

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
