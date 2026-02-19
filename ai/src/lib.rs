pub mod application;
pub mod infrastructure;
pub mod presentation;

use crate::{
    application::services::SchemaService,
    infrastructure::{
        clients::{MetadataClient, OpenRouterClient},
        config::Config,
    },
    presentation::grpc::AiService,
};
use bric_a_brac_protos::{ai::ai_server::AiServer, build_grpc_server};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let openrouter_client = OpenRouterClient::new(config.openrouter());
    let metadata_client = MetadataClient::new(config.metadata_server().clone());
    let schema_service = SchemaService::new(openrouter_client, metadata_client);
    let ai_service = AiService::new(schema_service);
    let grpc_addr = config.ai_server().url();
    tracing::info!(grpc_addr = %grpc_addr, "AI gRPC server starting");

    build_grpc_server(AiServer::new(ai_service), grpc_addr).await?;

    Ok(())
}
