use crate::domain::{
    CreateNodeDataModel, NodeDataIdModel, NodeDataModel, NodeSearchModel, UpdateNodeDataModel,
};
use bric_a_brac_dtos::{
    CreateNodeDataDto, NodeDataDto, NodeDataIdDto, NodeSearchDto, UpdateNodeDataDto,
};

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
        Self {
            node_data_id: model.node_data_id.into(),
            key: model.key.into(),
            properties: model.properties.into(),
        }
    }
}

impl From<CreateNodeDataDto> for CreateNodeDataModel {
    fn from(dto: CreateNodeDataDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.into(),
            key: dto.key.into(),
            properties: dto.properties.into(),
            embedding: dto.embedding,
        }
    }
}

impl From<UpdateNodeDataDto> for UpdateNodeDataModel {
    fn from(dto: UpdateNodeDataDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.into(),
            properties: dto.properties.into(),
            embedding: dto.embedding,
        }
    }
}

impl From<NodeSearchModel> for NodeSearchDto {
    fn from(model: NodeSearchModel) -> Self {
        Self {
            node_data_id: model.node_data_id.into(),
            key: model.key.into(),
            properties: model.properties.into(),
            distance: model.distance,
        }
    }
}
