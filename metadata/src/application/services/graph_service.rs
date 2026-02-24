use crate::{
    application::{
        dtos::{
            CreateEdgeDataDto, CreateEdgeSchemaDto, CreateGraphDto, CreateGraphSchemaDto,
            CreateNodeDataDto, CreateNodeSchemaDto, EdgeDataDto, EdgeSchemaDto, GraphDataDto,
            GraphMetadataDto, GraphSchemaDto, NodeDataDto, NodeSchemaDto,
        },
        services::ValidationService,
    },
    domain::models::{
        CreateAccess, CreateEdgeSchema, CreateNodeSchema, CreatePropertySchema, EdgeSchemaId,
        GraphId, NodeSchemaId, Role, UserId,
    },
    infrastructure::{
        clients::{AiClient, KnowledgeClient},
        repositories::{AccessRepository, GraphRepository},
    },
    presentation::errors::{AppError, DatabaseError},
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

    #[tracing::instrument(level = "trace", skip(self, user_id))]
    pub async fn get_all_metadata(
        &self,
        user_id: UserId,
    ) -> Result<Vec<GraphMetadataDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self.repository.get_all_metadata(&mut txn, user_id).await?;
        txn.commit().await?;

        Ok(graphs.into_iter().map(GraphMetadataDto::from).collect())
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, user_id))]
    pub async fn get_metadata(
        &self,
        graph_id: GraphId,
        user_id: UserId,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph_id, user_id)
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id))]
    pub async fn get_schema(&self, graph_id: GraphId) -> Result<GraphSchemaDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let schema = self.repository.get_schema(&mut txn, graph_id).await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(level = "trace", skip(self, user_id, create_graph_dto))]
    pub async fn create_graph(
        &self,
        user_id: UserId,
        create_graph_dto: CreateGraphDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .create_graph(&mut txn, create_graph_dto.into_domain())
            .await?;
        let create_access = CreateAccess {
            graph_id: graph.graph_id,
            user_id,
            role: Role::Owner,
        };
        self.access_repository
            .create(&mut txn, create_access)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph.graph_id, user_id)
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(level = "trace", skip(self, _graph_id, file_content, file_type))]
    pub async fn generate_schema(
        &self,
        _graph_id: GraphId,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchemaDto, AppError> {
        let schema = self
            .ai_client
            .generate_schema(file_content, file_type)
            .await?;

        Ok(schema)
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_graph_schema))]
    pub async fn create_schema(
        &self,
        graph_id: GraphId,
        create_graph_schema: CreateGraphSchemaDto,
    ) -> Result<GraphSchemaDto, AppError> {
        let domain = create_graph_schema.into_domain();
        let nodes_schemas_with_ids: Vec<(NodeSchemaId, CreateNodeSchema)> = domain
            .nodes
            .iter()
            .map(|n| (NodeSchemaId::new(), n.clone()))
            .collect();
        let edges_schemas_with_ids: Vec<(EdgeSchemaId, CreateEdgeSchema)> = domain
            .edges
            .iter()
            .map(|e| (EdgeSchemaId::new(), e.clone()))
            .collect();
        let properties_schemas_with_ids = nodes_schemas_with_ids
            .iter()
            .flat_map(|(node_schema_id, node_schema)| {
                node_schema.properties.iter().map(|p| CreatePropertySchema {
                    node_schema_id: Some(node_schema_id.clone()),
                    ..p.clone()
                })
            })
            .chain(
                edges_schemas_with_ids
                    .iter()
                    .flat_map(|(edge_schema_id, edge_schema)| {
                        edge_schema.properties.iter().map(|p| CreatePropertySchema {
                            edge_schema_id: Some(edge_schema_id.clone()),
                            ..p.clone()
                        })
                    }),
            )
            .collect();

        let mut txn = self.pool.begin().await?;
        let _nodes_schemas = self
            .repository
            .create_nodes_schemas(&mut txn, graph_id, nodes_schemas_with_ids)
            .await?;
        let _edges_schemas = self
            .repository
            .create_edges_schemas(&mut txn, graph_id, edges_schemas_with_ids)
            .await?;
        let _properties = self
            .repository
            .create_properties(&mut txn, properties_schemas_with_ids)
            .await?;
        let schema = self.repository.get_schema(&mut txn, graph_id).await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_node_schema))]
    pub async fn create_node_schema(
        &self,
        graph_id: GraphId,
        create_node_schema: CreateNodeSchemaDto,
    ) -> Result<NodeSchemaDto, AppError> {
        let domain = create_node_schema.into_domain();
        let mut txn = self.pool.begin().await?;
        let nodes_schemas = self
            .repository
            .create_nodes_schemas(
                &mut txn,
                graph_id,
                vec![(NodeSchemaId::new(), domain.clone())],
            )
            .await?;
        let properties = self
            .repository
            .create_properties(&mut txn, domain.properties)
            .await?;
        txn.commit().await?;

        Ok(NodeSchemaDto {
            properties: properties.into_iter().map(|p| p.into()).collect(),
            ..nodes_schemas
                .into_iter()
                .next()
                .ok_or_else(|| DatabaseError::UnexpectedState {
                    reason: "No rows returned for node schema".to_string(),
                })?
                .into()
        })
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_edge_schema))]
    pub async fn create_edge_schema(
        &self,
        graph_id: GraphId,
        create_edge_schema: CreateEdgeSchemaDto,
    ) -> Result<EdgeSchemaDto, AppError> {
        let domain = create_edge_schema.into_domain();
        let mut txn = self.pool.begin().await?;
        let edges_schemas = self
            .repository
            .create_edges_schemas(
                &mut txn,
                graph_id,
                vec![(EdgeSchemaId::new(), domain.clone())],
            )
            .await?;
        let properties = self
            .repository
            .create_properties(&mut txn, domain.properties)
            .await?;
        txn.commit().await?;

        Ok(EdgeSchemaDto {
            properties: properties.into_iter().map(|p| p.into()).collect(),
            ..edges_schemas
                .into_iter()
                .next()
                .ok_or_else(|| DatabaseError::UnexpectedState {
                    reason: "No rows returned for edge schema".to_string(),
                })?
                .into()
        })
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id))]
    pub async fn get_data(&self, graph_id: GraphId) -> Result<GraphDataDto, AppError> {
        Ok(self.knowledge_client.load_graph(graph_id).await?.into())
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_node_data))]
    pub async fn insert_node_data(
        &self,
        graph_id: GraphId,
        create_node_data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, AppError> {
        let domain = create_node_data.into_domain();

        self.validation_service
            .validate_create_node_data(&domain)
            .await?;

        let node_data = self.knowledge_client.insert_node(graph_id, domain).await?;

        Ok(node_data.into())
    }

    #[tracing::instrument(level = "trace", skip(self, _graph_id, create_edge_data))]
    pub async fn insert_edge_data(
        &self,
        _graph_id: GraphId,
        create_edge_data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, AppError> {
        let domain = create_edge_data.into_domain();

        self.validation_service
            .validate_create_edge_data(&domain)
            .await?;

        let edge_data = self.knowledge_client.insert_edge(domain).await?;

        Ok(edge_data.into())
    }
}
