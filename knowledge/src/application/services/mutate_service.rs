use crate::{
    domain::models::{
        EdgeDataModel, InsertEdgeDataModel, InsertNodeDataModel, NodeDataModel, UpdateNodeDataModel,
    },
    infrastructure::repositories::MutateRepository,
    presentation::errors::AppError,
};
use bric_a_brac_dtos::GraphIdDto;
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

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.insert_node",
        skip(self, graph_id, data)
    )]
    pub async fn insert_node(
        &self,
        graph_id: GraphIdDto,
        data: InsertNodeDataModel,
    ) -> Result<NodeDataModel, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .insert_node(&mut txn, graph_id.into(), data)
            .await?;
        txn.commit().await?;
        Ok(node)
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.update_node",
        skip(self, graph_id, data)
    )]
    pub async fn update_node(
        &self,
        graph_id: GraphIdDto,
        data: UpdateNodeDataModel,
    ) -> Result<NodeDataModel, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .update_node(&mut txn, graph_id.into(), data)
            .await?;
        txn.commit().await?;
        Ok(node)
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.insert_edge",
        skip(self, graph_id, data)
    )]
    pub async fn insert_edge(
        &self,
        graph_id: GraphIdDto,
        data: InsertEdgeDataModel,
    ) -> Result<EdgeDataModel, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let edge = self
            .repository
            .insert_edge(&mut txn, graph_id.into(), data)
            .await?;
        txn.commit().await?;
        Ok(edge)
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.initialize_schema",
        skip(self, node_keys)
    )]
    pub async fn initialize_schema(&self, node_keys: Vec<String>) -> Result<(), AppError> {
        self.repository
            .initialize_schema(&self.pool, node_keys)
            .await?;
        Ok(())
    }
}
