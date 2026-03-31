use crate::infrastructure::{InfraError, KnowledgeServerConfig};
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto, KeyDto};
use bric_a_brac_protos::{
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, DeleteGraphRequest,
        InitializeSchemaRequest, LoadGraphRequest,
    },
    with_retry,
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

    pub async fn load_graph(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(LoadGraphRequest {
                graph_id: graph_id.to_string(),
            });
            async move { c.load_graph(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    pub async fn initialize_schema(
        &self,
        graph_id: GraphIdDto,
        node_keys: Vec<KeyDto>,
    ) -> Result<(), InfraError> {
        with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(InitializeSchemaRequest {
                graph_id: graph_id.to_string(),
                node_keys: node_keys
                    .clone()
                    .into_iter()
                    .map(|k| k.as_str().to_owned())
                    .collect(),
            });
            async move { c.initialize_schema(req).await }
        })
        .await?;

        Ok(())
    }

    pub async fn delete_graph(
        &self,
        graph_id: GraphIdDto,
        node_keys: Vec<KeyDto>,
    ) -> Result<(), InfraError> {
        with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(DeleteGraphRequest {
                graph_id: graph_id.to_string(),
                node_keys: node_keys
                    .clone()
                    .into_iter()
                    .map(|k| k.as_str().to_owned())
                    .collect(),
            });
            async move { c.delete_graph(req).await }
        })
        .await?;

        Ok(())
    }
}
