use crate::infrastructure::{config::KnowledgeServerConfig, errors::GrpcClientError};
use bric_a_brac_protos::{
    common::GraphDataProto,
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, DeleteGraphRequest,
        InitializeSchemaRequest, LoadGraphRequest,
    },
    with_retry, GrpcServiceKind,
};
use tonic::transport::Channel;
use tonic::Request;

#[derive(Clone)]
pub struct KnowledgeClient {
    client: KnowledgeGrpcClient<Channel>,
}

impl KnowledgeClient {
    pub fn new(config: &KnowledgeServerConfig) -> anyhow::Result<Self> {
        let channel =
            tonic::transport::Endpoint::from_shared(config.url().to_string())?.connect_lazy();
        Ok(Self {
            client: KnowledgeGrpcClient::new(channel),
        })
    }

    pub async fn load_graph(
        &self,
        graph_id: impl std::fmt::Display,
    ) -> Result<GraphDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_string();
        with_retry(GrpcServiceKind::Knowledge, "Failed to load graph", || {
            let mut c = client.clone();
            let req = Request::new(LoadGraphRequest {
                graph_id: graph_id.clone(),
            });
            async move { c.load_graph(req).await }
        })
        .await
        .map_err(Into::into)
    }

    pub async fn initialize_schema(
        &self,
        graph_id: impl std::fmt::Display,
        node_keys: Vec<String>,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_string();
        with_retry(
            GrpcServiceKind::Knowledge,
            "Failed to initialize schema",
            || {
                let mut c = client.clone();
                let req = Request::new(InitializeSchemaRequest {
                    graph_id: graph_id.clone(),
                    node_keys: node_keys.clone(),
                });
                async move { c.initialize_schema(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    pub async fn delete_graph(
        &self,
        graph_id: impl std::fmt::Display,
        node_keys: Vec<String>,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_string();
        with_retry(
            GrpcServiceKind::Knowledge,
            "Failed to delete graph data",
            || {
                let mut c = client.clone();
                let req = Request::new(DeleteGraphRequest {
                    graph_id: graph_id.clone(),
                    node_keys: node_keys.clone(),
                });
                async move { c.delete_graph(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }
}
