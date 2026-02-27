use super::{CreatePropertySchemaDto, GraphIdDto, PropertySchemaDto};
use crate::{utils::ProtoTimestampExt, DtosConversionError};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{CreateEdgeSchemaProto, EdgeSchemaProto};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use prost_types::Timestamp;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
}

id!(EdgeSchemaIdDto);

impl TryFrom<String> for EdgeSchemaIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(EdgeSchemaIdDto::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EdgeSchemaDto {
    pub edge_schema_id: EdgeSchemaIdDto,
    pub graph_id: GraphIdDto,
    pub label: String,
    pub key: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchemaDto>,
}

impl TryFrom<EdgeSchemaProto> for EdgeSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: EdgeSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            edge_schema_id: proto.edge_schema_id.try_into()?,
            graph_id: proto.graph_id.try_into()?,
            label: proto.label,
            key: proto.key,
            color: proto.color,
            created_at: proto.created_at.to_chrono()?,
            updated_at: proto.updated_at.to_chrono()?,
            properties: proto
                .properties
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<EdgeSchemaDto> for EdgeSchemaProto {
    fn from(dto: EdgeSchemaDto) -> Self {
        Self {
            edge_schema_id: dto.edge_schema_id.to_string(),
            graph_id: dto.graph_id.to_string(),
            label: dto.label,
            key: dto.key,
            color: dto.color,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
            properties: dto.properties.into_iter().map(From::from).collect(),
        }
    }
}

// TODO: create a struct for labels and colors
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateEdgeSchemaDto {
    #[validate(length(min = 1, max = 25))]
    #[schema(min_length = 1, max_length = 25)]
    pub label: String,

    #[validate(regex(path = "*COLOR_REGEX"))]
    #[schema(pattern = "^#[0-9A-Fa-f]{6}$")]
    pub color: String,

    #[validate(nested)]
    pub properties: Vec<CreatePropertySchemaDto>,
}

impl TryFrom<CreateEdgeSchemaProto> for CreateEdgeSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateEdgeSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            label: proto.label,
            color: proto.color,
            properties: proto
                .properties
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<CreateEdgeSchemaDto> for CreateEdgeSchemaProto {
    fn from(dto: CreateEdgeSchemaDto) -> Self {
        Self {
            label: dto.label,
            color: dto.color,
            properties: dto.properties.into_iter().map(From::from).collect(),
        }
    }
}
