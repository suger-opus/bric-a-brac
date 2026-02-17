use super::{EdgeSchemaId, NodeDataId, PropertiesData};
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

#[derive(Debug, Clone)]
pub struct CreateEdgeData {
    pub edge_schema_id: EdgeSchemaId,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesData,
}
