use crate::{
    domain::models::{
        CreateEdgeSchemaModel, CreateGraphModel, CreateNodeSchemaModel, EdgeSchemaIdModel,
        EdgeSchemaModel, GraphIdModel, GraphMetadataModel, GraphModel, GraphSchemaModel,
        NodeSchemaIdModel, NodeSchemaModel, RoleModel, UserIdModel,
    },
    infrastructure::errors::DatabaseError,
};
use chrono::{DateTime, Utc};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct GraphRepository;

impl GraphRepository {
    pub fn new() -> Self {
        GraphRepository
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.get_all_metadata",
        skip(self, connection, user_id),
        err
    )]
    pub async fn get_all_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserIdModel,
    ) -> Result<Vec<GraphMetadataModel>, DatabaseError> {
        tracing::debug!(user_id = ?user_id);

        let graphs = sqlx::query_as!(
            GraphMetadataRow,
            r#"
SELECT
    g.graph_id,
    u.username AS owner_username,
    g.created_at,
    g.updated_at,
    g.name,
    g.description,
    user_access.role AS "user_role!:_",
    g.is_public
FROM graphs g
JOIN accesses user_access ON g.graph_id = user_access.graph_id AND user_access.user_id = $1 AND user_access.role IN ('Owner', 'Admin', 'Editor', 'Viewer')
JOIN accesses owner_access ON g.graph_id = owner_access.graph_id AND owner_access.role = 'Owner'
JOIN users u ON owner_access.user_id = u.user_id
            "#,
            user_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(graphs.into_iter().map(GraphMetadataRow::into).collect())
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.get_metadata",
        skip(self, connection, graph_id, user_id),
        err
    )]
    pub async fn get_metadata(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        user_id: UserIdModel,
    ) -> Result<GraphMetadataModel, DatabaseError> {
        tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

        let graph = sqlx::query_as!(
            GraphMetadataRow,
            r#"
SELECT
    g.graph_id,
    u.username AS owner_username,
    g.created_at,
    g.updated_at,
    g.name,
    g.description,
    COALESCE(user_access.role, 'None') AS "user_role!:_",
    g.is_public
FROM graphs g
LEFT JOIN accesses user_access ON g.graph_id = user_access.graph_id AND user_access.user_id = $1
JOIN accesses owner_access ON g.graph_id = owner_access.graph_id AND owner_access.role = 'Owner'
JOIN users u ON owner_access.user_id = u.user_id
WHERE g.graph_id = $2
            "#,
            user_id as _,
            graph_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.get_schema",
        skip(self, connection, graph_id),
        err
    )]
    pub async fn get_schema(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
    ) -> Result<GraphSchemaModel, DatabaseError> {
        tracing::debug!(graph_id = ?graph_id);

        let nodes = sqlx::query_as!(
            NodeSchemaRow,
            r#"
SELECT
    node_schema_id AS "node_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    description,
    created_at,
    updated_at
FROM nodes_schemas
WHERE graph_id = $1
            "#,
            graph_id as _,
        )
        .fetch_all(&mut *connection)
        .await?;

        let edges = sqlx::query_as!(
            EdgeSchemaRow,
            r#"
SELECT
    edge_schema_id AS "edge_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    description,
    created_at,
    updated_at
FROM edges_schemas
WHERE graph_id = $1
            "#,
            graph_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(GraphSchemaModel {
            nodes: nodes.into_iter().map(NodeSchemaRow::into).collect(),
            edges: edges.into_iter().map(EdgeSchemaRow::into).collect(),
        })
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.create_graph",
        skip(self, connection, create_graph),
        err
    )]
    pub async fn create_graph(
        &self,
        connection: &mut PgConnection,
        create_graph: CreateGraphModel,
    ) -> Result<GraphModel, DatabaseError> {
        tracing::debug!(graph_id = ?create_graph.graph_id);

        let graph = sqlx::query_as!(
            GraphRow,
            r#"
INSERT INTO graphs (graph_id, name, description, is_public)
VALUES ($1, $2, $3, $4)
RETURNING
    graph_id,
    name,
    description,
    is_public,
    created_at,
    updated_at
            "#,
            create_graph.graph_id as _,
            create_graph.name,
            create_graph.description,
            create_graph.is_public
        )
        .fetch_one(connection)
        .await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.delete_graph",
        skip(self, connection, graph_id),
        err
    )]
    pub async fn delete_graph(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
    ) -> Result<(), DatabaseError> {
        tracing::debug!(graph_id = ?graph_id);

        let result = sqlx::query!(
            "DELETE FROM graphs WHERE graph_id = $1",
            graph_id as _,
        )
        .execute(connection)
        .await?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::UnexpectedState {
                reason: "Graph not found".to_string(),
            });
        }
        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.create_node_schema",
        skip(self, connection, create),
        err
    )]
    pub async fn create_node_schema(
        &self,
        connection: &mut PgConnection,
        create: CreateNodeSchemaModel,
    ) -> Result<NodeSchemaModel, DatabaseError> {
        tracing::debug!(node_schema_id = ?create.node_schema_id, graph_id = ?create.graph_id);

        let row = sqlx::query_as!(
            NodeSchemaRow,
            r#"
INSERT INTO nodes_schemas (node_schema_id, graph_id, label, key, color, description)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING
    node_schema_id AS "node_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    description,
    created_at,
    updated_at
            "#,
            create.node_schema_id as _,
            create.graph_id as _,
            create.label,
            create.key,
            create.color,
            create.description,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "graph_repository.create_edge_schema",
        skip(self, connection, create),
        err
    )]
    pub async fn create_edge_schema(
        &self,
        connection: &mut PgConnection,
        create: CreateEdgeSchemaModel,
    ) -> Result<EdgeSchemaModel, DatabaseError> {
        tracing::debug!(edge_schema_id = ?create.edge_schema_id, graph_id = ?create.graph_id);

        let row = sqlx::query_as!(
            EdgeSchemaRow,
            r#"
INSERT INTO edges_schemas (edge_schema_id, graph_id, label, key, color, description)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING
    edge_schema_id AS "edge_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    description,
    created_at,
    updated_at
            "#,
            create.edge_schema_id as _,
            create.graph_id as _,
            create.label,
            create.key,
            create.color,
            create.description,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

}

#[derive(sqlx::FromRow)]
struct GraphRow {
    graph_id: GraphIdModel,
    name: String,
    description: String,
    is_public: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<GraphRow> for GraphModel {
    fn from(row: GraphRow) -> Self {
        Self {
            graph_id: row.graph_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GraphMetadataRow {
    graph_id: GraphIdModel,
    owner_username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    description: String,
    user_role: RoleModel,
    is_public: bool,
}

impl From<GraphMetadataRow> for GraphMetadataModel {
    fn from(row: GraphMetadataRow) -> Self {
        Self {
            graph_id: row.graph_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            created_at: row.created_at,
            updated_at: row.updated_at,
            owner_username: row.owner_username,
            user_role: row.user_role,
        }
    }
}

#[derive(sqlx::FromRow)]
struct NodeSchemaRow {
    node_schema_id: NodeSchemaIdModel,
    graph_id: GraphIdModel,
    label: String,
    key: String,
    color: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<NodeSchemaRow> for NodeSchemaModel {
    fn from(row: NodeSchemaRow) -> Self {
        Self {
            node_schema_id: row.node_schema_id,
            graph_id: row.graph_id,
            label: row.label,
            key: row.key,
            color: row.color,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct EdgeSchemaRow {
    edge_schema_id: EdgeSchemaIdModel,
    graph_id: GraphIdModel,
    label: String,
    key: String,
    color: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<EdgeSchemaRow> for EdgeSchemaModel {
    fn from(row: EdgeSchemaRow) -> Self {
        Self {
            edge_schema_id: row.edge_schema_id,
            graph_id: row.graph_id,
            label: row.label,
            key: row.key,
            color: row.color,
            description: row.description,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
