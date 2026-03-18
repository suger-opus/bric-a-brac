use crate::domain::models::{
    EdgeDataIdModel, EdgeDataModel, InsertEdgeDataModel,
};
use bric_a_brac_dtos::{EdgeDataDto, EdgeDataIdDto, InsertEdgeDataDto};

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

impl From<InsertEdgeDataDto> for InsertEdgeDataModel {
    fn from(dto: InsertEdgeDataDto) -> Self {
        Self {
            edge_data_id: EdgeDataIdModel::new(),
            from_node_data_id: dto.from_node_data_id.into(),
            to_node_data_id: dto.to_node_data_id.into(),
            key: dto.key.into(),
            properties: dto.properties.into(),
            session_id: dto.session_id,
        }
    }
}
