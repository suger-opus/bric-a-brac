use super::{CreatePropertySchemaDto, PropertySchemaDto};
use crate::domain::models::{CreateNodeSchema, GraphId, NodeSchema, NodeSchemaId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeSchemaDto {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<CreatePropertySchemaDto>,
}

impl CreateNodeSchemaDto {
    pub fn into_domain(self) -> CreateNodeSchema {
        CreateNodeSchema {
            label: self.label,
            formatted_label: self.formatted_label,
            color: self.color,
            properties: self
                .properties
                .into_iter()
                .map(|prop| prop.into_domain())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NodeSchemaDto {
    pub node_schema_id: NodeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchemaDto>,
}

impl From<NodeSchema> for NodeSchemaDto {
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
                .map(PropertySchemaDto::from)
                .collect(),
        }
    }
}
