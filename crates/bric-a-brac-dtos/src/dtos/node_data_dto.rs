use super::{KeyDto, PropertiesDataDto};
use crate::DtosConversionError;
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{
    CreateNodeDataProto, NodeDataProto, NodeSearchProto, UpdateNodeDataProto,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct NodeDataDto {
    pub node_data_id: NodeDataIdDto,
    #[validate(nested)]
    pub key: KeyDto,
    #[validate(nested)]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct NodeSearchDto {
    pub node_data_id: NodeDataIdDto,
    #[validate(nested)]
    pub key: KeyDto,
    #[validate(nested)]
    pub properties: PropertiesDataDto,
    pub distance: f32,
}

impl TryFrom<NodeSearchProto> for NodeSearchDto {
    type Error = DtosConversionError;

    fn try_from(proto: NodeSearchProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            key: proto.key.into(),
            properties: proto.properties.try_into()?,
            distance: proto.distance,
        })
    }
}

impl From<NodeSearchDto> for NodeSearchProto {
    fn from(dto: NodeSearchDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.to_string(),
            key: dto.key.into(),
            properties: dto.properties.into(),
            distance: dto.distance,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateNodeDataDto {
    pub node_data_id: NodeDataIdDto,
    #[validate(nested)]
    pub key: KeyDto,
    #[validate(nested)]
    pub properties: PropertiesDataDto,
    pub embedding: Vec<f32>,
}

impl TryFrom<CreateNodeDataProto> for CreateNodeDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateNodeDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: proto.node_data_id.try_into()?,
            key: proto.key.into(),
            properties: proto.properties.try_into()?,
            embedding: proto.embedding,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateNodeDataDto {
    pub node_data_id: NodeDataIdDto,
    #[validate(nested)]
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
