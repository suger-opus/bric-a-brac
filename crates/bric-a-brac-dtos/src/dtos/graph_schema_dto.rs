use super::{CreateEdgeSchemaDto, CreateNodeSchemaDto, EdgeSchemaDto, NodeSchemaDto};
use crate::DtosConversionError;
use bric_a_brac_protos::common::{CreateGraphSchemaProto, GraphSchemaProto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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
        match proto {
            Some(p) => Self::try_from(p),
            None => Err(DtosConversionError::NoField {
                field_name: "GraphSchemaProto".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateGraphSchemaDto {
    #[validate(nested)]
    pub nodes: Vec<CreateNodeSchemaDto>,

    #[validate(nested)]
    pub edges: Vec<CreateEdgeSchemaDto>,
}

impl TryFrom<CreateGraphSchemaProto> for CreateGraphSchemaDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateGraphSchemaProto) -> Result<Self, Self::Error> {
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

impl From<CreateGraphSchemaDto> for CreateGraphSchemaProto {
    fn from(dto: CreateGraphSchemaDto) -> Self {
        Self {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}
