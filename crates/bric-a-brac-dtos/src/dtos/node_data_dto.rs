use super::{KeyDto, PropertiesDataDto};
use crate::DtosConversionError;
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{CreateNodeDataProto, NodeDataProto};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

id!(NodeDataIdDto);

impl TryFrom<String> for NodeDataIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(NodeDataIdDto::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeDataDto {
    pub node_data_id: NodeDataIdDto,
    pub key: KeyDto,
    pub properties: PropertiesDataDto,
}

impl TryFrom<NodeDataProto> for NodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: NodeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            key: proto.key.into(),
            properties: proto.properties.try_into()?,
        })
    }
}

impl From<NodeDataDto> for NodeDataProto {
    fn from(dto: NodeDataDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.to_string(),
            key: dto.key.into(),
            properties: dto.properties.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct CreateNodeDataDto {
    pub node_data_id: NodeDataIdDto,
    pub key: KeyDto,
    pub properties: PropertiesDataDto,
}

impl TryFrom<CreateNodeDataProto> for CreateNodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateNodeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            key: proto.key.into(),
            properties: proto.properties.try_into()?,
        })
    }
}

impl TryFrom<Option<CreateNodeDataProto>> for CreateNodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: Option<CreateNodeDataProto>) -> Result<Self, Self::Error> {
        match proto {
            Some(p) => Self::try_from(p),
            None => Err(DtosConversionError::NoField {
                field_name: "CreateNodeDataProto".to_string(),
            }),
        }
    }
}

impl From<CreateNodeDataDto> for CreateNodeDataProto {
    fn from(dto: CreateNodeDataDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.to_string(),
            key: dto.key.into(),
            properties: dto.properties.into(),
        }
    }
}
