use crate::infrastructure::{config::KnowledgeServerConfig, errors::GrpcClientError};
use bric_a_brac_protos::{
    common::{
        InsertEdgeDataProto, InsertNodeDataProto, NodeDataProto, NodeSummaryProto, PathProto,
        SubgraphProto, UpdateNodeDataProto,
    },
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, FindPathsRequest,
        GetNeighborsRequest, GetNodeRequest, InsertEdgeRequest, InsertNodeRequest,
        SearchNodesRequest, UpdateNodeRequest,
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
    pub fn new(config: KnowledgeServerConfig) -> Self {
        let channel = tonic::transport::Endpoint::from_shared(config.url().to_string())
            .expect("valid knowledge gRPC URL")
            .connect_lazy();
        Self {
            client: KnowledgeGrpcClient::new(channel),
        }
    }

    pub async fn insert_node(
        &self,
        graph_id: &str,
        node: InsertNodeDataProto,
    ) -> Result<NodeDataProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to insert node", || {
            let mut c = client.clone();
            let req = Request::new(InsertNodeRequest {
                graph_id: graph_id.clone(),
                node: Some(node.clone()),
            });
            async move { c.insert_node(req).await }
        })
        .await
        .map_err(Into::into)
    }

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

    pub async fn insert_edge(
        &self,
        graph_id: &str,
        edge: InsertEdgeDataProto,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Knowledge, "Failed to insert edge", || {
            let mut c = client.clone();
            let req = Request::new(InsertEdgeRequest {
                graph_id: graph_id.clone(),
                edge: Some(edge.clone()),
            });
            async move { c.insert_edge(req).await }
        })
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    pub async fn search_nodes(
        &self,
        graph_id: &str,
        node_key: Option<String>,
        query_embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSummaryProto>, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let response = with_retry(
            GrpcServiceKind::Knowledge,
            "Failed to search nodes",
            || {
                let mut c = client.clone();
                let req = Request::new(SearchNodesRequest {
                    graph_id: graph_id.clone(),
                    node_key: node_key.clone(),
                    query_embedding: query_embedding.clone(),
                    limit,
                });
                async move { c.search_nodes(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(response.nodes)
    }

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

    pub async fn get_neighbors(
        &self,
        graph_id: &str,
        node_data_id: &str,
        edge_key: Option<String>,
        depth: i32,
    ) -> Result<SubgraphProto, GrpcClientError> {
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
        response
            .subgraph
            .ok_or_else(|| {
                GrpcClientError::Base(bric_a_brac_protos::BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "get_neighbors: missing subgraph in response".to_owned(),
                    source: tonic::Status::internal("missing subgraph field"),
                })
            })
    }

    pub async fn find_paths(
        &self,
        graph_id: &str,
        from_node_data_id: &str,
        to_node_data_id: &str,
        max_depth: i32,
    ) -> Result<Vec<PathProto>, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let from_node_data_id = from_node_data_id.to_owned();
        let to_node_data_id = to_node_data_id.to_owned();
        let response = with_retry(
            GrpcServiceKind::Knowledge,
            "Failed to find paths",
            || {
                let mut c = client.clone();
                let req = Request::new(FindPathsRequest {
                    graph_id: graph_id.clone(),
                    from_node_data_id: from_node_data_id.clone(),
                    to_node_data_id: to_node_data_id.clone(),
                    max_depth,
                });
                async move { c.find_paths(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(response.paths)
    }
}
