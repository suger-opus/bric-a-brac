use crate::{
    application::dtos::{CreateGraphDto, GraphMetadataDto, UserIdDto},
    domain::models::{CreateAccessModel, RoleModel},
    infrastructure::{
        clients::KnowledgeClient,
        repositories::{AccessRepository, GraphRepository},
    },
    presentation::errors::AppError,
};
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto, GraphSchemaDto};
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
        pool: PgPool,
        repository: GraphRepository,
        access_repository: AccessRepository,
        knowledge_client: KnowledgeClient,
    ) -> Self {
        GraphService {
            pool,
            repository,
            access_repository,
            knowledge_client,
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_all_metadata",
        skip(self, user_id)
    )]
    pub async fn get_all_metadata(
        &self,
        user_id: UserIdDto,
    ) -> Result<Vec<GraphMetadataDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self
            .repository
            .get_all_metadata(&mut txn, user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graphs.into_iter().map(GraphMetadataDto::from).collect())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.create_graph",
        skip(self, user_id, create_graph)
    )]
    pub async fn create_graph(
        &self,
        user_id: UserIdDto,
        create_graph: CreateGraphDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .create_graph(&mut txn, create_graph.into())
            .await?;
        let create_access = CreateAccessModel {
            graph_id: graph.graph_id,
            user_id: user_id.into(),
            role: RoleModel::Owner,
        };
        self.access_repository
            .create(&mut txn, create_access)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph.graph_id, user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_metadata",
        skip(self, graph_id, user_id)
    )]
    pub async fn get_metadata(
        &self,
        graph_id: GraphIdDto,
        user_id: UserIdDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph_id.into(), user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_schema",
        skip(self, graph_id)
    )]
    pub async fn get_schema(&self, graph_id: GraphIdDto) -> Result<GraphSchemaDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(level = "trace", name = "graph_service.get_data", skip(self, graph_id))]
    pub async fn get_data(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, AppError> {
        Ok(self.knowledge_client.load_graph(graph_id).await?)
    }
}
