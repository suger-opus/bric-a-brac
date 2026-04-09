use crate::domain::GraphDataModel;
use bric_a_brac_dtos::GraphDataDto;

impl From<GraphDataModel> for GraphDataDto {
    fn from(model: GraphDataModel) -> Self {
        Self {
            nodes: model.nodes.into_iter().map(From::from).collect(),
            edges: model.edges.into_iter().map(From::from).collect(),
        }
    }
}
