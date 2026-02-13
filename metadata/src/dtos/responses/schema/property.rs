use crate::{
    dtos::{PropertyMetadataDto, PropertyTypeDto},
    models::{EdgeSchemaId, NodeSchemaId, PropertySchema, PropertySchemaId},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PropertySchemaResponse {
    pub property_schema_id: PropertySchemaId,
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub formatted_label: String,
    pub property_type: PropertyTypeDto,
    pub metadata: PropertyMetadataDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PropertySchema> for PropertySchemaResponse {
    fn from(property_schema: PropertySchema) -> Self {
        Self {
            property_schema_id: property_schema.property_schema_id,
            node_schema_id: property_schema.node_schema_id,
            edge_schema_id: property_schema.edge_schema_id,
            label: property_schema.label,
            formatted_label: property_schema.formatted_label,
            property_type: property_schema.property_type.into(),
            metadata: property_schema.metadata.into(),
            created_at: property_schema.created_at,
            updated_at: property_schema.updated_at,
        }
    }
}
