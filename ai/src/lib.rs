pub mod application;
pub mod infrastructure;
pub mod presentation;

use crate::{
    infrastructure::config::Config,
    presentation::grpc::AiService,
};
use bric_a_brac_protos::{ai::ai_server::AiServer, build_grpc_server};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let ai_service = AiService::new();
    let grpc_addr = config.ai_server().url();
    tracing::info!(grpc_addr = %grpc_addr, "AI gRPC server starting");

    build_grpc_server(AiServer::new(ai_service), grpc_addr).await?;

    Ok(())
}
