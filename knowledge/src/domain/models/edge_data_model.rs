use super::PropertiesDataModel;
use crate::domain::models::NodeDataIdModel;
use bric_a_brac_id::id;

id!(EdgeDataIdModel);

pub struct EdgeDataModel {
    pub edge_data_id: EdgeDataIdModel,
    pub key: String,
    pub from_node_data_id: NodeDataIdModel,
    pub to_node_data_id: NodeDataIdModel,
    pub properties: PropertiesDataModel,
}

pub struct CreateEdgeDataModel {
    pub edge_data_id: EdgeDataIdModel,
    pub from_node_data_id: NodeDataIdModel,
    pub to_node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
}

pub struct UpdateEdgeDataModel {
    pub edge_data_id: EdgeDataIdModel,
    pub properties: PropertiesDataModel,
}
