use crate::{
    domain::models::{GraphDataModel, NodeDataModel, NodeSummaryModel},
    domain::utils::validate_depth,
    infrastructure::repositories::QueryRepository,
    presentation::errors::AppError,
};
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto, NodeDataIdDto};
use neo4rs::Graph;
use std::sync::Arc;

pub struct QueryService {
    pool: Arc<Graph>,
    repository: QueryRepository,
}

impl QueryService {
    pub fn new(pool: Arc<Graph>, repository: QueryRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_service.load_graph",
        skip(self, graph_id)
    )]
    pub async fn load_graph(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let graph = self
            .repository
            .load_graph(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_service.get_node",
        skip(self, graph_id, node_data_id)
    )]
    pub async fn get_node(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
    ) -> Result<NodeDataModel, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .get_node(&mut txn, graph_id.into(), node_data_id.into())
            .await?;
        txn.commit().await?;
        Ok(node)
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_service.search_nodes",
        skip(self, graph_id, embedding)
    )]
    pub async fn search_nodes(
        &self,
        graph_id: GraphIdDto,
        node_key: Option<String>,
        embedding: Vec<f32>,
        limit: i32,
    ) -> Result<Vec<NodeSummaryModel>, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let results = self
            .repository
            .search_nodes(&mut txn, graph_id.into(), node_key, embedding, limit)
            .await?;
        txn.commit().await?;
        Ok(results)
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_service.get_neighbors",
        skip(self, graph_id, node_data_id)
    )]
    pub async fn get_neighbors(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
        edge_key: Option<String>,
        depth: i32,
    ) -> Result<GraphDataModel, AppError> {
        let depth = validate_depth(depth)?;
        let mut txn = self.pool.start_txn().await?;
        let subgraph = self
            .repository
            .get_neighbors(
                &mut txn,
                graph_id.into(),
                node_data_id.into(),
                edge_key,
                depth,
            )
            .await?;
        txn.commit().await?;
        Ok(subgraph)
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_service.find_paths",
        skip(self, graph_id, from_id, to_id)
    )]
    pub async fn find_paths(
        &self,
        graph_id: GraphIdDto,
        from_id: NodeDataIdDto,
        to_id: NodeDataIdDto,
        max_depth: i32,
    ) -> Result<Vec<GraphDataModel>, AppError> {
        let max_depth = validate_depth(max_depth)?;
        let mut txn = self.pool.start_txn().await?;
        let paths = self
            .repository
            .find_paths(
                &mut txn,
                graph_id.into(),
                from_id.into(),
                to_id.into(),
                max_depth,
            )
            .await?;
        txn.commit().await?;
        Ok(paths)
    }
}
