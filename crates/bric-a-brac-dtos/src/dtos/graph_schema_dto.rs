use super::{EdgeSchemaDto, NodeSchemaDto};
use crate::DtosConversionError;
use bric_a_brac_protos::common::GraphSchemaProto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphSchemaDto {
    pub nodes: Vec<NodeSchemaDto>,
    pub edges: Vec<EdgeSchemaDto>,
}

impl From<GraphSchemaDto> for GraphSchemaProto {
    fn from(dto: GraphSchemaDto) -> Self {
        Self {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}

impl TryFrom<GraphSchemaProto> for GraphSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: GraphSchemaProto) -> Result<Self, Self::Error> {
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

impl TryFrom<Option<GraphSchemaProto>> for GraphSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: Option<GraphSchemaProto>) -> Result<Self, Self::Error> {
        proto.map_or_else(
            || {
                Err(DtosConversionError::NoField {
                    field_name: "GraphSchemaProto".to_owned(),
                })
            },
            Self::try_from,
        )
    }
}
