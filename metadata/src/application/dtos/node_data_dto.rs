use super::PropertiesDataDto;
use crate::domain::models::{CreateNodeData, NodeData, NodeDataId, NodeSchemaId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeDataDto {
    pub node_schema_id: NodeSchemaId,
    pub properties: PropertiesDataDto,
}

impl CreateNodeDataDto {
    pub fn into_domain(self) -> CreateNodeData {
        CreateNodeData {
            node_schema_id: self.node_schema_id,
            properties: self.properties.into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NodeDataDto {
    pub node_data_id: NodeDataId,
    pub formatted_label: String,
    pub properties: PropertiesDataDto,
}

impl From<NodeData> for NodeDataDto {
    fn from(node_data: NodeData) -> Self {
        Self {
            node_data_id: node_data.node_data_id,
            formatted_label: node_data.formatted_label,
            properties: node_data.properties.into(),
        }
    }
}
