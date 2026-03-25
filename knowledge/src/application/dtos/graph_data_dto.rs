use crate::domain::models::GraphDataModel;
use bric_a_brac_dtos::GraphDataDto;
use bric_a_brac_protos::common::{PathProto, SubgraphProto};

impl From<GraphDataModel> for GraphDataDto {
    fn from(model: GraphDataModel) -> Self {
        Self {
            nodes: model.nodes.into_iter().map(From::from).collect(),
            edges: model.edges.into_iter().map(From::from).collect(),
        }
    }
}

fn graph_data_to_proto_vecs(
    model: GraphDataModel,
) -> (
    Vec<bric_a_brac_protos::common::NodeDataProto>,
    Vec<bric_a_brac_protos::common::EdgeDataProto>,
) {
    let dto: GraphDataDto = model.into();
    let nodes = dto.nodes.into_iter().map(Into::into).collect();
    let edges = dto.edges.into_iter().map(Into::into).collect();
    (nodes, edges)
}

impl From<GraphDataModel> for SubgraphProto {
    fn from(model: GraphDataModel) -> Self {
        let (nodes, edges) = graph_data_to_proto_vecs(model);
        Self { nodes, edges }
    }
}

impl From<GraphDataModel> for PathProto {
    fn from(model: GraphDataModel) -> Self {
        let (nodes, edges) = graph_data_to_proto_vecs(model);
        Self { nodes, edges }
    }
}
