use super::PropertiesData;
use bric_a_brac_id::id;

id!(NodeDataId);

#[derive(Debug)]
pub struct NodeData {
    pub node_data_id: NodeDataId,
    pub key: String,
    pub properties: PropertiesData,
}

#[derive(Debug, Clone)]
pub struct CreateNodeData {
    pub key: String,
    pub properties: PropertiesData,
}
