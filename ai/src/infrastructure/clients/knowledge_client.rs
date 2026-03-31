use crate::infrastructure::{InfraError, KnowledgeServerConfig};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, EdgeDataDto, EdgeDataIdDto, GraphDataDto, GraphIdDto,
    KeyDto, NodeDataDto, NodeDataIdDto, NodeSearchDto, UpdateEdgeDataDto, UpdateNodeDataDto,
};
use bric_a_brac_protos::{
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, CreateEdgeRequest,
        CreateNodeRequest, DeleteEdgeRequest, DeleteNodeRequest, FindPathsRequest,
        GetNeighborsRequest, GetNodeRequest, SearchNodesRequest, UpdateEdgeRequest,
        UpdateNodeRequest,
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

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.create_node",
        skip(self, graph_id, node),
        err
    )]
    pub async fn create_node(
        &self,
        graph_id: GraphIdDto,
        node: CreateNodeDataDto,
    ) -> Result<NodeDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(CreateNodeRequest {
                graph_id: graph_id.to_string(),
                node: Some(node.clone().into()),
            });
            async move { c.create_node(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.update_node",
        skip(self, graph_id, node),
        err
    )]
    pub async fn update_node(
        &self,
        graph_id: GraphIdDto,
        node: UpdateNodeDataDto,
    ) -> Result<NodeDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(UpdateNodeRequest {
                graph_id: graph_id.to_string(),
                node: Some(node.clone().into()),
            });
            async move { c.update_node(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.delete_node",
        skip(self, graph_id, node_data_id),
        err
    )]
    pub async fn delete_node(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
    ) -> Result<(), InfraError> {
        with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(DeleteNodeRequest {
                graph_id: graph_id.to_string(),
                node_data_id: node_data_id.to_string(),
            });
            async move { c.delete_node(req).await }
        })
        .await?;

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
        graph_id: GraphIdDto,
        edge_data_id: EdgeDataIdDto,
    ) -> Result<(), InfraError> {
        with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(DeleteEdgeRequest {
                graph_id: graph_id.to_string(),
                edge_data_id: edge_data_id.to_string(),
            });
            async move { c.delete_edge(req).await }
        })
        .await?;

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
        graph_id: GraphIdDto,
        edge: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(CreateEdgeRequest {
                graph_id: graph_id.to_string(),
                edge: Some(edge.clone().into()),
            });
            async move { c.create_edge(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.update_edge",
        skip(self, graph_id, edge),
        err
    )]
    pub async fn update_edge(
        &self,
        graph_id: GraphIdDto,
        edge: UpdateEdgeDataDto,
    ) -> Result<EdgeDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(UpdateEdgeRequest {
                graph_id: graph_id.to_string(),
                edge: Some(edge.clone().into()),
            });
            async move { c.update_edge(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.search_nodes",
        skip(self, graph_id, node_key, query_embedding, limit),
        err
    )]
    pub async fn search_nodes(
        &self,
        graph_id: GraphIdDto,
        node_key: Option<KeyDto>,
        query_embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSearchDto>, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(SearchNodesRequest {
                graph_id: graph_id.to_string(),
                node_key: node_key.clone().map(String::from),
                query_embedding: query_embedding.clone(),
                limit,
            });
            async move { c.search_nodes(req).await }
        })
        .await?;

        Ok(data
            .nodes
            .into_iter()
            .map(NodeSearchDto::try_from)
            .collect::<Result<_, _>>()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.get_node",
        skip(self, graph_id, node_data_id),
        err
    )]
    pub async fn get_node(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
    ) -> Result<NodeDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetNodeRequest {
                graph_id: graph_id.to_string(),
                node_data_id: node_data_id.to_string(),
            });
            async move { c.get_node(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.get_neighbors",
        skip(self, graph_id, node_data_id, edge_key, depth),
        err
    )]
    pub async fn get_neighbors(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
        edge_key: Option<KeyDto>,
        depth: i32,
    ) -> Result<GraphDataDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetNeighborsRequest {
                graph_id: graph_id.to_string(),
                node_data_id: node_data_id.to_string(),
                edge_key: edge_key.clone().map(String::from),
                depth,
            });
            async move { c.get_neighbors(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "knowledge_client.find_paths",
        skip(self, graph_id, from_node_data_id, to_node_data_id, max_depth),
        err
    )]
    pub async fn find_paths(
        &self,
        graph_id: GraphIdDto,
        from_node_data_id: NodeDataIdDto,
        to_node_data_id: NodeDataIdDto,
        max_depth: i32,
    ) -> Result<Vec<GraphDataDto>, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(FindPathsRequest {
                graph_id: graph_id.to_string(),
                from_node_data_id: from_node_data_id.to_string(),
                to_node_data_id: to_node_data_id.to_string(),
                max_depth,
            });
            async move { c.find_paths(req).await }
        })
        .await?;

        Ok(data
            .paths
            .into_iter()
            .map(GraphDataDto::try_from)
            .collect::<Result<_, _>>()?)
    }
}
