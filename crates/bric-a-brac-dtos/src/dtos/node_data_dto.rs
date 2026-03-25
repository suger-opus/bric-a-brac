use super::{KeyDto, PropertiesDataDto};
use crate::DtosConversionError;
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{
    InsertNodeDataProto, NodeDataProto, UpdateNodeDataProto,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

id!(NodeDataIdDto);

impl TryFrom<String> for NodeDataIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
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

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InsertNodeDataDto {
    pub node_data_id: NodeDataIdDto,
    #[validate(nested)]
    pub key: KeyDto,
    pub properties: PropertiesDataDto,
    pub embedding: Vec<f32>,
    pub session_id: Option<String>,
}

impl TryFrom<InsertNodeDataProto> for InsertNodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: InsertNodeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            key: proto.key.into(),
            properties: proto.properties.try_into()?,
            embedding: proto.embedding,
            session_id: proto.session_id,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeDataDto {
    pub node_data_id: NodeDataIdDto,
    pub properties: PropertiesDataDto,
    pub embedding: Vec<f32>,
}

impl TryFrom<UpdateNodeDataProto> for UpdateNodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: UpdateNodeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            properties: proto.properties.try_into()?,
            embedding: proto.embedding,
        })
    }
}
