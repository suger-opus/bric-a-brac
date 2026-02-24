use super::PropertiesDataDto;
use crate::domain::models::{CreateNodeData, NodeData, NodeDataId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateNodeDataDto {
    pub key: String,
    pub properties: PropertiesDataDto,
}

impl CreateNodeDataDto {
    pub fn into_domain(self) -> CreateNodeData {
        CreateNodeData {
            key: self.key,
            properties: self.properties.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NodeDataDto {
    pub node_data_id: NodeDataId,
    pub key: String,
    pub properties: PropertiesDataDto,
}

impl From<NodeData> for NodeDataDto {
    fn from(node_data: NodeData) -> Self {
        Self {
            node_data_id: node_data.node_data_id,
            key: node_data.key,
            properties: node_data.properties.into(),
        }
    }
}
