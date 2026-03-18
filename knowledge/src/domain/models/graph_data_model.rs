use super::{EdgeDataModel, NodeDataModel};
use bric_a_brac_id::id;

id!(GraphIdModel);

pub struct GraphDataModel {
    pub nodes: Vec<NodeDataModel>,
    pub edges: Vec<EdgeDataModel>,
}
