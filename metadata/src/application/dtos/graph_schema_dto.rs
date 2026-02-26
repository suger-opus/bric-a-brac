use crate::domain::models::{CreateGraphSchemaModel, GraphSchemaModel};
use bric_a_brac_dtos::{CreateGraphSchemaDto, GraphSchemaDto};

impl From<GraphSchemaModel> for GraphSchemaDto {
    fn from(schema: GraphSchemaModel) -> Self {
        Self {
            nodes: schema.nodes.into_iter().map(From::from).collect(),
            edges: schema.edges.into_iter().map(From::from).collect(),
        }
    }
}

impl From<CreateGraphSchemaDto> for CreateGraphSchemaModel {
    fn from(dto: CreateGraphSchemaDto) -> Self {
        Self {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}
