mod application;
mod infrastructure;
mod presentation;

pub use infrastructure::Config;
pub use presentation::setup_tracing;

use crate::{
    application::{AgentService, ToolExecutor},
    infrastructure::{EmbeddingClient, KnowledgeClient, MetadataClient, OpenRouterClient},
    presentation::AiService,
};
use bric_a_brac_protos::{ai::ai_server::AiServer, build_grpc_server};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let openrouter_client = OpenRouterClient::new(config.openrouter());
    let embedding_client = EmbeddingClient::new(config.openrouter());
    let knowledge_client = KnowledgeClient::new(config.knowledge_server())?;
    let metadata_client = MetadataClient::new(config.metadata_server())?;
    let tool_executor =
        ToolExecutor::new(knowledge_client, metadata_client.clone(), embedding_client);
    let agent_service = AgentService::new(openrouter_client, metadata_client, tool_executor);
    let ai_service = AiService::new(agent_service);
    let grpc_addr = config.ai_server().url();
    tracing::info!(grpc_addr = %grpc_addr, "AI gRPC server starting");

    build_grpc_server(AiServer::new(ai_service), grpc_addr).await?;
    Ok(())
}
