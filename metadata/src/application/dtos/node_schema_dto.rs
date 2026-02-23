use super::{CreatePropertySchemaDto, PropertySchemaDto};
use crate::domain::models::{CreateNodeSchema, GraphId, NodeSchema, NodeSchemaId};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
    static ref LABEL_REGEX: Regex = Regex::new(r"^([A-Z][a-z]*_)*[A-Z][a-z]*$").unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateNodeSchemaDto {
    #[validate(length(min = 3, max = 25))]
    #[schema(min_length = 3, max_length = 25)]
    pub label: String,

    #[validate(length(min = 3, max = 25), regex(path = "*LABEL_REGEX"))]
    #[schema(
        min_length = 3,
        max_length = 25,
        pattern = "^([A-Z][a-z]*_)*[A-Z][a-z]*$"
    )]
    pub formatted_label: String,

    #[validate(regex(path = "*COLOR_REGEX"))]
    #[schema(pattern = "^#[0-9A-Fa-f]{6}$")]
    pub color: String,

    #[validate(nested)]
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

impl From<CreateNodeSchema> for CreateNodeSchemaDto {
    fn from(node_schema: CreateNodeSchema) -> Self {
        Self {
            label: node_schema.label,
            formatted_label: node_schema.formatted_label,
            color: node_schema.color,
            properties: node_schema
                .properties
                .into_iter()
                .map(CreatePropertySchemaDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
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
