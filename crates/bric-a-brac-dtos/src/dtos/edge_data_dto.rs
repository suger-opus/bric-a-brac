use super::{NodeDataIdDto, PropertiesDataDto};
use crate::DtosConversionError;
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{CreateEdgeDataProto, EdgeDataProto};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

id!(EdgeDataIdDto);

impl TryFrom<String> for EdgeDataIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(EdgeDataIdDto::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EdgeDataDto {
    pub edge_data_id: EdgeDataIdDto,
    pub key: String,
    pub from_node_data_id: NodeDataIdDto,
    pub to_node_data_id: NodeDataIdDto,
    pub properties: PropertiesDataDto,
}

impl TryFrom<EdgeDataProto> for EdgeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: EdgeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            edge_data_id: proto.edge_data_id.try_into()?,
            key: proto.key,
            from_node_data_id: proto.from_node_data_id.try_into()?,
            to_node_data_id: proto.to_node_data_id.try_into()?,
            properties: proto.properties.try_into()?,
        })
    }
}

impl From<EdgeDataDto> for EdgeDataProto {
    fn from(dto: EdgeDataDto) -> Self {
        Self {
            edge_data_id: dto.edge_data_id.to_string(),
            key: dto.key,
            from_node_data_id: dto.from_node_data_id.to_string(),
            to_node_data_id: dto.to_node_data_id.to_string(),
            properties: dto.properties.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateEdgeDataDto {
    pub key: String,
    pub from_node_data_id: NodeDataIdDto,
    pub to_node_data_id: NodeDataIdDto,
    pub properties: PropertiesDataDto,
}

impl TryFrom<CreateEdgeDataProto> for CreateEdgeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateEdgeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            key: proto.key,
            from_node_data_id: proto.from_node_data_id.try_into()?,
            to_node_data_id: proto.to_node_data_id.try_into()?,
            properties: proto.properties.try_into()?,
        })
    }
}

impl From<CreateEdgeDataDto> for CreateEdgeDataProto {
    fn from(dto: CreateEdgeDataDto) -> Self {
        Self {
            key: dto.key,
            from_node_data_id: dto.from_node_data_id.to_string(),
            to_node_data_id: dto.to_node_data_id.to_string(),
            properties: dto.properties.into(),
        }
    }
}

impl TryFrom<Option<CreateEdgeDataProto>> for CreateEdgeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: Option<CreateEdgeDataProto>) -> Result<Self, Self::Error> {
        match proto {
            Some(p) => Self::try_from(p),
            None => Err(DtosConversionError::NoField {
                field_name: "CreateEdgeDataProto".to_string(),
            }),
        }
    }
}
