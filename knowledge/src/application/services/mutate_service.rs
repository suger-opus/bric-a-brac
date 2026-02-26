use crate::{infrastructure::repositories::MutateRepository, presentation::errors::AppError};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, EdgeDataDto, GraphIdDto, NodeDataDto,
};
use neo4rs::Graph;
use std::sync::Arc;

pub struct MutateService {
    pool: Arc<Graph>,
    repository: MutateRepository,
}

impl MutateService {
    pub fn new(pool: Arc<Graph>, repository: MutateRepository) -> Self {
        Self { pool, repository }
    }

    pub async fn insert_node(
        &self,
        graph_id: GraphIdDto,
        create_node_data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .insert_node(&mut txn, graph_id.into(), create_node_data.into())
            .await?;
        txn.commit().await?;

        Ok(node.into())
    }

    pub async fn insert_edge(
        &self,
        create_edge_data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let edge = self
            .repository
            .insert_edge(&mut txn, create_edge_data.into())
            .await?;
        txn.commit().await?;

        Ok(edge.into())
    }
}
