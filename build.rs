use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn compression(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    // todo: copy -> compression
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());

        if ty.is_dir() {
            compression(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
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
