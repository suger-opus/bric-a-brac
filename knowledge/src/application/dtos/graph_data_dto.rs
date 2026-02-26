use crate::domain::models::GraphDataModel;
use bric_a_brac_dtos::GraphDataDto;

impl From<GraphDataModel> for GraphDataDto {
    fn from(model: GraphDataModel) -> Self {
        GraphDataDto {
            nodes: model.nodes.into_iter().map(From::from).collect(),
            edges: model.edges.into_iter().map(From::from).collect(),
        }
    }
}
