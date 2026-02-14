use crate::{
    application::{
        dtos::{
            CreateEdgeDataDto, CreateEdgeSchemaDto, CreateGraphDto, CreateNodeDataDto,
            CreateNodeSchemaDto, EdgeDataDto, EdgeSchemaDto, GraphDataDto, GraphMetadataDto,
            GraphSchemaDto, NodeDataDto, NodeSchemaDto,
        },
        services::ValidationService,
    },
    domain::models::{CreateAccess, GraphId, Role, UserId},
    infrastructure::{
        clients::KnowledgeClient,
        repositories::{AccessRepository, GraphRepository},
    },
    presentation::error::{AppError, ResultExt},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct GraphService {
    pool: PgPool,
    repository: GraphRepository,
    access_repository: AccessRepository,
    knowledge_client: KnowledgeClient,
    schema_validator: ValidationService,
}

impl GraphService {
    pub fn new(
        pool: PgPool,
        repository: GraphRepository,
        access_repository: AccessRepository,
        knowledge_client: KnowledgeClient,
        schema_validator: ValidationService,
    ) -> Self {
        GraphService {
            pool,
            repository,
            access_repository,
            knowledge_client,
            schema_validator,
        }
    }

    pub async fn get_all_metadata(
        &self,
        user_id: UserId,
    ) -> Result<Vec<GraphMetadataDto>, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for get_all_metadata")?;
        let graphs = self
            .repository
            .get_all_metadata(&mut txn, user_id)
            .await
            .context("Failed to fetch all graph metadata from repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after fetching all metadata")?;

        Ok(graphs.into_iter().map(GraphMetadataDto::from).collect())
    }

    pub async fn get_metadata(
        &self,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for get_metadata")?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph_id)
            .await
            .context("Failed to fetch graph metadata from repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after fetching metadata")?;

        Ok(graph.into())
    }

    pub async fn get_schema(&self, graph_id: GraphId) -> Result<GraphSchemaDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for get_schema")?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id)
            .await
            .context("Failed to fetch graph schema from repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after fetching schema")?;

        Ok(schema.into())
    }

    pub async fn create_graph(
        &self,
        user_id: UserId,
        create_graph_dto: CreateGraphDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for create_graph")?;
        let graph = self
            .repository
            .create_graph(&mut txn, create_graph_dto.into_domain())
            .await
            .context("Failed to create graph in repository")?;
        let new_access = CreateAccess {
            graph_id: graph.graph_id,
            user_id,
            role: Role::Owner,
        };
        self.access_repository
            .create_access(&mut txn, new_access)
            .await
            .context("Failed to create owner access for graph")?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph.graph_id)
            .await
            .context("Failed to fetch created graph metadata")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after creating graph")?;

        Ok(graph.into())
    }

    pub async fn create_node_schema(
        &self,
        graph_id: GraphId,
        new_node_schema: CreateNodeSchemaDto,
    ) -> Result<NodeSchemaDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for create_node_schema")?;
        let node_schema = self
            .repository
            .create_node_schema(&mut txn, graph_id, new_node_schema.clone().into_domain())
            .await
            .context("Failed to create node schema in repository")?;
        let properties = self
            .repository
            .create_properties(&mut txn, new_node_schema.into_domain().properties)
            .await
            .context("Failed to create node schema properties in repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after creating node schema")?;

        Ok(NodeSchemaDto {
            properties: properties.into_iter().map(|p| p.into()).collect(),
            ..node_schema.into()
        })
    }

    pub async fn create_edge_schema(
        &self,
        graph_id: GraphId,
        create_edge_schema: CreateEdgeSchemaDto,
    ) -> Result<EdgeSchemaDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for create_edge_schema")?;
        let edge_schema = self
            .repository
            .create_edge_schema(&mut txn, graph_id, create_edge_schema.clone().into_domain())
            .await
            .context("Failed to create edge schema in repository")?;
        let properties = self
            .repository
            .create_properties(&mut txn, create_edge_schema.into_domain().properties)
            .await
            .context("Failed to create edge schema properties in repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after creating edge schema")?;

        Ok(EdgeSchemaDto {
            properties: properties.into_iter().map(|p| p.into()).collect(),
            ..edge_schema.into()
        })
    }

    pub async fn get_data(&self, graph_id: GraphId) -> Result<GraphDataDto, AppError> {
        Ok(self
            .knowledge_client
            .load_graph(graph_id)
            .await
            .context("Failed to load graph data from knowledge service")?
            .into())
    }

    pub async fn insert_node_data(
        &self,
        graph_id: GraphId,
        new_node_data: CreateNodeDataDto,
    ) -> Result<NodeDataDto, AppError> {
        let formatted_label = self
            .schema_validator
            .validate_node_data(
                new_node_data.node_schema_id,
                &new_node_data.clone().into_domain().properties,
            )
            .await
            .context("Failed to validate node data against schema")?;

        let node_data = self
            .knowledge_client
            .insert_node(graph_id, formatted_label, new_node_data.into_domain())
            .await
            .context("Failed to insert node data in knowledge service")?;

        Ok(node_data.into())
    }

    pub async fn insert_edge_data(
        &self,
        _graph_id: GraphId,
        new_edge_data: CreateEdgeDataDto,
    ) -> Result<EdgeDataDto, AppError> {
        let formatted_label = self
            .schema_validator
            .validate_edge_data(
                new_edge_data.edge_schema_id,
                &new_edge_data.clone().into_domain().properties,
            )
            .await
            .context("Failed to validate edge data against schema")?;

        let edge_data = self
            .knowledge_client
            .insert_edge(formatted_label, new_edge_data.into_domain())
            .await
            .context("Failed to insert edge data in knowledge service")?;

        Ok(edge_data.into())
    }
}
