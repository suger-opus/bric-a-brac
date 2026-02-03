use crate::dtos::graph_dto::{GraphMetadata, PostGraph};
use crate::error::ApiError;
use crate::grpc_client::KnowledgeClient;
use crate::models::{access_model::Role, graph_model::GraphId, user_model::UserId};
use crate::repositories::{access_repository::AccessRepository, graph_repository::GraphRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct GraphService {
    pool: PgPool,
    repository: GraphRepository,
    access_repository: AccessRepository,
    knowledge_client: KnowledgeClient,
}

impl GraphService {
    pub fn new(
        pool: &PgPool,
        repository: &GraphRepository,
        access_repository: &AccessRepository,
        knowledge_client: &KnowledgeClient,
    ) -> Self {
        GraphService {
            pool: pool.clone(),
            repository: repository.clone(),
            access_repository: access_repository.clone(),
            knowledge_client: knowledge_client.clone(),
        }
    }

    pub async fn post(
        &self,
        user_id: UserId,
        new_graph: PostGraph,
    ) -> Result<GraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self.repository.post(&mut txn, new_graph).await?;
        self.access_repository
            .post(&mut txn, graph.graph_id, user_id, Role::Owner)
            .await?;
        let graph = self
            .repository
            .get_one_metadata(&mut txn, user_id, graph.graph_id)
            .await?;
        txn.commit().await?;

        Ok(graph)
    }

    pub async fn get_one_metadata(
        &self,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<GraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self.repository.get_one_metadata(&mut txn, user_id, graph_id).await?;
        txn.commit().await?;
        Ok(graph)
    }

    pub async fn get_all_metadata(&self, user_id: UserId) -> Result<Vec<GraphMetadata>, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self
            .repository
            .get_all_metadata(&mut txn, user_id)
            .await?;
        txn.commit().await?;
        Ok(graphs)
    }
}
