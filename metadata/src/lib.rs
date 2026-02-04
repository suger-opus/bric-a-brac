mod clients;
pub mod config;
pub mod dtos;
mod error;
mod extractors;
mod handlers;
pub mod models;
mod repositories;
mod router;
pub mod services;
pub mod state;

use crate::config::Config;
use crate::state::ApiState;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

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

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "metadata=trace,tower_http=trace,sqlx=trace,tonic=trace".into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::FULL),
        )
        .init();
}
