use super::{ColorDto, GraphIdDto, KeyDto, LabelDto};
use crate::{utils::ProtoTimestampExt, DtosConversionError};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::EdgeSchemaProto;
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
        Ok(Self::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct EdgeSchemaDto {
    pub edge_schema_id: EdgeSchemaIdDto,
    pub graph_id: GraphIdDto,
    #[validate(nested)]
    pub label: LabelDto,
    #[validate(nested)]
    pub key: KeyDto,
    #[validate(nested)]
    pub color: ColorDto,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            description: proto.description,
            created_at: proto.created_at.to_chrono()?,
            updated_at: proto.updated_at.to_chrono()?,
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
            description: dto.description,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
        }
    }
}
