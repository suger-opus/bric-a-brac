use super::{CreateEdgeDataDto, CreateNodeDataDto, EdgeDataDto, NodeDataDto};
use crate::DtosConversionError;
use bric_a_brac_protos::common::{CreateGraphDataProto, GraphDataProto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphDataDto {
    pub nodes: Vec<NodeDataDto>,
    pub edges: Vec<EdgeDataDto>,
}

impl From<GraphDataDto> for GraphDataProto {
    fn from(dto: GraphDataDto) -> Self {
        Self {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}

impl TryFrom<GraphDataProto> for GraphDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: GraphDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            nodes: proto
                .nodes
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            edges: proto
                .edges
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateGraphDataDto {
    #[validate(nested)]
    pub nodes: Vec<CreateNodeDataDto>,

    #[validate(nested)]
    pub edges: Vec<CreateEdgeDataDto>,
}

impl TryFrom<CreateGraphDataProto> for CreateGraphDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateGraphDataProto) -> Result<Self, Self::Error> {
        Ok(Self {
            nodes: proto
                .nodes
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            edges: proto
                .edges
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl TryFrom<Option<CreateGraphDataProto>> for CreateGraphDataDto {
    type Error = DtosConversionError;

    fn try_from(proto: Option<CreateGraphDataProto>) -> Result<Self, Self::Error> {
        let proto = proto.ok_or_else(|| DtosConversionError::NoField {
            field_name: "CreateGraphDataProto".to_string(),
        })?;
        Self::try_from(proto)
    }
}

impl From<CreateGraphDataDto> for CreateGraphDataProto {
    fn from(dto: CreateGraphDataDto) -> Self {
        Self {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}
