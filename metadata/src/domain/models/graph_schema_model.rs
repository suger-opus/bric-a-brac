use super::{CreateEdgeSchemaModel, CreateNodeSchemaModel, EdgeSchemaModel, NodeSchemaModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GraphSchemaModel {
    pub nodes: Vec<NodeSchemaModel>,
    pub edges: Vec<EdgeSchemaModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGraphSchemaModel {
    pub nodes: Vec<CreateNodeSchemaModel>,
    pub edges: Vec<CreateEdgeSchemaModel>,
}
