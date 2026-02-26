use super::{EdgeSchemaIdDto, NodeSchemaIdDto};
use crate::{error::DtosConversionError, utils::ProtoTimestampExt};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{
    CreatePropertySchemaProto, PropertyMetadataProto, PropertySchemaProto, PropertyTypeProto,
};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::{PartialSchema, ToSchema};
use validator::Validate;

id!(PropertySchemaIdDto);

impl TryFrom<String> for PropertySchemaIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(PropertySchemaIdDto::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertySchemaDto {
    pub property_schema_id: PropertySchemaIdDto,
    pub node_schema_id: Option<NodeSchemaIdDto>,
    pub edge_schema_id: Option<EdgeSchemaIdDto>,
    pub label: String,
    pub key: String,
    pub property_type: PropertyTypeDto,
    pub metadata: PropertyMetadataDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<PropertySchemaProto> for PropertySchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: PropertySchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            property_schema_id: proto.property_schema_id.try_into()?,
            node_schema_id: proto
                .node_schema_id
                .map(|id| id.try_into())
                .transpose()?,
            edge_schema_id: proto
                .edge_schema_id
                .map(|id| id.try_into())
                .transpose()?,
            label: proto.label,
            key: proto.key,
            property_type: proto.property_type.try_into()?,
            metadata: proto.metadata.into(),
            created_at: proto.created_at.to_chrono()?,
            updated_at: proto.updated_at.to_chrono()?,
        })
    }
}

impl From<PropertySchemaDto> for PropertySchemaProto {
    fn from(dto: PropertySchemaDto) -> Self {
        Self {
            property_schema_id: dto.property_schema_id.to_string(),
            node_schema_id: dto.node_schema_id.map(|id| id.to_string()),
            edge_schema_id: dto.edge_schema_id.map(|id| id.to_string()),
            label: dto.label,
            key: dto.key,
            property_type: dto.property_type.into(),
            metadata: dto.metadata.into(),
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct PropertyMetadataDto {
    #[validate(nested)]
    pub options: Option<Vec<MetadataOptionString>>,
}

impl From<Option<PropertyMetadataProto>> for PropertyMetadataDto {
    fn from(metadata: Option<PropertyMetadataProto>) -> Self {
        metadata
            .map(|m| Self {
                options: Some(m.options.into_iter().map(From::from).collect()),
            })
            .unwrap_or_else(|| Self { options: None })
    }
}

impl From<PropertyMetadataDto> for Option<PropertyMetadataProto> {
    fn from(metadata: PropertyMetadataDto) -> Self {
        metadata.options.map(|options| PropertyMetadataProto {
            options: options.into_iter().map(From::from).collect(),
        })
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Validate,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
)]
#[display("{value}")]
#[serde(transparent)]
pub struct MetadataOptionString {
    #[validate(length(min = 1, max = 50))]
    value: String,
}

impl PartialSchema for MetadataOptionString {
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

impl ToSchema for MetadataOptionString {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("MetadataOptionString")
    }
}

impl From<String> for MetadataOptionString {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl From<MetadataOptionString> for String {
    fn from(s: MetadataOptionString) -> Self {
        s.value
    }
}

// TODO: Add metadata validation from property_type
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreatePropertySchemaDto {
    #[validate(length(min = 1, max = 25))]
    #[schema(min_length = 1, max_length = 25)]
    pub label: String,

    pub property_type: PropertyTypeDto,

    #[validate(nested)]
    pub metadata: PropertyMetadataDto,
}

impl TryFrom<CreatePropertySchemaProto> for CreatePropertySchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreatePropertySchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            label: proto.label,
            property_type: proto.property_type.try_into()?,
            metadata: proto.metadata.into(),
        })
    }
}

impl From<CreatePropertySchemaDto> for CreatePropertySchemaProto {
    fn from(dto: CreatePropertySchemaDto) -> Self {
        Self {
            label: dto.label,
            property_type: dto.property_type.into(),
            metadata: dto.metadata.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, derive_more::Display)]
pub enum PropertyTypeDto {
    Number,
    String,
    Boolean,
    Select,
}

impl TryFrom<i32> for PropertyTypeDto {
    type Error = DtosConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let proto = PropertyTypeProto::try_from(value).map_err(|_| DtosConversionError::Enum {
            name: "PropertyTypeDto".to_string(),
            value,
        })?;
        Ok(proto.into())
    }
}

impl From<PropertyTypeDto> for i32 {
    fn from(value: PropertyTypeDto) -> Self {
        let proto: PropertyTypeProto = value.into();
        proto as i32
    }
}

impl From<PropertyTypeProto> for PropertyTypeDto {
    fn from(value: PropertyTypeProto) -> Self {
        match value {
            PropertyTypeProto::Number => Self::Number,
            PropertyTypeProto::String => Self::String,
            PropertyTypeProto::Boolean => Self::Boolean,
            PropertyTypeProto::Select => Self::Select,
        }
    }
}

impl From<PropertyTypeDto> for PropertyTypeProto {
    fn from(value: PropertyTypeDto) -> Self {
        match value {
            PropertyTypeDto::Number => Self::Number,
            PropertyTypeDto::String => Self::String,
            PropertyTypeDto::Boolean => Self::Boolean,
            PropertyTypeDto::Select => Self::Select,
        }
    }
}
