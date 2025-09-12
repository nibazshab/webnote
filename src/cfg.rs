use std::env;

pub fn data_dir() -> String {
    env::var("DATA_DIR").ok().unwrap_or_else(|| {
        let mut path = env::current_exe().expect("failed to get current_exe path");
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
