use super::ValidationService;
use crate::{
    application::dtos::{CreateGraphDto, GraphMetadataDto, UserIdDto},
    domain::models::{CreateAccessModel, CreateGraphSchemaModel, RoleModel},
    infrastructure::{
        clients::{AiClient, KnowledgeClient},
        repositories::{AccessRepository, GraphRepository},
    },
    presentation::errors::AppError,
};
use bric_a_brac_dtos::{
    CreateGraphDataDto, CreateGraphSchemaDto, GraphDataDto, GraphIdDto, GraphSchemaDto,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct GraphService {
    pool: PgPool,
    repository: GraphRepository,
    access_repository: AccessRepository,
    knowledge_client: KnowledgeClient,
    ai_client: AiClient,
    validation_service: ValidationService,
}

impl GraphService {
    pub fn new(
        pool: PgPool,
        repository: GraphRepository,
        access_repository: AccessRepository,
        knowledge_client: KnowledgeClient,
        ai_client: AiClient,
        validation_service: ValidationService,
    ) -> Self {
        GraphService {
            pool,
            repository,
            access_repository,
            knowledge_client,
            ai_client,
            validation_service,
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

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.create_schema",
        skip(self, graph_id, create_graph_schema)
    )]
    pub async fn create_schema(
        &self,
        graph_id: GraphIdDto,
        create_graph_schema: CreateGraphSchemaDto,
    ) -> Result<GraphSchemaDto, AppError> {
        let domain: CreateGraphSchemaModel = create_graph_schema.into();
        let properties = domain
            .nodes
            .clone()
            .into_iter()
            .flat_map(|node_schema| node_schema.properties.into_iter().map(|p| p))
            .chain(
                domain
                    .edges
                    .clone()
                    .into_iter()
                    .flat_map(|edge_schema| edge_schema.properties.into_iter().map(|p| p)),
            )
            .collect();

        let mut txn = self.pool.begin().await?;
        let _nodes_schemas = self
            .repository
            .create_nodes_schemas(&mut txn, graph_id.into(), domain.nodes)
            .await?;
        let _edges_schemas = self
            .repository
            .create_edges_schemas(&mut txn, graph_id.into(), domain.edges)
            .await?;
        let _properties = self
            .repository
            .create_properties(&mut txn, graph_id.into(), properties)
            .await?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.generate_schema",
        skip(self, _graph_id, file_content, file_type)
    )]
    pub async fn generate_schema(
        &self,
        _graph_id: GraphIdDto,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchemaDto, AppError> {
        let schema = self
            .ai_client
            .generate_schema(file_content, file_type)
            .await?;

        Ok(schema)
    }

    #[tracing::instrument(level = "trace", name = "graph_service.get_data", skip(self, graph_id))]
    pub async fn get_data(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, AppError> {
        Ok(self.knowledge_client.load_graph(graph_id).await?)
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.insert_data",
        skip(self, graph_id, create_graph_data)
    )]
    pub async fn insert_data(
        &self,
        graph_id: GraphIdDto,
        mut create_graph_data: CreateGraphDataDto,
    ) -> Result<GraphDataDto, AppError> {
        self.validation_service
            .validate_create_graph_data(graph_id, &mut create_graph_data)
            .await?;

        let graph_data = self
            .knowledge_client
            .insert_graph(graph_id, create_graph_data)
            .await?;

        Ok(graph_data)
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.generate_data",
        skip(self, graph_id, file_content, file_type)
    )]
    pub async fn generate_data(
        &self,
        graph_id: GraphIdDto,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphDataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        let data = self
            .ai_client
            .generate_data(schema.into(), file_content, file_type)
            .await?;

        Ok(data)
    }
}
