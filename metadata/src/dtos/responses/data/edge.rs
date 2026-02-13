use crate::{
    dtos::PropertiesDto,
    models::{EdgeData, EdgeDataId, NodeDataId},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EdgeDataResponse {
    pub edge_data_id: EdgeDataId,
    pub formatted_label: String,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesDto,
}

impl From<EdgeData> for EdgeDataResponse {
    fn from(edge_data: EdgeData) -> Self {
        Self {
            edge_data_id: edge_data.edge_data_id,
            formatted_label: edge_data.formatted_label,
            from_node_data_id: edge_data.from_node_data_id,
            to_node_data_id: edge_data.to_node_data_id,
            properties: edge_data.properties.into(),
        }
    }
}
