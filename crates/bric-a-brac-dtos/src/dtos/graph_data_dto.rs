use super::{EdgeDataDto, NodeDataDto};
use crate::DtosConversionError;
use bric_a_brac_protos::common::GraphDataProto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
