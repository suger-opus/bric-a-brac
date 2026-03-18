use crate::{infrastructure::config::KnowledgeServerConfig, presentation::errors::GrpcClientError};
use axum::http::Uri;
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto};
use bric_a_brac_protos::{
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient,
        LoadGraphRequest,
    },
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use std::sync::{Arc, Mutex};
use tonic::Request;

#[derive(Clone)]
pub struct KnowledgeClient {
    config: KnowledgeServerConfig,
    client: Arc<Mutex<Option<KnowledgeGrpcClient<tonic::transport::Channel>>>>,
}

#[tonic::async_trait]
impl GrpcClient for KnowledgeClient {
    type Client = KnowledgeGrpcClient<tonic::transport::Channel>;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>> {
        &self.client
    }

    fn service_kind(&self) -> GrpcServiceKind {
        GrpcServiceKind::Knowledge
    }

    fn url(&self) -> &Uri {
        self.config.url()
    }

    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error> {
        KnowledgeGrpcClient::connect(self.url().clone()).await
    }
}

impl KnowledgeClient {
    pub fn new(config: KnowledgeServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.load_graph",
        skip(self, graph_id)
    )]
    pub async fn load_graph(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, GrpcClientError> {
        tracing::debug!(graph_id = ?graph_id);

        match self.try_load_graph(graph_id).await {
            Ok(graph) => Ok(graph),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", err);
                    self.reset_connection();
                    self.try_load_graph(graph_id).await
                } else {
                    Err(err)
                }
            }
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_client.try_load_graph",
        skip(self, graph_id)
    )]
    async fn try_load_graph(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(LoadGraphRequest {
            graph_id: graph_id.to_string(),
        });
        let response =
            client
                .load_graph(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to load graph from Knowledge service".to_string(),
                    source: err,
                })?;

        Ok(response.into_inner().try_into()?)
    }
}
