use crate::domain::models::{
    CreatePropertySchema, EdgeSchemaId, NodeSchemaId, PropertyMetadata, PropertySchema,
    PropertySchemaId, PropertyType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PropertyMetadataDto {
    pub options: Option<Vec<String>>,
}

impl From<PropertyMetadataDto> for PropertyMetadata {
    fn from(metadata: PropertyMetadataDto) -> Self {
        Self {
            options: metadata.options,
        }
    }
}

impl From<PropertyMetadata> for PropertyMetadataDto {
    fn from(metadata: PropertyMetadata) -> Self {
        Self {
            options: metadata.options,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum PropertyTypeDto {
    Number,
    String,
    Boolean,
    Select,
}

impl From<PropertyTypeDto> for PropertyType {
    fn from(property_type: PropertyTypeDto) -> Self {
        match property_type {
            PropertyTypeDto::Number => PropertyType::Number,
            PropertyTypeDto::String => PropertyType::String,
            PropertyTypeDto::Boolean => PropertyType::Boolean,
            PropertyTypeDto::Select => PropertyType::Select,
        }
    }
}

impl From<PropertyType> for PropertyTypeDto {
    fn from(property_type: PropertyType) -> Self {
        match property_type {
            PropertyType::Number => PropertyTypeDto::Number,
            PropertyType::String => PropertyTypeDto::String,
            PropertyType::Boolean => PropertyTypeDto::Boolean,
            PropertyType::Select => PropertyTypeDto::Select,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePropertySchemaDto {
    #[schema(value_type = Option<String>)]
    pub node_schema_id: Option<NodeSchemaId>,

    #[schema(value_type = Option<String>)]
    pub edge_schema_id: Option<EdgeSchemaId>,

    #[validate(length(min = 1, max = 100))]
    #[schema(min_length = 1, max_length = 100)]
    pub label: String,

    #[validate(length(min = 1, max = 100))]
    #[schema(min_length = 1, max_length = 100)]
    pub formatted_label: String,

    pub property_type: PropertyTypeDto,

    #[validate(nested)]
    pub metadata: PropertyMetadataDto,
}

impl CreatePropertySchemaDto {
    pub fn into_domain(self) -> CreatePropertySchema {
        CreatePropertySchema {
            node_schema_id: self.node_schema_id,
            edge_schema_id: self.edge_schema_id,
            label: self.label,
            formatted_label: self.formatted_label,
            property_type: self.property_type.into(),
            metadata: self.metadata.into(),
        }
    }
}

impl From<CreatePropertySchema> for CreatePropertySchemaDto {
    fn from(property_schema: CreatePropertySchema) -> Self {
        Self {
            node_schema_id: property_schema.node_schema_id,
            edge_schema_id: property_schema.edge_schema_id,
            label: property_schema.label,
            formatted_label: property_schema.formatted_label,
            property_type: property_schema.property_type.into(),
            metadata: property_schema.metadata.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PropertySchemaDto {
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

impl From<PropertySchema> for PropertySchemaDto {
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
