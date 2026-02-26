use crate::{infrastructure::config::KnowledgeServerConfig, presentation::errors::GrpcClientError};
use axum::http::Uri;
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, EdgeDataDto, GraphDataDto, GraphIdDto, NodeDataDto,
};
use bric_a_brac_protos::{
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, InsertEdgeRequest,
        InsertNodeRequest, LoadGraphRequest,
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

    #[tracing::instrument(level = "debug", skip(self, graph_id, node_data))]
    pub async fn insert_node(
        &self,
        graph_id: GraphIdDto,
        node_data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, GrpcClientError> {
        tracing::debug!(graph_id = ?graph_id, key = ?node_data.key);

        match self.try_insert_node(graph_id, node_data.clone()).await {
            Ok(node) => Ok(node),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", err);
                    self.reset_connection();
                    self.try_insert_node(graph_id, node_data).await
                } else {
                    Err(err)
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self, graph_id, edge_data))]
    pub async fn insert_edge(
        &self,
        graph_id: GraphIdDto,
        edge_data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, GrpcClientError> {
        tracing::debug!(graph_id = ?graph_id, key = ?edge_data.key, from_node_data_id = ?edge_data.from_node_data_id, to_node_data_id = ?edge_data.to_node_data_id);

        match self.try_insert_edge(graph_id, edge_data.clone()).await {
            Ok(edge) => Ok(edge),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", err);
                    self.reset_connection();
                    self.try_insert_edge(graph_id, edge_data).await
                } else {
                    Err(err)
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self, graph_id))]
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

    #[tracing::instrument(level = "trace", skip(self, graph_id, node_data))]
    async fn try_insert_node(
        &self,
        graph_id: GraphIdDto,
        node_data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertNodeRequest {
            graph_id: graph_id.to_string(),
            node_data: Some(node_data.into()),
        });

        let response =
            client
                .insert_node(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to insert node in Knowledge service".to_string(),
                    source: err,
                })?;

        Ok(response.into_inner().try_into()?)
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, edge_data))]
    async fn try_insert_edge(
        &self,
        graph_id: GraphIdDto,
        edge_data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertEdgeRequest {
            graph_id: graph_id.to_string(),
            edge_data: Some(edge_data.into()),
        });

        let response =
            client
                .insert_edge(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to insert edge in Knowledge service".to_string(),
                    source: err,
                })?;

        Ok(response.into_inner().try_into()?)
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id))]
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
