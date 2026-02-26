mod application;
mod domain;
pub mod infrastructure;
pub mod presentation;

use crate::{
    application::services::{MutateService, QueryService},
    infrastructure::{
        config::Config,
        database,
        repositories::{MutateRepository, QueryRepository},
    },
    presentation::grpc::KnowledgeService,
};
use bric_a_brac_protos::{build_grpc_server, knowledge::knowledge_server::KnowledgeServer};

pub async fn run(config: &Config) -> anyhow::Result<()> {
    let graph = database::connect(config.knowledge_db()).await?;

    let query_repository = QueryRepository::new();
    let mutate_repository = MutateRepository::new();
    let query_service = QueryService::new(graph.clone(), query_repository);
    let mutate_service = MutateService::new(graph, mutate_repository);
    let knowledge_service = KnowledgeService::new(query_service, mutate_service);
    let grpc_addr = config.knowledge_server().url();
    tracing::info!(grpc_addr = %grpc_addr, "Knowledge gRPC server starting");

    build_grpc_server(KnowledgeServer::new(knowledge_service), grpc_addr).await?;

    Ok(())
}
