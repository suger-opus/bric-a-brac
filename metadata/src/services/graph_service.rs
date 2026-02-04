use crate::dtos::graph_dto::{
    ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeSchema, ResEdgeSchema, ResGraphMetadata,
    ResGraphSchema, ResNodeSchema,
};
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

    pub async fn get_all_metadata(
        &self,
        user_id: UserId,
    ) -> Result<Vec<ResGraphMetadata>, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self.repository.get_all_metadata(&mut txn, user_id).await?;
        txn.commit().await?;
        Ok(graphs)
    }

    pub async fn get_metadata(
        &self,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<ResGraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph_id)
            .await?;
        txn.commit().await?;
        Ok(graph)
    }

    pub async fn get_schema(&self, graph_id: GraphId) -> Result<ResGraphSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let schema = self.repository.get_schema(&mut txn, graph_id).await?;
        txn.commit().await?;
        Ok(schema)
    }

    pub async fn post(
        &self,
        user_id: UserId,
        new_graph: &ReqPostGraph,
    ) -> Result<ResGraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self.repository.post(&mut txn, new_graph).await?;
        self.access_repository
            .post(&mut txn, graph.graph_id, user_id, Role::Owner)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph.graph_id)
            .await?;
        txn.commit().await?;

        Ok(graph)
    }

    pub async fn post_node_schema(
        &self,
        graph_id: GraphId,
        new_node_schema: &ReqPostNodeSchema,
    ) -> Result<ResNodeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .post_node_schema(&mut txn, graph_id, new_node_schema)
            .await?;
        let properties = self
            .repository
            .post_properties(
                &mut txn,
                Some(node_schema.node_schema_id),
                None,
                &new_node_schema.properties,
            )
            .await?;
        txn.commit().await?;
        Ok(ResNodeSchema {
            node_schema,
            properties,
        })
    }

    pub async fn post_edge_schema(
        &self,
        graph_id: GraphId,
        new_edge_schema: &ReqPostEdgeSchema,
    ) -> Result<ResEdgeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .post_edge_schema(&mut txn, graph_id, new_edge_schema)
            .await?;
        let properties = self
            .repository
            .post_properties(
                &mut txn,
                None,
                Some(edge_schema.edge_schema_id),
                &new_edge_schema.properties,
            )
            .await?;
        txn.commit().await?;
        Ok(ResEdgeSchema {
            edge_schema,
            properties,
        })
    }
}
