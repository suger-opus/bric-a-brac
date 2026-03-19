use crate::{infrastructure::config::KnowledgeServerConfig, presentation::errors::GrpcClientError};
use bric_a_brac_protos::{
    common::{
        GraphDataProto, InsertEdgeDataProto, InsertNodeDataProto, NodeDataProto, NodeSummaryProto,
        SubgraphProto, UpdateNodeDataProto,
    },
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, FindPathsRequest,
        GetNeighborsRequest, GetNodeRequest, InsertEdgeRequest, InsertNodeRequest,
        SearchNodesRequest, UpdateNodeRequest,
    },
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use http::Uri;
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

    pub async fn insert_node(
        &self,
        graph_id: &str,
        node: InsertNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        match self.try_insert_node(graph_id, node.clone()).await {
            Ok(n) => Ok(n),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_insert_node(graph_id, node).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_insert_node(
        &self,
        graph_id: &str,
        node: InsertNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .insert_node(Request::new(InsertNodeRequest {
                graph_id: graph_id.to_owned(),
                node: Some(node),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to insert node".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn update_node(
        &self,
        graph_id: &str,
        node: UpdateNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        match self.try_update_node(graph_id, node.clone()).await {
            Ok(n) => Ok(n),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_update_node(graph_id, node).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_update_node(
        &self,
        graph_id: &str,
        node: UpdateNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .update_node(Request::new(UpdateNodeRequest {
                graph_id: graph_id.to_owned(),
                node: Some(node),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to update node".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn insert_edge(
        &self,
        graph_id: &str,
        edge: InsertEdgeDataProto,
    ) -> Result<(), GrpcClientError> {
        match self.try_insert_edge(graph_id, edge.clone()).await {
            Ok(()) => Ok(()),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_insert_edge(graph_id, edge).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_insert_edge(
        &self,
        graph_id: &str,
        edge: InsertEdgeDataProto,
    ) -> Result<(), GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        client
            .insert_edge(Request::new(InsertEdgeRequest {
                graph_id: graph_id.to_owned(),
                edge: Some(edge),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to insert edge".to_owned(),
                source: err,
            })?;
        Ok(())
    }

    pub async fn search_nodes(
        &self,
        graph_id: &str,
        node_key: Option<String>,
        query_embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSummaryProto>, GrpcClientError> {
        match self
            .try_search_nodes(graph_id, node_key.clone(), query_embedding.clone(), limit)
            .await
        {
            Ok(n) => Ok(n),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_search_nodes(graph_id, node_key, query_embedding, limit)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_search_nodes(
        &self,
        graph_id: &str,
        node_key: Option<String>,
        query_embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSummaryProto>, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .search_nodes(Request::new(SearchNodesRequest {
                graph_id: graph_id.to_owned(),
                node_key,
                query_embedding,
                limit,
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to search nodes".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner().nodes)
    }

    pub async fn get_node(
        &self,
        graph_id: &str,
        node_data_id: &str,
    ) -> Result<NodeDataProto, GrpcClientError> {
        match self.try_get_node(graph_id, node_data_id).await {
            Ok(n) => Ok(n),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_get_node(graph_id, node_data_id).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_get_node(
        &self,
        graph_id: &str,
        node_data_id: &str,
    ) -> Result<NodeDataProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .get_node(Request::new(GetNodeRequest {
                graph_id: graph_id.to_owned(),
                node_data_id: node_data_id.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to get node".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn get_neighbors(
        &self,
        graph_id: &str,
        node_data_id: &str,
        edge_key: Option<String>,
        depth: i32,
    ) -> Result<SubgraphProto, GrpcClientError> {
        match self
            .try_get_neighbors(graph_id, node_data_id, edge_key.clone(), depth)
            .await
        {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_get_neighbors(graph_id, node_data_id, edge_key, depth)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_get_neighbors(
        &self,
        graph_id: &str,
        node_data_id: &str,
        edge_key: Option<String>,
        depth: i32,
    ) -> Result<SubgraphProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .get_neighbors(Request::new(GetNeighborsRequest {
                graph_id: graph_id.to_owned(),
                node_data_id: node_data_id.to_owned(),
                edge_key,
                depth,
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to get neighbors".to_owned(),
                source: err,
            })?;
        Ok(response
            .into_inner()
            .subgraph
            .unwrap_or_default())
    }

    pub async fn find_paths(
        &self,
        graph_id: &str,
        from_node_data_id: &str,
        to_node_data_id: &str,
        max_depth: i32,
    ) -> Result<Vec<GraphDataProto>, GrpcClientError> {
        match self
            .try_find_paths(graph_id, from_node_data_id, to_node_data_id, max_depth)
            .await
        {
            Ok(p) => Ok(p),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_find_paths(graph_id, from_node_data_id, to_node_data_id, max_depth)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_find_paths(
        &self,
        graph_id: &str,
        from_node_data_id: &str,
        to_node_data_id: &str,
        max_depth: i32,
    ) -> Result<Vec<GraphDataProto>, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .find_paths(Request::new(FindPathsRequest {
                graph_id: graph_id.to_owned(),
                from_node_data_id: from_node_data_id.to_owned(),
                to_node_data_id: to_node_data_id.to_owned(),
                max_depth,
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "Failed to find paths".to_owned(),
                source: err,
            })?;
        Ok(response
            .into_inner()
            .paths
            .into_iter()
            .map(|p| GraphDataProto {
                nodes: p.nodes,
                edges: p.edges,
            })
            .collect())
    }
}
