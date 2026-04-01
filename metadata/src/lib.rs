mod application;
mod domain;
mod infrastructure;
mod presentation;

pub use infrastructure::Config;
pub use presentation::setup_tracing;

use crate::presentation::{build_http_router, ApiState, MetadataGrpcService};
use bric_a_brac_protos::{build_grpc_server, metadata::metadata_server::MetadataServer};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let state = ApiState::build(config).await?;

    // HTTP server
    let http_listener = tokio::net::TcpListener::bind(&config.metadata_server().http_url()).await?;
    let http_routes = build_http_router(state.clone());
    let http_addr = config.metadata_server().http_url();
    tracing::info!(http_addr = %http_addr, "Metadata REST API starting");

    // gRPC server
    let grpc_service =
        MetadataGrpcService::new(state.session_service.clone(), state.graph_service.clone());
    let grpc_addr = config.metadata_server().grpc_url();
    tracing::info!(grpc_addr = %grpc_addr, "Metadata gRPC server starting");

    tokio::select! {
        result = axum::serve(http_listener, http_routes) => {
            result?;
        }
        result = build_grpc_server(MetadataServer::new(grpc_service), grpc_addr, config.internal_services_auth_token()) => {
            result?;
        }
    }
    Ok(())
}
