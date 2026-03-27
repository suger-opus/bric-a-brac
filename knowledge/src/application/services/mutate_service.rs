use crate::{application::AppError, infrastructure::MutateRepository};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, EdgeDataDto, EdgeDataIdDto, GraphIdDto, NodeDataDto,
    NodeDataIdDto, UpdateEdgeDataDto, UpdateNodeDataDto,
};
use neo4rs::Graph;
use std::sync::Arc;

pub struct MutateService {
    pool: Arc<Graph>,
    repository: MutateRepository,
}

impl MutateService {
    pub const fn new(pool: Arc<Graph>, repository: MutateRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.create_node",
        skip(self, graph_id, data),
        err
    )]
    pub async fn create_node(
        &self,
        graph_id: GraphIdDto,
        data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .create_node(&mut txn, graph_id.into(), data.into())
            .await?;
        txn.commit().await?;

        Ok(node.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.update_node",
        skip(self, graph_id, data),
        err
    )]
    pub async fn update_node(
        &self,
        graph_id: GraphIdDto,
        data: UpdateNodeDataDto,
    ) -> Result<NodeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let node = self
            .repository
            .update_node(&mut txn, graph_id.into(), data.into())
            .await?;
        txn.commit().await?;

        Ok(node.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.create_edge",
        skip(self, graph_id, data),
        err
    )]
    pub async fn create_edge(
        &self,
        graph_id: GraphIdDto,
        data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let edge = self
            .repository
            .create_edge(&mut txn, graph_id.into(), data.into())
            .await?;
        txn.commit().await?;

        Ok(edge.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.update_edge",
        skip(self, graph_id, data),
        err
    )]
    pub async fn update_edge(
        &self,
        graph_id: GraphIdDto,
        data: UpdateEdgeDataDto,
    ) -> Result<EdgeDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let edge = self
            .repository
            .update_edge(&mut txn, graph_id.into(), data.into())
            .await?;
        txn.commit().await?;

        Ok(edge.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.delete_node",
        skip(self, graph_id, node_data_id),
        err
    )]
    pub async fn delete_node(
        &self,
        graph_id: GraphIdDto,
        node_data_id: NodeDataIdDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.start_txn().await?;
        self.repository
            .delete_node(&mut txn, graph_id.into(), node_data_id.into())
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.delete_edge",
        skip(self, graph_id, edge_data_id),
        err
    )]
    pub async fn delete_edge(
        &self,
        graph_id: GraphIdDto,
        edge_data_id: EdgeDataIdDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.start_txn().await?;
        self.repository
            .delete_edge(&mut txn, graph_id.into(), edge_data_id.into())
            .await?;
        txn.commit().await?;
        Ok(())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.delete_graph_data",
        skip(self, graph_id, node_keys),
        err
    )]
    pub async fn delete_graph_data(
        &self,
        graph_id: GraphIdDto,
        node_keys: Vec<String>,
    ) -> Result<(), AppError> {
        self.repository
            .delete_graph_data(&self.pool, graph_id.into(), node_keys)
            .await?;
        Ok(())
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_service.initialize_schema",
        skip(self, node_keys),
        err
    )]
    pub async fn initialize_schema(&self, node_keys: Vec<String>) -> Result<(), AppError> {
        self.repository
            .initialize_schema(&self.pool, node_keys)
            .await?;
        Ok(())
    }
}
