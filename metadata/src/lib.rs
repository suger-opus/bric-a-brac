pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::{
    infrastructure::config::Config,
    presentation::{grpc::MetadataService, router, state::ApiState},
};
use bric_a_brac_protos::{build_grpc_server, metadata::metadata_server::MetadataServer};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let state = ApiState::build(config).await?;

    let grpc_addr = config.metadata_server().grpc_url();
    let grpc_service = MetadataService::new();
    let grpc_server = build_grpc_server(MetadataServer::new(grpc_service), grpc_addr);
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
