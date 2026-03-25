use crate::domain::models::{
    InsertNodeDataModel, NodeDataIdModel, NodeDataModel, NodeSummaryModel, UpdateNodeDataModel,
};
use bric_a_brac_dtos::{
    InsertNodeDataDto, NodeDataDto, NodeDataIdDto, PropertiesDataDto, UpdateNodeDataDto,
};
use bric_a_brac_protos::common::NodeSummaryProto;

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

impl From<InsertNodeDataDto> for InsertNodeDataModel {
    fn from(dto: InsertNodeDataDto) -> Self {
        Self {
            node_data_id: dto.node_data_id.into(),
            key: dto.key.into(),
            properties: dto.properties.into(),
            embedding: dto.embedding,
            session_id: dto.session_id,
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

impl From<NodeSummaryModel> for NodeSummaryProto {
    fn from(model: NodeSummaryModel) -> Self {
        let props_dto: PropertiesDataDto = model.properties.into();
        Self {
            node_data_id: model.node_data_id.to_string(),
            key: model.key,
            properties: props_dto.into(),
            distance: model.distance,
        }
    }
}

impl From<NodeDataModel> for NodeSummaryModel {
    fn from(model: NodeDataModel) -> Self {
        Self {
            node_data_id: model.node_data_id,
            key: model.key,
            properties: model.properties,
            distance: 0.0,
        }
    }
}
