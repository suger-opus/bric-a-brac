use crate::{
    dtos::PropertiesDto,
    models::{EdgeSchemaId, NewEdgeData, NodeDataId},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateEdgeDataRequest {
    pub edge_schema_id: EdgeSchemaId,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesDto,
}

impl CreateEdgeDataRequest {
    pub fn into_domain(self) -> NewEdgeData {
        NewEdgeData {
            edge_schema_id: self.edge_schema_id,
            from_node_data_id: self.from_node_data_id,
            to_node_data_id: self.to_node_data_id,
            properties: self.properties.into(),
        }
    }
}
