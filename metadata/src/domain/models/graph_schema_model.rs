use super::{EdgeSchemaModel, NodeSchemaModel};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GraphSchemaModel {
    pub nodes: Vec<NodeSchemaModel>,
    pub edges: Vec<EdgeSchemaModel>,
}
