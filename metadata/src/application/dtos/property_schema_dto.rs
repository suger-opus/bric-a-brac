use crate::domain::{models::{
    CreatePropertySchema, EdgeSchemaId, NodeSchemaId, PropertyMetadata, PropertySchema,
    PropertySchemaId, PropertyType,
}, utils::generate_key};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{PartialSchema, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct OptionString {
    #[validate(length(min = 1, max = 50))]
    value: String,
}

impl PartialSchema for OptionString {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::new(
                utoipa::openapi::schema::Type::String,
            ))
            .min_length(Some(1))
            .max_length(Some(50))
            .build()
            .into()
    }
}

impl ToSchema for OptionString {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("OptionString")
    }
}

impl From<String> for OptionString {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl From<OptionString> for String {
    fn from(s: OptionString) -> Self {
        s.value
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PropertyMetadataDto {
    pub options: Option<Vec<String>>,
}

impl From<PropertyMetadata> for PropertyMetadataDto {
    fn from(metadata: PropertyMetadata) -> Self {
        Self {
            options: metadata.options,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePropertyMetadataDto {
    #[validate(nested)]
    pub options: Option<Vec<OptionString>>,
}

impl From<CreatePropertyMetadataDto> for PropertyMetadata {
    fn from(metadata: CreatePropertyMetadataDto) -> Self {
        Self {
            options: metadata
                .options
                .map(|v| v.into_iter().map(String::from).collect()),
        }
    }
}

impl From<PropertyMetadata> for CreatePropertyMetadataDto {
    fn from(metadata: PropertyMetadata) -> Self {
        Self {
            options: metadata
                .options
                .map(|v| v.into_iter().map(OptionString::from).collect()),
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
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,

    #[validate(length(min = 1, max = 25))]
    #[schema(min_length = 1, max_length = 25)]
    pub label: String,

    pub property_type: PropertyTypeDto,

    #[validate(nested)]
    pub metadata: CreatePropertyMetadataDto,
}

impl CreatePropertySchemaDto {
    pub fn into_domain(self) -> CreatePropertySchema {
        CreatePropertySchema {
            node_schema_id: self.node_schema_id,
            edge_schema_id: self.edge_schema_id,
            label: self.label,
            key: generate_key(),
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
            property_type: property_schema.property_type.into(),
            metadata: property_schema.metadata.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PropertySchemaDto {
    pub property_schema_id: PropertySchemaId,
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub key: String,
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
            key: property_schema.key,
            property_type: property_schema.property_type.into(),
            metadata: property_schema.metadata.into(),
            created_at: property_schema.created_at,
            updated_at: property_schema.updated_at,
        }
    }
}
