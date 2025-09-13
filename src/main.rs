mod core;
mod db;
mod features;
mod mode;
mod utils;

use axum::extract::DefaultBodyLimit;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::error;

use crate::core::router;
use crate::db::{init_schemas, pool};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    if let Err(e) = application().await {
        error!("{e}");
        std::process::exit(1);
    }
}

async fn application() -> Result<(), Box<dyn std::error::Error>> {
    println!("v{}", env!("CARGO_PKG_VERSION"));

    features::inits()?;

    let pool = pool().await?;
    init_schemas(&pool).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port()));
    println!("Server running on {addr}");

    let middleware = ServiceBuilder::new()
        .layer(DefaultBodyLimit::max(5 << 20))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(addr).await?;
    let router = router()
        .merge(features::routers())
        .with_state(pool.clone())
        .layer(middleware)
        .into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    pool.close().await;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl+c");
    };

    #[cfg(unix)]
    let terminate = {
        async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to listen for signal")
                .recv()
                .await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub fn data_dir() -> String {
    env::var("DATA_DIR").ok().unwrap_or_else(|| {
        let mut path = env::current_exe().expect("failed to get current_exe path");
        path.pop();
        path.display().to_string()
    })
}

fn port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}
