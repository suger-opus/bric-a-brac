use crate::infrastructure::{config::KnowledgeServerConfig, errors::GrpcClientError};
use bric_a_brac_protos::{
    common::{
        CreateEdgeDataProto, CreateNodeDataProto, GraphDataProto, NodeDataProto, NodeSearchProto,
        UpdateEdgeDataProto, UpdateNodeDataProto,
    },
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, CreateEdgeRequest,
        CreateNodeRequest, DeleteEdgeRequest, DeleteNodeRequest, FindPathsRequest,
        GetNeighborsRequest, GetNodeRequest, SearchNodesRequest, UpdateEdgeRequest,
        UpdateNodeRequest,
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

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.create_node",
        skip(self, graph_id, node),
        err
    )]
    pub async fn create_node(
        &self,
        graph_id: &str,
        node: CreateNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to create node", || {
            let mut c = client.clone();
            let req = Request::new(CreateNodeRequest {
                graph_id: graph_id.clone(),
                node: Some(node.clone()),
            });
            async move { c.create_node(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.update_node",
        skip(self, graph_id, node),
        err
    )]
    pub async fn update_node(
        &self,
        graph_id: &str,
        node: UpdateNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to update node", || {
            let mut c = client.clone();
            let req = Request::new(UpdateNodeRequest {
                graph_id: graph_id.clone(),
                node: Some(node.clone()),
            });
            async move { c.update_node(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.delete_node",
        skip(self, graph_id, node_data_id),
        err
    )]
    pub async fn delete_node(
        &self,
        graph_id: &str,
        node_data_id: &str,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let node_data_id = node_data_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to delete node", || {
            let mut c = client.clone();
            let req = Request::new(DeleteNodeRequest {
                graph_id: graph_id.clone(),
                node_data_id: node_data_id.clone(),
            });
            async move { c.delete_node(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.delete_edge",
        skip(self, graph_id, edge_data_id),
        err
    )]
    pub async fn delete_edge(
        &self,
        graph_id: &str,
        edge_data_id: &str,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let edge_data_id = edge_data_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to delete edge", || {
            let mut c = client.clone();
            let req = Request::new(DeleteEdgeRequest {
                graph_id: graph_id.clone(),
                edge_data_id: edge_data_id.clone(),
            });
            async move { c.delete_edge(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.create_edge",
        skip(self, graph_id, edge),
        err
    )]
    pub async fn create_edge(
        &self,
        graph_id: &str,
        edge: CreateEdgeDataProto,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to create edge", || {
            let mut c = client.clone();
            let req = Request::new(CreateEdgeRequest {
                graph_id: graph_id.clone(),
                edge: Some(edge.clone()),
            });
            async move { c.create_edge(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.update_edge",
        skip(self, graph_id, edge),
        err
    )]
    pub async fn update_edge(
        &self,
        graph_id: &str,
        edge: UpdateEdgeDataProto,
    ) -> Result<bric_a_brac_protos::common::EdgeDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to update edge", || {
            let mut c = client.clone();
            let req = Request::new(UpdateEdgeRequest {
                graph_id: graph_id.clone(),
                edge: Some(edge.clone()),
            });
            async move { c.update_edge(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.search_nodes",
        skip(self, graph_id, node_key, query_embedding, limit),
        err
    )]
    pub async fn search_nodes(
        &self,
        graph_id: &str,
        node_key: Option<String>,
        query_embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSearchProto>, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let response = with_retry(GrpcServiceKind::Knowledge, "Failed to search nodes", || {
            let mut c = client.clone();
            let req = Request::new(SearchNodesRequest {
                graph_id: graph_id.clone(),
                node_key: node_key.clone(),
                query_embedding: query_embedding.clone(),
                limit,
            });
            async move { c.search_nodes(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(response.nodes)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.get_node",
        skip(self, graph_id, node_data_id),
        err
    )]
    pub async fn get_node(
        &self,
        graph_id: &str,
        node_data_id: &str,
    ) -> Result<NodeDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let node_data_id = node_data_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to get node", || {
            let mut c = client.clone();
            let req = Request::new(GetNodeRequest {
                graph_id: graph_id.clone(),
                node_data_id: node_data_id.clone(),
            });
            async move { c.get_node(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.get_neighbors",
        skip(self, graph_id, node_data_id, edge_key, depth),
        err
    )]
    pub async fn get_neighbors(
        &self,
        graph_id: &str,
        node_data_id: &str,
        edge_key: Option<String>,
        depth: i32,
    ) -> Result<GraphDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let node_data_id = node_data_id.to_owned();
        let response = with_retry(
            GrpcServiceKind::Knowledge,
            "Failed to get neighbors",
            || {
                let mut c = client.clone();
                let req = Request::new(GetNeighborsRequest {
                    graph_id: graph_id.clone(),
                    node_data_id: node_data_id.clone(),
                    edge_key: edge_key.clone(),
                    depth,
                });
                async move { c.get_neighbors(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        response.subgraph.ok_or_else(|| {
            GrpcClientError::Base(bric_a_brac_protos::BaseGrpcClientError::Request {
                service: GrpcServiceKind::Knowledge,
                message: "get_neighbors: missing subgraph in response".to_owned(),
                source: tonic::Status::internal("missing subgraph field"),
            })
        })
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.find_paths",
        skip(self, graph_id, from_node_data_id, to_node_data_id, max_depth),
        err
    )]
    pub async fn find_paths(
        &self,
        graph_id: &str,
        from_node_data_id: &str,
        to_node_data_id: &str,
        max_depth: i32,
    ) -> Result<Vec<GraphDataProto>, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let from_node_data_id = from_node_data_id.to_owned();
        let to_node_data_id = to_node_data_id.to_owned();
        let response = with_retry(GrpcServiceKind::Knowledge, "Failed to find paths", || {
            let mut c = client.clone();
            let req = Request::new(FindPathsRequest {
                graph_id: graph_id.clone(),
                from_node_data_id: from_node_data_id.clone(),
                to_node_data_id: to_node_data_id.clone(),
                max_depth,
            });
            async move { c.find_paths(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(response.paths)
    }
}
