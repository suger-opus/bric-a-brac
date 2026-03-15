use super::{ColorDto, CreatePropertySchemaDto, GraphIdDto, KeyDto, LabelDto, PropertySchemaDto};
use crate::{utils::ProtoTimestampExt, DtosConversionError};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{CreateEdgeSchemaProto, EdgeSchemaProto};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

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
    pub label: LabelDto,
    pub key: KeyDto,
    pub color: ColorDto,
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
            label: proto.label.into(),
            key: proto.key.into(),
            color: proto.color.into(),
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
            label: dto.label.into(),
            key: dto.key.into(),
            color: dto.color.into(),
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
            properties: dto.properties.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateEdgeSchemaDto {
    #[validate(nested)]
    pub label: LabelDto,

    #[validate(nested)]
    pub color: ColorDto,

    #[validate(nested)]
    pub properties: Vec<CreatePropertySchemaDto>,
}

impl TryFrom<CreateEdgeSchemaProto> for CreateEdgeSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateEdgeSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            label: proto.label.into(),
            color: proto.color.into(),
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
            label: dto.label.into(),
            color: dto.color.into(),
            properties: dto.properties.into_iter().map(From::from).collect(),
        }
    }
}
