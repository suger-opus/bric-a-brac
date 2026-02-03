pub mod config;
mod dtos;
mod error;
mod grpc_client;
mod handlers;
mod models;
mod repositories;
mod router;
mod services;
mod state;

use crate::config::Config;
use crate::state::ApiState;

pub async fn run(config: &Config) -> anyhow::Result<()> {
    tracing::info!("Initializing metadata service");

    let state = ApiState::from_config(config).await?;

    tracing::info!(
        server_url = %config.metadata_server.url(),
        "Starting HTTP server"
    );

    let listener = tokio::net::TcpListener::bind(&config.metadata_server.url()).await?;
    let routes = router::build(state);

    tracing::info!("Metadata REST API ready");
    axum::serve(listener, routes).await?;

    Ok(())
}
