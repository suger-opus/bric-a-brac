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
use bric_a_brac_protos::ai::ai_server::AiServer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run(config: Config) -> anyhow::Result<()> {
    let openrouter_client = OpenRouterClient::new(config.openrouter());
    let metadata_client = MetadataClient::new(config.metadata_server().clone());
    let schema_service = SchemaService::new(openrouter_client, metadata_client);
    let ai_service = AiService::new(schema_service);
    let grpc_addr = config.ai_server().url();
    tracing::info!(grpc_addr = %grpc_addr, "AI gRPC server starting");

    tonic::transport::Server::builder()
        .add_service(AiServer::new(ai_service))
        .serve(grpc_addr)
        .await?;

    Ok(())
}

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai=trace,tower_http=trace".into()),
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
