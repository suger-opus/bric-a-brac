use crate::{infrastructure::repositories::QueryRepository, presentation::errors::AppError};
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto};
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

    pub async fn load_graph(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, AppError> {
        let mut txn = self.pool.start_txn().await?;
        let graph = self
            .repository
            .load_graph(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }
}
