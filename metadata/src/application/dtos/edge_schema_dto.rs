use super::{CreatePropertySchemaDto, PropertySchemaDto};
use crate::domain::models::{CreateEdgeSchema, EdgeSchema, EdgeSchemaId, GraphId};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateEdgeSchemaDto {
    #[validate(length(min = 1, max = 100))]
    #[schema(example = "RELATES_TO", min_length = 1, max_length = 100)]
    pub label: String,

    #[validate(length(min = 1, max = 100))]
    #[schema(example = "relates_to", min_length = 1, max_length = 100)]
    pub formatted_label: String,

    #[validate(regex(path = "*COLOR_REGEX"))]
    #[schema(example = "#10B981", pattern = "^#[0-9A-Fa-f]{6}$")]
    pub color: String,

    #[validate(nested)]
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

impl From<CreateEdgeSchema> for CreateEdgeSchemaDto {
    fn from(edge_schema: CreateEdgeSchema) -> Self {
        Self {
            label: edge_schema.label,
            formatted_label: edge_schema.formatted_label,
            color: edge_schema.color,
            properties: edge_schema
                .properties
                .into_iter()
                .map(CreatePropertySchemaDto::from)
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
