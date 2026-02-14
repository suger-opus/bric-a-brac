use super::{CreatePropertySchemaDto, PropertySchemaDto};
use crate::domain::models::{CreateEdgeSchema, EdgeSchema, EdgeSchemaId, GraphId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEdgeSchemaDto {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<CreatePropertySchemaDto>,
}

impl CreateEdgeSchemaDto {
    pub fn into_domain(self) -> CreateEdgeSchema {
        CreateEdgeSchema {
            label: self.label,
            formatted_label: self.formatted_label,
            color: self.color,
            properties: self
                .properties
                .into_iter()
                .map(CreatePropertySchemaDto::into_domain)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EdgeSchemaDto {
    pub edge_schema_id: EdgeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchemaDto>,
}

impl From<EdgeSchema> for EdgeSchemaDto {
    fn from(edge_schema: EdgeSchema) -> Self {
        Self {
            edge_schema_id: edge_schema.edge_schema_id,
            graph_id: edge_schema.graph_id,
            label: edge_schema.label,
            formatted_label: edge_schema.formatted_label,
            color: edge_schema.color,
            created_at: edge_schema.created_at,
            updated_at: edge_schema.updated_at,
            properties: edge_schema
                .properties
                .into_iter()
                .map(PropertySchemaDto::from)
                .collect(),
        }
    }
}
