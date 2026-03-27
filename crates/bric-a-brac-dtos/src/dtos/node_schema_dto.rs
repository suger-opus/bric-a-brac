use super::{ColorDto, GraphIdDto, KeyDto, LabelDto};
use crate::{utils::ProtoTimestampExt, DtosConversionError};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::NodeSchemaProto;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

id!(NodeSchemaIdDto);

impl TryFrom<String> for NodeSchemaIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct NodeSchemaDto {
    pub node_schema_id: NodeSchemaIdDto,
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

impl TryFrom<NodeSchemaProto> for NodeSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: NodeSchemaProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_schema_id: proto.node_schema_id.try_into()?,
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

impl From<NodeSchemaDto> for NodeSchemaProto {
    fn from(dto: NodeSchemaDto) -> Self {
        Self {
            node_schema_id: dto.node_schema_id.to_string(),
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
