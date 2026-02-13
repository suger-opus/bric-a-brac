use crate::models::{NodeSchemaId, PropertiesData};
use bric_a_brac_id::id;

id!(NodeDataId);

#[derive(Debug)]
pub struct NodeData {
    pub node_data_id: NodeDataId,
    pub formatted_label: String,
    pub properties: PropertiesData,
}

#[derive(Debug)]
pub struct NewNodeData {
    pub node_schema_id: NodeSchemaId,
    pub properties: PropertiesData,
}
