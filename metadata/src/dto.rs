use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct CreateNodeRequest {
    pub graph_id: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValueDto>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEdgeRequest {
    pub from_id: String,
    pub to_id: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValueDto>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub graph_id: Option<String>,
    pub node_label: Option<String>,
    pub node_properties: Option<HashMap<String, PropertyValueDto>>,
    pub edge_label: Option<String>,
    pub edge_properties: Option<HashMap<String, PropertyValueDto>>,
    #[serde(default)]
    pub include_edges: bool,
}

// Property value DTO for JSON serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValueDto {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct NodeResponse {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValueDto>,
}

#[derive(Debug, Serialize)]
pub struct EdgeResponse {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub label: String,
    pub properties: HashMap<String, PropertyValueDto>,
}

#[derive(Debug, Serialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<NodeResponse>,
    pub edges: Vec<EdgeResponse>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
