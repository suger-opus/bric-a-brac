mod config;
mod db;
mod grpc;
mod models;
mod service;

use config::Config;
use grpc::KnowledgeServer;
use tonic::transport::Server;

// todo: logging

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load();
    // todo: do not show sensitive info
    println!("{:?}", config);

    let addr = config.server_address().parse()?;
    let server = KnowledgeServer::new(config.clone()).await?;

    println!("\nKnowledge gRPC server listening on {}", addr);

    Server::builder()
        .add_service(server.into_service())
        .serve(addr)
        .await?;

    Ok(())
}
