pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::{
    infrastructure::config::Config,
    presentation::{grpc::MetadataService, router, state::ApiState},
};
use bric_a_brac_protos::metadata::metadata_server::MetadataServer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let state = ApiState::build(config).await?;

    let grpc_addr = config.metadata_server().grpc_url();
    let grpc_service = MetadataService::new();
    let grpc_server = tonic::transport::Server::builder()
        .add_service(MetadataServer::new(grpc_service))
        .serve(grpc_addr);
    tracing::info!(grpc_addr = %grpc_addr, "Metadata gRPC server starting");

    let http_listener = tokio::net::TcpListener::bind(&config.metadata_server().http_url()).await?;
    let http_routes = router::build(state);
    let http_addr = config.metadata_server().http_url();
    tracing::info!(http_addr = %http_addr, "Metadata REST API starting");

    tokio::select! {
        result = grpc_server => {
            if let Err(e) = result {
                tracing::error!(error = ?e, "gRPC server error");
            }
        }
        result = axum::serve(http_listener, http_routes) => {
            if let Err(e) = result {
                tracing::error!(error = ?e, "REST server error");
            }
        }
    }

    Ok(())
}

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "metadata=trace".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE),
        )
        .init();
}
