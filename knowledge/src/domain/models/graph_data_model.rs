use super::{CreateEdgeDataModel, CreateNodeDataModel, EdgeDataModel, NodeDataModel};
use bric_a_brac_id::id;

id!(GraphIdModel);

pub struct GraphDataModel {
    pub nodes: Vec<NodeDataModel>,
    pub edges: Vec<EdgeDataModel>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CreateGraphDataModel {
    pub nodes: Vec<CreateNodeDataModel>,
    pub edges: Vec<CreateEdgeDataModel>,
}
