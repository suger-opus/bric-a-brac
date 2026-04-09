use super::PropertiesDataModel;
use crate::domain::models::GraphIdModel;
use bric_a_brac_id::id;

id!(NodeDataIdModel);

pub struct NodeDataModel {
    pub graph_id: GraphIdModel,
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
}

pub struct NodeSearchModel {
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
    pub distance: f32,
}

pub struct CreateNodeDataModel {
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
    pub embedding: Vec<f32>,
}

pub struct UpdateNodeDataModel {
    pub node_data_id: NodeDataIdModel,
    pub properties: PropertiesDataModel,
    pub embedding: Vec<f32>,
}
