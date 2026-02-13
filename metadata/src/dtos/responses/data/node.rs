use crate::{
    dtos::PropertiesDto,
    models::{NodeData, NodeDataId},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NodeDataResponse {
    pub node_data_id: NodeDataId,
    pub formatted_label: String,
    pub properties: PropertiesDto,
}

impl From<NodeData> for NodeDataResponse {
    fn from(node_data: NodeData) -> Self {
        Self {
            node_data_id: node_data.node_data_id,
            formatted_label: node_data.formatted_label,
            properties: node_data.properties.into(),
        }
    }
}
