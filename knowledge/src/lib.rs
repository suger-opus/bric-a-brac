pub mod config;
mod db;
mod grpc;
mod models;
mod service;

use crate::config::Config;
use crate::grpc::KnowledgeServer;
use tonic::transport::Server;

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let addr = config.knowledge_server.url();
    let server = KnowledgeServer::new(config.clone()).await?;

    tracing::info!("Knowledge gRPC server listening on {}", addr);

    Server::builder()
        .add_service(server.into_service())
        .serve(addr)
        .await?;

    Ok(())
}
