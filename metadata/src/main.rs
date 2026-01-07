mod config;
mod conversions;
mod dto;
mod error;
mod grpc_client;
mod handlers;
mod routes;
mod state;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use grpc_client::KnowledgeClient;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "metadata=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();
    tracing::info!("Metadata service starting...");
    tracing::info!("Server will listen on {}", config.server_address());

    // Connect to Knowledge service
    tracing::info!(
        "Connecting to Knowledge service at {}...",
        config.knowledge_uri()
    );
    let knowledge_client = KnowledgeClient::connect(config.knowledge_uri()).await?;
    tracing::info!("✓ Connected to Knowledge service");

    // Create shared application state (KnowledgeClient is Clone + thread-safe)
    let state = AppState { knowledge_client };

    // Build the router
    let app = routes::create_router(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    tracing::info!(
        "✓ Metadata REST API listening on {}",
        config.server_address()
    );

    axum::serve(listener, app).await?;

    Ok(())
}
