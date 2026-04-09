use crate::domain::GraphIdModel;
use bric_a_brac_dtos::GraphIdDto;

impl From<GraphIdModel> for GraphIdDto {
    fn from(graph_id: GraphIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<GraphIdDto> for GraphIdModel {
    fn from(graph_id: GraphIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}
