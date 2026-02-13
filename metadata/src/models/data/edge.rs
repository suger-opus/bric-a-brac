use crate::models::{EdgeSchemaId, NodeDataId, PropertiesData};
use bric_a_brac_id::id;

id!(EdgeDataId);

#[derive(Debug)]
pub struct EdgeData {
    pub edge_data_id: EdgeDataId,
    pub formatted_label: String,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesData,
}

#[derive(Debug)]
pub struct NewEdgeData {
    pub edge_schema_id: EdgeSchemaId,
    pub formatted_label: String,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesData,
}
