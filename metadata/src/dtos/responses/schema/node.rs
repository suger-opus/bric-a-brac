use crate::{
    dtos::PropertySchemaResponse,
    models::{GraphId, NodeSchema, NodeSchemaId},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NodeSchemaResponse {
    pub node_schema_id: NodeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchemaResponse>,
}

impl From<NodeSchema> for NodeSchemaResponse {
    fn from(node_schema: NodeSchema) -> Self {
        Self {
            node_schema_id: node_schema.node_schema_id,
            graph_id: node_schema.graph_id,
            label: node_schema.label,
            formatted_label: node_schema.formatted_label,
            color: node_schema.color,
            created_at: node_schema.created_at,
            updated_at: node_schema.updated_at,
            properties: node_schema
                .properties
                .into_iter()
                .map(PropertySchemaResponse::from)
                .collect(),
        }
    }
}
