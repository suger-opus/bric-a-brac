use crate::domain::models::{CreateNodeDataModel, NodeDataIdModel, NodeDataModel};
use bric_a_brac_dtos::{CreateNodeDataDto, NodeDataDto, NodeDataIdDto};

impl From<NodeDataIdModel> for NodeDataIdDto {
    fn from(graph_id: NodeDataIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<NodeDataIdDto> for NodeDataIdModel {
    fn from(graph_id: NodeDataIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<NodeDataModel> for NodeDataDto {
    fn from(model: NodeDataModel) -> Self {
        NodeDataDto {
            node_data_id: model.node_data_id.into(),
            key: model.key,
            properties: model.properties.into(),
        }
    }
}

impl From<CreateNodeDataDto> for CreateNodeDataModel {
    fn from(dto: CreateNodeDataDto) -> Self {
        CreateNodeDataModel {
            node_data_id: NodeDataIdModel::new(),
            key: dto.key,
            properties: dto.properties.into(),
        }
    }
}
