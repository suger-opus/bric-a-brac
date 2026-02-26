pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::{
    infrastructure::config::Config,
    presentation::{router, state::ApiState},
};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let state = ApiState::build(config).await?;

    let http_listener = tokio::net::TcpListener::bind(&config.metadata_server().http_url()).await?;
    let http_routes = router::build(state);
    let http_addr = config.metadata_server().http_url();
    tracing::info!(http_addr = %http_addr, "Metadata REST API starting");

    axum::serve(http_listener, http_routes).await?;
    Ok(())
}
