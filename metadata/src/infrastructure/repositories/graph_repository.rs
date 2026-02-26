use crate::{
    domain::models::{
        CreateEdgeSchemaModel, CreateGraphModel, CreateNodeSchemaModel, CreatePropertySchemaModel,
        EdgeSchemaIdModel, EdgeSchemaModel, GraphIdModel, GraphMetadataModel, GraphModel,
        GraphSchemaModel, NodeSchemaIdModel, NodeSchemaModel, PropertyMetadataModel,
        PropertySchemaIdModel, PropertySchemaModel, PropertyTypeModel, RedditModel, RoleModel,
        UserIdModel,
    },
    presentation::errors::DatabaseError,
};
use chrono::{DateTime, Utc};
use sqlx::{types::Json, PgConnection};
use std::collections::HashMap;

#[derive(Clone)]
pub struct GraphRepository;

impl GraphRepository {
    pub fn new() -> Self {
        GraphRepository
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_all_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserIdModel,
    ) -> Result<Vec<GraphMetadataModel>, DatabaseError> {
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
    g.is_public,
    g.reddit AS "reddit!:_",
    EXISTS(SELECT 1 FROM bookmarks b WHERE b.user_id = $1 AND b.graph_id = g.graph_id) AS "is_bookmarked_by_user!",
    EXISTS(SELECT 1 FROM cheers c WHERE c.user_id = $1 AND c.graph_id = g.graph_id) AS "is_cheered_by_user!",
    g.nb_data_nodes,
    g.nb_data_edges,
    (SELECT COUNT(*)::INT FROM bookmarks b WHERE b.graph_id = g.graph_id) AS "nb_bookmarks!",
    (SELECT COUNT(*)::INT FROM cheers c WHERE c.graph_id = g.graph_id) AS "nb_cheers!"
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

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_metadata(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        user_id: UserIdModel,
    ) -> Result<GraphMetadataModel, DatabaseError> {
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
    g.is_public,
    g.reddit AS "reddit!:_",
    EXISTS(SELECT 1 FROM bookmarks b WHERE b.user_id = $1 AND b.graph_id = g.graph_id) AS "is_bookmarked_by_user!",
    EXISTS(SELECT 1 FROM cheers c WHERE c.user_id = $1 AND c.graph_id = g.graph_id) AS "is_cheered_by_user!",
    g.nb_data_nodes,
    g.nb_data_edges,
    (SELECT COUNT(*)::INT FROM bookmarks b WHERE b.graph_id = g.graph_id) AS "nb_bookmarks!",
    (SELECT COUNT(*)::INT FROM cheers c WHERE c.graph_id = g.graph_id) AS "nb_cheers!"
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

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_schema(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
    ) -> Result<GraphSchemaModel, DatabaseError> {
        let schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'node' AS "schema_type!:_",
    ns.node_schema_id AS "node_schema_id?:_",
    NULL::uuid AS "edge_schema_id?:_",
    ns.graph_id AS "graph_id!:_",
    ns.label AS "label!:_",
    ns.key AS "key!:_",
    ns.color AS "color!:_",
    ns.created_at AS "schema_created_at!:_",
    ns.updated_at AS "schema_updated_at!:_",
    p.property_schema_id AS "property_schema_id?:_",
    p.node_schema_id AS "property_node_schema_id?:_",
    p.edge_schema_id AS "property_edge_schema_id?:_",
    p.label AS "property_label?:_",
    p.key AS "property_key?:_",
    p.property_type AS "property_type?:_",
    p.metadata AS "property_metadata?:_",
    p.created_at AS "property_created_at?:_",
    p.updated_at AS "property_updated_at?:_"
FROM nodes_schemas ns
LEFT JOIN properties_schemas p ON ns.node_schema_id = p.node_schema_id
WHERE ns.graph_id = $1

UNION ALL

SELECT
    'edge' AS "schema_type!:_",
    NULL::uuid AS "node_schema_id?:_",
    es.edge_schema_id AS "edge_schema_id?:_",
    es.graph_id AS "graph_id!:_",
    es.label AS "label!:_",
    es.key AS "key!:_",
    es.color AS "color!:_",
    es.created_at AS "schema_created_at!:_",
    es.updated_at AS "schema_updated_at!:_",
    ps.property_schema_id AS "property_schema_id?:_",
    ps.node_schema_id AS "property_node_schema_id?:_",
    ps.edge_schema_id AS "property_edge_schema_id?:_",
    ps.label AS "property_label?:_",
    ps.key AS "property_key?:_",
    ps.property_type AS "property_type?:_",
    ps.metadata AS "property_metadata?:_",
    ps.created_at AS "property_created_at?:_",
    ps.updated_at AS "property_updated_at?:_"
FROM edges_schemas es
LEFT JOIN properties_schemas ps ON es.edge_schema_id = ps.edge_schema_id
WHERE es.graph_id = $1
ORDER BY "schema_type!:_"
            "#,
            graph_id as _,
        )
        .fetch_all(connection)
        .await?;

        schemas.try_into()
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_node_schema_by_id(
        &self,
        connection: &mut PgConnection,
        node_schema_id: NodeSchemaIdModel,
    ) -> Result<NodeSchemaModel, DatabaseError> {
        let nodes_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'node' AS "schema_type!:_",
    ns.node_schema_id AS "node_schema_id!:_",
    NULL::uuid AS "edge_schema_id?:_",
    ns.graph_id,
    ns.label,
    ns.key,
    ns.color,
    ns.created_at AS schema_created_at,
    ns.updated_at AS schema_updated_at,
    ps.property_schema_id AS "property_schema_id?:_",
    ps.node_schema_id AS "property_node_schema_id?:_",
    ps.edge_schema_id AS "property_edge_schema_id?:_",
    ps.label AS "property_label?:_",
    ps.key AS "property_key?:_",
    ps.property_type AS "property_type?:_",
    ps.metadata AS "property_metadata?:_",
    ps.created_at AS "property_created_at?:_",
    ps.updated_at AS "property_updated_at?:_"
FROM nodes_schemas ns
LEFT JOIN properties_schemas ps ON ns.node_schema_id = ps.node_schema_id
WHERE ns.node_schema_id = $1
            "#,
            node_schema_id as _,
        )
        .fetch_all(connection)
        .await?;

        nodes_schemas.try_into()
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_node_schema_by_key(
        &self,
        connection: &mut PgConnection,
        node_schema_key: String,
    ) -> Result<NodeSchemaModel, DatabaseError> {
        let nodes_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'node' AS "schema_type!:_",
    ns.node_schema_id AS "node_schema_id!:_",
    NULL::uuid AS "edge_schema_id?:_",
    ns.graph_id,
    ns.label,
    ns.key,
    ns.color,
    ns.created_at AS schema_created_at,
    ns.updated_at AS schema_updated_at,
    ps.property_schema_id AS "property_schema_id?:_",
    ps.node_schema_id AS "property_node_schema_id?:_",
    ps.edge_schema_id AS "property_edge_schema_id?:_",
    ps.label AS "property_label?:_",
    ps.key AS "property_key?:_",
    ps.property_type AS "property_type?:_",
    ps.metadata AS "property_metadata?:_",
    ps.created_at AS "property_created_at?:_",
    ps.updated_at AS "property_updated_at?:_"
FROM nodes_schemas ns
LEFT JOIN properties_schemas ps ON ns.node_schema_id = ps.node_schema_id
WHERE ns.key = $1
            "#,
            node_schema_key as _,
        )
        .fetch_all(connection)
        .await?;

        nodes_schemas.try_into()
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_edge_schema_by_id(
        &self,
        connection: &mut PgConnection,
        edge_schema_id: EdgeSchemaIdModel,
    ) -> Result<EdgeSchemaModel, DatabaseError> {
        let edges_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'edge' AS "schema_type!:_",
    es.edge_schema_id AS "edge_schema_id!:_",
    NULL::uuid AS "node_schema_id?:_",
    es.graph_id,
    es.label,
    es.key,
    es.color,
    es.created_at AS schema_created_at,
    es.updated_at AS schema_updated_at,
    ps.property_schema_id AS "property_schema_id?:_",
    ps.node_schema_id AS "property_node_schema_id?:_",
    ps.edge_schema_id AS "property_edge_schema_id?:_",
    ps.label AS "property_label?:_",
    ps.key AS "property_key?:_",
    ps.property_type AS "property_type?:_",
    ps.metadata AS "property_metadata?:_",
    ps.created_at AS "property_created_at?:_",
    ps.updated_at AS "property_updated_at?:_"
FROM edges_schemas es
LEFT JOIN properties_schemas ps ON es.edge_schema_id = ps.edge_schema_id
WHERE es.edge_schema_id = $1
            "#,
            edge_schema_id as _,
        )
        .fetch_all(connection)
        .await?;

        edges_schemas.try_into()
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get_edge_schema_by_key(
        &self,
        connection: &mut PgConnection,
        edge_schema_key: String,
    ) -> Result<EdgeSchemaModel, DatabaseError> {
        let edges_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'edge' AS "schema_type!:_",
    es.edge_schema_id AS "edge_schema_id!:_",
    NULL::uuid AS "node_schema_id?:_",
    es.graph_id,
    es.label,
    es.key,
    es.color,
    es.created_at AS schema_created_at,
    es.updated_at AS schema_updated_at,
    ps.property_schema_id AS "property_schema_id?:_",
    ps.node_schema_id AS "property_node_schema_id?:_",
    ps.edge_schema_id AS "property_edge_schema_id?:_",
    ps.label AS "property_label?:_",
    ps.key AS "property_key?:_",
    ps.property_type AS "property_type?:_",
    ps.metadata AS "property_metadata?:_",
    ps.created_at AS "property_created_at?:_",
    ps.updated_at AS "property_updated_at?:_"
FROM edges_schemas es
LEFT JOIN properties_schemas ps ON es.edge_schema_id = ps.edge_schema_id
WHERE es.key = $1
            "#,
            edge_schema_key as _,
        )
        .fetch_all(connection)
        .await?;

        edges_schemas.try_into()
    }

    #[tracing::instrument(level = "debug", skip(self, connection, create_graph))]
    pub async fn create_graph(
        &self,
        connection: &mut PgConnection,
        create_graph: CreateGraphModel,
    ) -> Result<GraphModel, DatabaseError> {
        tracing::debug!(create_graph_name = ?create_graph.name);

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
    reddit AS "reddit!:_",
    created_at,
    updated_at,
    nb_data_nodes,
    nb_data_edges
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

    #[tracing::instrument(level = "debug", skip(self, connection, nodes_schemas))]
    pub async fn create_nodes_schemas(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        nodes_schemas: Vec<CreateNodeSchemaModel>,
    ) -> Result<Vec<NodeSchemaModel>, DatabaseError> {
        tracing::debug!(nodes_schemas_len = ?nodes_schemas.len());

        let mut node_schema_ids = vec![];
        let mut graph_ids = vec![];
        let mut labels = vec![];
        let mut keys = vec![];
        let mut colors = vec![];

        for schema in nodes_schemas {
            node_schema_ids.push(schema.node_schema_id);
            graph_ids.push(graph_id);
            labels.push(schema.label.clone());
            keys.push(schema.key.clone());
            colors.push(schema.color.clone());
        }

        let nodes_schemas = sqlx::query_as!(
            NodeSchemaRow,
            r#"
INSERT INTO nodes_schemas (node_schema_id, graph_id, label, key, color)
SELECT * FROM UNNEST(
    $1::uuid[],
    $2::uuid[],
    $3::text[],
    $4::text[],
    $5::text[]
) RETURNING
    node_schema_id AS "node_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    created_at,
    updated_at
            "#,
            &node_schema_ids as _,
            &graph_ids as _,
            &labels,
            &keys,
            &colors,
        )
        .fetch_all(connection)
        .await?;

        Ok(nodes_schemas.into_iter().map(NodeSchemaRow::into).collect())
    }

    #[tracing::instrument(level = "debug", skip(self, connection, edges_schemas))]
    pub async fn create_edges_schemas(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        edges_schemas: Vec<CreateEdgeSchemaModel>,
    ) -> Result<Vec<EdgeSchemaModel>, DatabaseError> {
        tracing::debug!(edges_schemas_len = ?edges_schemas.len());

        let mut edge_schema_ids = vec![];
        let mut graph_ids = vec![];
        let mut labels = vec![];
        let mut keys = vec![];
        let mut colors = vec![];

        for schema in edges_schemas {
            edge_schema_ids.push(schema.edge_schema_id);
            graph_ids.push(graph_id);
            labels.push(schema.label.clone());
            keys.push(schema.key.clone());
            colors.push(schema.color.clone());
        }

        let edges_schemas = sqlx::query_as!(
            EdgeSchemaRow,
            r#"
INSERT INTO edges_schemas (edge_schema_id, graph_id, label, key, color)
SELECT * FROM UNNEST(
    $1::uuid[],
    $2::uuid[],
    $3::text[],
    $4::text[],
    $5::text[]
) RETURNING
    edge_schema_id AS "edge_schema_id!:_",
    graph_id AS "graph_id!:_",
    label,
    key,
    color,
    created_at,
    updated_at
            "#,
            &edge_schema_ids as _,
            &graph_ids as _,
            &labels,
            &keys,
            &colors,
        )
        .fetch_all(connection)
        .await?;

        Ok(edges_schemas.into_iter().map(EdgeSchemaRow::into).collect())
    }

    #[tracing::instrument(level = "debug", skip(self, connection, create_properties))]
    pub async fn create_properties(
        &self,
        connection: &mut PgConnection,
        create_properties: Vec<CreatePropertySchemaModel>,
    ) -> Result<Vec<PropertySchemaModel>, DatabaseError> {
        tracing::debug!(create_properties_len = ?create_properties.len());

        let mut property_schema_ids = vec![];
        let mut node_schema_ids = vec![];
        let mut edge_schema_ids = vec![];
        let mut labels = vec![];
        let mut keys = vec![];
        let mut property_types = vec![];
        let mut metadatas = vec![];

        for property in create_properties {
            property_schema_ids.push(property.property_schema_id);
            node_schema_ids.push(property.node_schema_id);
            edge_schema_ids.push(property.edge_schema_id);
            labels.push(property.label.clone());
            keys.push(property.key.clone());
            property_types.push(property.property_type);
            metadatas.push(Json(property.metadata));
        }

        let properties = sqlx::query_as!(
            PropertySchemaRow,
            r#"
INSERT INTO properties_schemas (property_schema_id, node_schema_id, edge_schema_id, label, key, property_type, metadata)
SELECT * FROM UNNEST(
    $1::uuid[],
    $2::uuid[],
    $3::uuid[],
    $4::text[],
    $5::text[],
    $6::property_type[],
    $7::jsonb[]
) RETURNING
    property_schema_id,
    node_schema_id AS "node_schema_id?:_",
    edge_schema_id AS "edge_schema_id?:_",
    label,
    key,
    property_type AS "property_type!:_",
    metadata AS "metadata!:_",
    created_at,
    updated_at
            "#,
            &property_schema_ids as _,
            &node_schema_ids as _,
            &edge_schema_ids as _,
            &labels,
            &keys,
            &property_types as _,
            &metadatas as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(properties
            .into_iter()
            .map(PropertySchemaRow::into)
            .collect())
    }
}

#[derive(sqlx::FromRow)]
struct GraphRow {
    graph_id: GraphIdModel,
    name: String,
    description: String,
    is_public: bool,
    reddit: Json<RedditModel>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    nb_data_nodes: i32,
    nb_data_edges: i32,
}

impl From<GraphRow> for GraphModel {
    fn from(row: GraphRow) -> Self {
        Self {
            graph_id: row.graph_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            reddit: row.reddit.0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            nb_data_nodes: row.nb_data_nodes as u32,
            nb_data_edges: row.nb_data_edges as u32,
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
    reddit: Json<RedditModel>,
    user_role: RoleModel,
    is_public: bool,
    is_bookmarked_by_user: bool,
    is_cheered_by_user: bool,
    nb_data_nodes: i32,
    nb_data_edges: i32,
    nb_bookmarks: i32,
    nb_cheers: i32,
}

impl From<GraphMetadataRow> for GraphMetadataModel {
    fn from(row: GraphMetadataRow) -> Self {
        Self {
            graph_id: row.graph_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            reddit: row.reddit.0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            nb_data_nodes: row.nb_data_nodes as u32,
            nb_data_edges: row.nb_data_edges as u32,
            owner_username: row.owner_username,
            user_role: row.user_role,
            is_bookmarked_by_user: row.is_bookmarked_by_user,
            is_cheered_by_user: row.is_cheered_by_user,
            nb_bookmarks: row.nb_bookmarks as u32,
            nb_cheers: row.nb_cheers as u32,
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
            created_at: row.created_at,
            updated_at: row.updated_at,
            properties: vec![],
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
            created_at: row.created_at,
            updated_at: row.updated_at,
            properties: vec![],
        }
    }
}

#[derive(sqlx::FromRow)]
struct PropertySchemaRow {
    property_schema_id: PropertySchemaIdModel,
    node_schema_id: Option<NodeSchemaIdModel>,
    edge_schema_id: Option<EdgeSchemaIdModel>,
    label: String,
    key: String,
    property_type: PropertyTypeModel,
    metadata: Json<PropertyMetadataModel>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PropertySchemaRow> for PropertySchemaModel {
    fn from(row: PropertySchemaRow) -> Self {
        PropertySchemaModel {
            property_schema_id: row.property_schema_id,
            node_schema_id: row.node_schema_id,
            edge_schema_id: row.edge_schema_id,
            label: row.label,
            key: row.key,
            property_type: row.property_type,
            metadata: row.metadata.0,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Default, sqlx::FromRow)]
struct SchemaRow {
    schema_type: String,
    node_schema_id: Option<NodeSchemaIdModel>,
    edge_schema_id: Option<EdgeSchemaIdModel>,
    graph_id: GraphIdModel,
    label: String,
    key: String,
    color: String,
    schema_created_at: DateTime<Utc>,
    schema_updated_at: DateTime<Utc>,
    property_schema_id: Option<PropertySchemaIdModel>,
    property_node_schema_id: Option<NodeSchemaIdModel>,
    property_edge_schema_id: Option<EdgeSchemaIdModel>,
    property_label: Option<String>,
    property_key: Option<String>,
    property_type: Option<PropertyTypeModel>,
    property_metadata: Option<Json<PropertyMetadataModel>>,
    property_created_at: Option<DateTime<Utc>>,
    property_updated_at: Option<DateTime<Utc>>,
}

impl SchemaRow {
    fn extract_property(&self) -> Option<PropertySchemaModel> {
        match (
            self.property_schema_id,
            &self.property_label,
            &self.property_key,
            &self.property_type,
            &self.property_metadata,
            self.property_created_at,
            self.property_updated_at,
        ) {
            (
                Some(property_schema_id),
                Some(label),
                Some(key),
                Some(property_type),
                Some(metadata),
                Some(created_at),
                Some(updated_at),
            ) => Some(PropertySchemaModel {
                property_schema_id,
                node_schema_id: self.property_node_schema_id,
                edge_schema_id: self.property_edge_schema_id,
                label: label.clone(),
                key: key.clone(),
                property_type: property_type.clone(),
                metadata: metadata.0.clone(),
                created_at,
                updated_at,
            }),
            _ => None,
        }
    }
}

impl TryFrom<Vec<SchemaRow>> for GraphSchemaModel {
    type Error = DatabaseError;

    fn try_from(schemas: Vec<SchemaRow>) -> Result<Self, Self::Error> {
        let mut nodes_schemas_map: HashMap<NodeSchemaIdModel, Vec<SchemaRow>> = HashMap::new();
        let mut edges_schemas_map: HashMap<EdgeSchemaIdModel, Vec<SchemaRow>> = HashMap::new();

        for row in schemas {
            if row.schema_type == "node" {
                if let Some(node_schema_id) = row.node_schema_id {
                    nodes_schemas_map
                        .entry(node_schema_id)
                        .or_default()
                        .push(row);
                }
            } else {
                if let Some(edge_schema_id) = row.edge_schema_id {
                    edges_schemas_map
                        .entry(edge_schema_id)
                        .or_default()
                        .push(row);
                }
            }
        }

        Ok(GraphSchemaModel {
            nodes: nodes_schemas_map
                .into_iter()
                .map(|(_, schemas)| schemas.try_into())
                .collect::<Result<_, Self::Error>>()?,
            edges: edges_schemas_map
                .into_iter()
                .map(|(_, schemas)| schemas.try_into())
                .collect::<Result<_, Self::Error>>()?,
        })
    }
}

impl TryFrom<Vec<SchemaRow>> for NodeSchemaModel {
    type Error = DatabaseError;

    fn try_from(schemas: Vec<SchemaRow>) -> Result<Self, Self::Error> {
        let first_row = if let Some(row) = schemas.first() {
            Ok(row)
        } else {
            Err(DatabaseError::UnexpectedState {
                reason: "No rows returned for node schema".to_string(),
            })
        }?;

        let node_schema_id =
            first_row
                .node_schema_id
                .ok_or_else(|| DatabaseError::UnexpectedState {
                    reason: "Missing node_schema_id in node schema row".to_string(),
                })?;

        let properties = schemas
            .iter()
            .filter_map(|row| row.extract_property())
            .collect();

        Ok(NodeSchemaModel {
            node_schema_id: node_schema_id,
            graph_id: first_row.graph_id,
            label: first_row.label.clone(),
            key: first_row.key.clone(),
            color: first_row.color.clone(),
            created_at: first_row.schema_created_at,
            updated_at: first_row.schema_updated_at,
            properties,
        })
    }
}

impl TryFrom<Vec<SchemaRow>> for EdgeSchemaModel {
    type Error = DatabaseError;

    fn try_from(schemas: Vec<SchemaRow>) -> Result<Self, Self::Error> {
        let first_row = if let Some(row) = schemas.first() {
            Ok(row)
        } else {
            Err(DatabaseError::UnexpectedState {
                reason: "No rows returned for edge schema".to_string(),
            })
        }?;

        let edge_schema_id =
            first_row
                .edge_schema_id
                .ok_or_else(|| DatabaseError::UnexpectedState {
                    reason: "Missing edge_schema_id in edge schema row".to_string(),
                })?;

        let properties = schemas
            .iter()
            .filter_map(|row| row.extract_property())
            .collect();

        Ok(EdgeSchemaModel {
            edge_schema_id: edge_schema_id,
            graph_id: first_row.graph_id,
            label: first_row.label.clone(),
            key: first_row.key.clone(),
            color: first_row.color.clone(),
            created_at: first_row.schema_created_at,
            updated_at: first_row.schema_updated_at,
            properties,
        })
    }
}
