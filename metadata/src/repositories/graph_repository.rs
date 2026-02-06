use crate::dtos::graph_dto::{
    ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeSchema, ReqPostProperty, ResEdgeSchema,
    ResGraphMetadata, ResGraphSchema, ResNodeSchema,
};
use crate::error::ApiError;
use crate::models::{
    access_model::Role,
    edge_schema_model::{EdgeSchema, EdgeSchemaId},
    graph_model::{Graph, GraphId, Reddit},
    node_schema_model::{NodeSchema, NodeSchemaId},
    property_model::{Property, PropertyId, PropertyMetadata, PropertyType},
    user_model::UserId,
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

    pub async fn get_all_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
    ) -> Result<Vec<ResGraphMetadata>, ApiError> {
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

    pub async fn get_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<ResGraphMetadata, ApiError> {
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

    pub async fn get_schema(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphId,
    ) -> Result<ResGraphSchema, ApiError> {
        let schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'node' AS "schema_type!:_",
    ns.node_schema_id AS "node_schema_id?:_",
    NULL::uuid AS "edge_schema_id?:_",
    ns.graph_id AS "graph_id!:_",
    ns.label AS "label!:_",
    ns.formatted_label AS "formatted_label!:_",
    ns.color AS "color!:_",
    ns.created_at AS "schema_created_at!:_",
    ns.updated_at AS "schema_updated_at!:_",
    p.property_id AS "property_id?:_",
    p.node_schema_id AS "property_node_schema_id?:_",
    p.edge_schema_id AS "property_edge_schema_id?:_",
    p.label AS "property_label?:_",
    p.formatted_label AS "property_formatted_label?:_",
    p.property_type AS "property_type?:_",
    p.metadata AS "property_metadata?:_",
    p.created_at AS "property_created_at?:_",
    p.updated_at AS "property_updated_at?:_"
FROM node_schemas ns
LEFT JOIN properties p ON ns.node_schema_id = p.node_schema_id
WHERE ns.graph_id = $1

UNION ALL

SELECT
    'edge' AS "schema_type!:_",
    NULL::uuid AS "node_schema_id?:_",
    es.edge_schema_id AS "edge_schema_id?:_",
    es.graph_id AS "graph_id!:_",
    es.label AS "label!:_",
    es.formatted_label AS "formatted_label!:_",
    es.color AS "color!:_",
    es.created_at AS "schema_created_at!:_",
    es.updated_at AS "schema_updated_at!:_",
    p.property_id AS "property_id?:_",
    p.node_schema_id AS "property_node_schema_id?:_",
    p.edge_schema_id AS "property_edge_schema_id?:_",
    p.label AS "property_label?:_",
    p.formatted_label AS "property_formatted_label?:_",
    p.property_type AS "property_type?:_",
    p.metadata AS "property_metadata?:_",
    p.created_at AS "property_created_at?:_",
    p.updated_at AS "property_updated_at?:_"
FROM edge_schemas es
LEFT JOIN properties p ON es.edge_schema_id = p.edge_schema_id
WHERE es.graph_id = $1
ORDER BY "schema_type!:_"
            "#,
            graph_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(schemas.into())
    }

    pub async fn get_node_schema(
        &self,
        connection: &mut PgConnection,
        node_schema_id: NodeSchemaId,
    ) -> Result<ResNodeSchema, ApiError> {
        let node_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'node' AS "schema_type!:_",
    ns.node_schema_id AS "node_schema_id!:_",
    NULL::uuid AS "edge_schema_id?:_",
    ns.graph_id,
    ns.label,
    ns.formatted_label,
    ns.color,
    ns.created_at AS schema_created_at,
    ns.updated_at AS schema_updated_at,
    p.property_id AS "property_id?:_",
    p.node_schema_id AS "property_node_schema_id?:_",
    p.edge_schema_id AS "property_edge_schema_id?:_",
    p.label AS "property_label?:_",
    p.formatted_label AS "property_formatted_label?:_",
    p.property_type AS "property_type?:_",
    p.metadata AS "property_metadata?:_",
    p.created_at AS "property_created_at?:_",
    p.updated_at AS "property_updated_at?:_"
FROM node_schemas ns
LEFT JOIN properties p ON ns.node_schema_id = p.node_schema_id
WHERE ns.node_schema_id = $1
            "#,
            node_schema_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(node_schemas.into())
    }

    pub async fn get_edge_schema(
        &self,
        connection: &mut PgConnection,
        edge_schema_id: EdgeSchemaId,
    ) -> Result<ResEdgeSchema, ApiError> {
        let edge_schemas = sqlx::query_as!(
            SchemaRow,
            r#"
SELECT
    'edge' AS "schema_type!:_",
    es.edge_schema_id AS "edge_schema_id!:_",
    NULL::uuid AS "node_schema_id?:_",
    es.graph_id,
    es.label,
    es.formatted_label,
    es.color,
    es.created_at AS schema_created_at,
    es.updated_at AS schema_updated_at,
    p.property_id AS "property_id?:_",
    p.node_schema_id AS "property_node_schema_id?:_",
    p.edge_schema_id AS "property_edge_schema_id?:_",
    p.label AS "property_label?:_",
    p.formatted_label AS "property_formatted_label?:_",
    p.property_type AS "property_type?:_",
    p.metadata AS "property_metadata?:_",
    p.created_at AS "property_created_at?:_",
    p.updated_at AS "property_updated_at?:_"
FROM edge_schemas es
LEFT JOIN properties p ON es.edge_schema_id = p.edge_schema_id
WHERE es.edge_schema_id = $1
            "#,
            edge_schema_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(edge_schemas.into())
    }

    pub async fn post(
        &self,
        connection: &mut PgConnection,
        new_graph: &ReqPostGraph,
    ) -> Result<Graph, ApiError> {
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
            GraphId::new() as _,
            new_graph.name,
            new_graph.description,
            new_graph.is_public
        )
        .fetch_one(connection)
        .await?;

        Ok(graph.into())
    }

    pub async fn post_node_schema(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphId,
        new_node_schema: &ReqPostNodeSchema,
    ) -> Result<NodeSchema, ApiError> {
        let node_schema = sqlx::query_as!(
            NodeSchema,
            r#"
INSERT INTO node_schemas (node_schema_id, graph_id, label, formatted_label, color)
VALUES ($1, $2, $3, $4, $5)
RETURNING
    node_schema_id,
    graph_id,
    label,
    formatted_label,
    color,
    created_at,
    updated_at
            "#,
            NodeSchemaId::new() as _,
            graph_id as _,
            new_node_schema.label,
            new_node_schema.formatted_label,
            new_node_schema.color,
        )
        .fetch_one(connection)
        .await?;

        Ok(node_schema)
    }

    pub async fn post_edge_schema(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphId,
        new_edge_schema: &ReqPostEdgeSchema,
    ) -> Result<EdgeSchema, ApiError> {
        let edge_schema = sqlx::query_as!(
            EdgeSchema,
            r#"
INSERT INTO edge_schemas (edge_schema_id, graph_id, label, formatted_label, color)
VALUES ($1, $2, $3, $4, $5)
RETURNING
    edge_schema_id,
    graph_id,
    label,
    formatted_label,
    color,
    created_at,
    updated_at
            "#,
            EdgeSchemaId::new() as _,
            graph_id as _,
            new_edge_schema.label,
            new_edge_schema.formatted_label,
            new_edge_schema.color,
        )
        .fetch_one(connection)
        .await?;

        Ok(edge_schema)
    }

    pub async fn post_properties(
        &self,
        connection: &mut PgConnection,
        node_schema_id: Option<NodeSchemaId>,
        edge_schema_id: Option<EdgeSchemaId>,
        new_properties: &Vec<ReqPostProperty>,
    ) -> Result<Vec<Property>, ApiError> {
        let mut ids = vec![];
        let mut node_schema_ids = vec![];
        let mut edge_schema_ids = vec![];
        let mut labels = vec![];
        let mut formatted_labels = vec![];
        let mut property_types = vec![];
        let mut metadatas = vec![];

        for property in new_properties {
            ids.push(PropertyId::new());
            node_schema_ids.push(node_schema_id);
            edge_schema_ids.push(edge_schema_id);
            labels.push(property.label.clone());
            formatted_labels.push(property.formatted_label.clone());
            property_types.push(&property.property_type);
            metadatas.push(Json(&property.metadata));
        }

        let properties = sqlx::query_as!(
            PropertyRow,
            r#"
INSERT INTO properties (property_id, node_schema_id, edge_schema_id, label, formatted_label, property_type, metadata)
SELECT * FROM UNNEST(
    $1::uuid[],
    $2::uuid[],
    $3::uuid[],
    $4::text[],
    $5::text[],
    $6::property_type[],
    $7::jsonb[]
) RETURNING
    property_id,
    node_schema_id AS "node_schema_id?:_",
    edge_schema_id AS "edge_schema_id?:_",
    label,
    formatted_label,
    property_type AS "property_type!:_",
    metadata AS "metadata!:_",
    created_at,
    updated_at
            "#,
            &ids as _,
            &node_schema_ids as _,
            &edge_schema_ids as _,
            &labels,
            &formatted_labels,
            &property_types as _,
            &metadatas as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(properties.into_iter().map(PropertyRow::into).collect())
    }
}

#[derive(sqlx::FromRow)]
struct GraphRow {
    graph_id: GraphId,
    name: String,
    description: String,
    is_public: bool,
    reddit: Json<Reddit>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    nb_data_nodes: i32,
    nb_data_edges: i32,
}

impl From<GraphRow> for Graph {
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
    graph_id: GraphId,
    owner_username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    description: String,
    reddit: Json<Reddit>,
    user_role: Role,
    is_public: bool,
    is_bookmarked_by_user: bool,
    is_cheered_by_user: bool,
    nb_data_nodes: i32,
    nb_data_edges: i32,
    nb_bookmarks: i32,
    nb_cheers: i32,
}

impl From<GraphMetadataRow> for ResGraphMetadata {
    fn from(row: GraphMetadataRow) -> Self {
        Self {
            graph: Graph {
                graph_id: row.graph_id,
                name: row.name,
                description: row.description,
                is_public: row.is_public,
                reddit: row.reddit.0,
                created_at: row.created_at,
                updated_at: row.updated_at,
                nb_data_nodes: row.nb_data_nodes as u32,
                nb_data_edges: row.nb_data_edges as u32,
            },
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
struct PropertyRow {
    property_id: PropertyId,
    node_schema_id: Option<NodeSchemaId>,
    edge_schema_id: Option<EdgeSchemaId>,
    label: String,
    formatted_label: String,
    property_type: PropertyType,
    metadata: Json<PropertyMetadata>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<PropertyRow> for Property {
    fn from(row: PropertyRow) -> Self {
        Property {
            property_id: row.property_id,
            node_schema_id: row.node_schema_id,
            edge_schema_id: row.edge_schema_id,
            label: row.label,
            formatted_label: row.formatted_label,
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
    node_schema_id: Option<NodeSchemaId>,
    edge_schema_id: Option<EdgeSchemaId>,
    graph_id: GraphId,
    label: String,
    formatted_label: String,
    color: String,
    schema_created_at: DateTime<Utc>,
    schema_updated_at: DateTime<Utc>,
    property_id: Option<PropertyId>,
    property_node_schema_id: Option<NodeSchemaId>,
    property_edge_schema_id: Option<EdgeSchemaId>,
    property_label: Option<String>,
    property_formatted_label: Option<String>,
    property_type: Option<PropertyType>,
    property_metadata: Option<Json<PropertyMetadata>>,
    property_created_at: Option<DateTime<Utc>>,
    property_updated_at: Option<DateTime<Utc>>,
}

impl SchemaRow {
    fn extract_property(&self) -> Option<Property> {
        match (
            self.property_id,
            &self.property_label,
            &self.property_formatted_label,
            &self.property_type,
            &self.property_metadata,
            self.property_created_at,
            self.property_updated_at,
        ) {
            (
                Some(property_id),
                Some(label),
                Some(formatted_label),
                Some(property_type),
                Some(metadata),
                Some(created_at),
                Some(updated_at),
            ) => Some(Property {
                property_id,
                node_schema_id: self.property_node_schema_id,
                edge_schema_id: self.property_edge_schema_id,
                label: label.clone(),
                formatted_label: formatted_label.clone(),
                property_type: property_type.clone(),
                metadata: metadata.0.clone(),
                created_at,
                updated_at,
            }),
            _ => None,
        }
    }
}

impl From<Vec<SchemaRow>> for ResGraphSchema {
    fn from(schemas: Vec<SchemaRow>) -> Self {
        let mut node_schemas_map: HashMap<NodeSchemaId, Vec<SchemaRow>> = HashMap::new();
        let mut edge_schemas_map: HashMap<EdgeSchemaId, Vec<SchemaRow>> = HashMap::new();

        for row in schemas {
            if row.schema_type == "node" {
                if let Some(node_schema_id) = row.node_schema_id {
                    node_schemas_map
                        .entry(node_schema_id)
                        .or_default()
                        .push(row);
                }
            } else {
                if let Some(edge_schema_id) = row.edge_schema_id {
                    edge_schemas_map
                        .entry(edge_schema_id)
                        .or_default()
                        .push(row);
                }
            }
        }

        ResGraphSchema {
            node_schemas: node_schemas_map
                .into_iter()
                .map(|(_, schemas)| schemas.into())
                .collect(),
            edge_schemas: edge_schemas_map
                .into_iter()
                .map(|(_, schemas)| schemas.into())
                .collect(),
        }
    }
}

impl From<Vec<SchemaRow>> for ResNodeSchema {
    fn from(schemas: Vec<SchemaRow>) -> Self {
        // Default to empty schema if not found, as repository guarantees non-empty vector
        let first_row = if let Some(row) = schemas.first() {
            row
        } else {
            &SchemaRow::default()
        };

        let node_schema = NodeSchema {
            node_schema_id: first_row.node_schema_id.unwrap_or_default(),
            graph_id: first_row.graph_id,
            label: first_row.label.clone(),
            formatted_label: first_row.formatted_label.clone(),
            color: first_row.color.clone(),
            created_at: first_row.schema_created_at,
            updated_at: first_row.schema_updated_at,
        };

        let properties = schemas
            .iter()
            .filter_map(|row| row.extract_property())
            .collect();

        ResNodeSchema {
            node_schema,
            properties,
        }
    }
}

impl From<Vec<SchemaRow>> for ResEdgeSchema {
    fn from(schemas: Vec<SchemaRow>) -> Self {
        // Default to empty schema if not found, as repository guarantees non-empty vector
        let first_row = if let Some(row) = schemas.first() {
            row
        } else {
            &SchemaRow::default()
        };

        let edge_schema = EdgeSchema {
            edge_schema_id: first_row.edge_schema_id.unwrap_or_default(),
            graph_id: first_row.graph_id,
            label: first_row.label.clone(),
            formatted_label: first_row.formatted_label.clone(),
            color: first_row.color.clone(),
            created_at: first_row.schema_created_at,
            updated_at: first_row.schema_updated_at,
        };

        let properties = schemas
            .iter()
            .filter_map(|row| row.extract_property())
            .collect();

        ResEdgeSchema {
            edge_schema,
            properties,
        }
    }
}
