use super::PropertiesDataDto;
use crate::domain::models::{CreateEdgeData, EdgeData, EdgeDataId, EdgeSchemaId, NodeDataId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateEdgeDataDto {
    pub edge_schema_id: EdgeSchemaId,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesDataDto,
}

impl CreateEdgeDataDto {
    pub fn into_domain(self) -> CreateEdgeData {
        CreateEdgeData {
            edge_schema_id: self.edge_schema_id,
            from_node_data_id: self.from_node_data_id,
            to_node_data_id: self.to_node_data_id,
            properties: self.properties.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EdgeDataDto {
    pub edge_data_id: EdgeDataId,
    pub formatted_label: String,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesDataDto,
}

impl From<EdgeData> for EdgeDataDto {
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
