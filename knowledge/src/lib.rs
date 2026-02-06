pub mod config;
mod database;
mod error;
mod repositories;
mod server;
mod services;

use crate::config::Config;
use crate::repositories::Repository;
use crate::server::KnowledgeServer;
use crate::services::Service;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let addr = config.knowledge_server.url();
    let graph = database::connect(&config.knowledge_db).await?;
    let repository = Repository::new();
    let service = Service::new(graph, repository).await?;
    let server = KnowledgeServer::new(service).await?;

    tracing::info!("Knowledge gRPC server listening on {}", addr);

    Server::builder()
        .add_service(server.into_service())
        .serve(addr)
        .await?;

    Ok(())
}

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
