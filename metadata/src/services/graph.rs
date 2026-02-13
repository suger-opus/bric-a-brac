use crate::{
    clients::KnowledgeClient,
    error::ApiError,
    models::{
        EdgeData, EdgeSchema, GraphData, GraphId, GraphMetadata, GraphSchema, NewAccess,
        NewEdgeData, NewEdgeSchema, NewGraph, NewNodeData, NewNodeSchema, NodeData, NodeSchema,
        Role, UserId,
    },
    repositories::{AccessRepository, GraphRepository},
    services::ValidationService,
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

    pub async fn get_all_metadata(&self, user_id: UserId) -> Result<Vec<GraphMetadata>, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self.repository.get_all_metadata(&mut txn, user_id).await?;
        txn.commit().await?;
        Ok(graphs)
    }

    pub async fn get_metadata(
        &self,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<GraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph_id)
            .await?;
        txn.commit().await?;
        Ok(graph)
    }

    pub async fn get_schema(&self, graph_id: GraphId) -> Result<GraphSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let schema = self.repository.get_schema(&mut txn, graph_id).await?;
        txn.commit().await?;
        Ok(schema)
    }

    pub async fn create_graph(
        &self,
        user_id: UserId,
        new_graph: NewGraph,
    ) -> Result<GraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self.repository.create_graph(&mut txn, &new_graph).await?;
        let new_access = NewAccess {
            graph_id: graph.graph_id,
            user_id,
            role: Role::Owner,
        };
        self.access_repository
            .create_access(&mut txn, new_access)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph.graph_id)
            .await?;
        txn.commit().await?;

        Ok(graph)
    }

    pub async fn create_node_schema(
        &self,
        graph_id: GraphId,
        new_node_schema: NewNodeSchema,
    ) -> Result<NodeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .create_node_schema(&mut txn, graph_id, &new_node_schema)
            .await?;
        let properties = self
            .repository
            .create_properties(&mut txn, &new_node_schema.properties)
            .await?;
        txn.commit().await?;

        Ok(NodeSchema {
            properties,
            ..node_schema
        })
    }

    pub async fn create_edge_schema(
        &self,
        graph_id: GraphId,
        new_edge_schema: NewEdgeSchema,
    ) -> Result<EdgeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .create_edge_schema(&mut txn, graph_id, &new_edge_schema)
            .await?;
        let properties = self
            .repository
            .create_properties(&mut txn, &new_edge_schema.properties)
            .await?;
        txn.commit().await?;

        Ok(EdgeSchema {
            properties,
            ..edge_schema
        })
    }

    pub async fn get_data(&self, graph_id: GraphId) -> Result<GraphData, ApiError> {
        self.knowledge_client.load_graph(graph_id).await
    }

    pub async fn insert_node_data(
        &self,
        graph_id: GraphId,
        new_node_data: NewNodeData,
    ) -> Result<NodeData, ApiError> {
        let formatted_label = self
            .schema_validator
            .validate_node_data(new_node_data.node_schema_id, &new_node_data.properties)
            .await?;

        let node_data = self
            .knowledge_client
            .insert_node(graph_id, formatted_label, new_node_data)
            .await?;

        Ok(node_data)
    }

    pub async fn insert_edge_data(
        &self,
        _graph_id: GraphId,
        new_edge_data: NewEdgeData,
    ) -> Result<EdgeData, ApiError> {
        let formatted_label = self
            .schema_validator
            .validate_edge_data(new_edge_data.edge_schema_id, &new_edge_data.properties)
            .await?;

        let edge_data = self
            .knowledge_client
            .insert_edge(formatted_label, new_edge_data)
            .await?;

        Ok(edge_data)
    }
}
