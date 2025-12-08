use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::Router;
use server::route::manga::build_manga_routers;
use shared::logging::init_logger;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
}

async fn bootstrap() -> Result<()> {
    let app_config = shared::config::AppConfig::new()?;
    let registry = registry::init_app_registry(app_config).await?;

    let router = Router::new().merge(build_manga_routers()).layer(cors());

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, router.with_state(registry))
        .await
        .map_err(|e| e.into())
}
