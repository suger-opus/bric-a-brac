use crate::domain::models::{CreateEdgeDataModel, EdgeDataIdModel, EdgeDataModel};
use bric_a_brac_dtos::{CreateEdgeDataDto, EdgeDataDto, EdgeDataIdDto};

impl From<EdgeDataIdModel> for EdgeDataIdDto {
    fn from(graph_id: EdgeDataIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<EdgeDataIdDto> for EdgeDataIdModel {
    fn from(graph_id: EdgeDataIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<EdgeDataModel> for EdgeDataDto {
    fn from(model: EdgeDataModel) -> Self {
        EdgeDataDto {
            edge_data_id: model.edge_data_id.into(),
            key: model.key.into(),
            from_node_data_id: model.from_node_data_id.into(),
            to_node_data_id: model.to_node_data_id.into(),
            properties: model.properties.into(),
        }
    }
}

impl From<CreateEdgeDataDto> for CreateEdgeDataModel {
    fn from(dto: CreateEdgeDataDto) -> Self {
        CreateEdgeDataModel {
            edge_data_id: EdgeDataIdModel::new(),
            from_node_data_id: dto.from_node_data_id.into(),
            to_node_data_id: dto.to_node_data_id.into(),
            key: dto.key.into(),
            properties: dto.properties.into(),
        }
    }
}
