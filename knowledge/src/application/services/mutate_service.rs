use crate::{
    application::errors::AppError,
    domain::models::{
        EdgeDataModel, InsertEdgeDataModel, InsertNodeDataModel, NodeDataModel,
        UpdateEdgeDataModel, UpdateNodeDataModel,
    },
    infrastructure::repositories::MutateRepository,
};
use bric_a_brac_dtos::{EdgeDataIdDto, GraphIdDto, NodeDataIdDto};
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
        name = "mutate_service.insert_node",
        skip(self, graph_id, data),
        err
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
        skip(self, graph_id, data),
        err
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
        skip(self, graph_id, data),
        err
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
        name = "mutate_service.update_edge",
        skip(self, graph_id, data),
        err
    )]
    pub async fn update_edge(
        &self,
        graph_id: GraphIdDto,
        data: UpdateEdgeDataModel,
    ) -> Result<EdgeDataModel, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let edge = self
            .repository
            .update_edge(&mut txn, graph_id.into(), data)
            .await?;
        txn.commit().await?;
        Ok(edge)
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
