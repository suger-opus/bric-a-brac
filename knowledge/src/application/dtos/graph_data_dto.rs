use crate::domain::models::{CreateGraphDataModel, GraphDataModel};
use bric_a_brac_dtos::{CreateGraphDataDto, GraphDataDto};

impl From<GraphDataModel> for GraphDataDto {
    fn from(model: GraphDataModel) -> Self {
        GraphDataDto {
            nodes: model.nodes.into_iter().map(From::from).collect(),
            edges: model.edges.into_iter().map(From::from).collect(),
        }
    }
}

impl From<CreateGraphDataDto> for CreateGraphDataModel {
    fn from(dto: CreateGraphDataDto) -> Self {
        CreateGraphDataModel {
            nodes: dto.nodes.into_iter().map(From::from).collect(),
            edges: dto.edges.into_iter().map(From::from).collect(),
        }
    }
}
