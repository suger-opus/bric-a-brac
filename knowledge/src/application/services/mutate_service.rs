use crate::{infrastructure::repositories::MutateRepository, presentation::errors::AppError};
use bric_a_brac_dtos::{CreateGraphDataDto, GraphDataDto, GraphIdDto};
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
        name = "mutate_service.insert_graph",
        skip(self, graph_id, create_graph_data)
    )]
    pub async fn insert_graph(
        &self,
        graph_id: GraphIdDto,
        create_graph_data: CreateGraphDataDto,
    ) -> Result<GraphDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let data = self
            .repository
            .insert_graph(&mut txn, graph_id.into(), create_graph_data.into())
            .await?;
        txn.commit().await?;

        Ok(data.into())
    }
}
