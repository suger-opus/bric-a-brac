use crate::models::{
    access_model::Role,
    edge_schema::EdgeSchema,
    graph_model::Graph,
    node_schema::NodeSchema,
    property_model::{Property, PropertyMetadata, PropertyType},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ReqPostGraph {
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct ReqPostNodeSchema {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<ReqPostProperty>,
}

#[derive(Debug, Deserialize)]
pub struct ReqPostEdgeSchema {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<ReqPostProperty>,
}

#[derive(Debug, Deserialize)]
pub struct ReqPostProperty {
    pub label: String,
    pub formatted_label: String,
    pub property_type: PropertyType,
    pub metadata: PropertyMetadata,
}

#[derive(Debug, Serialize)]
pub struct ResGraphMetadata {
    #[serde(flatten)]
    pub graph: Graph,

    pub owner_username: String,
    pub user_role: Role,
    pub is_bookmarked_by_user: bool,
    pub is_cheered_by_user: bool,
    pub nb_bookmarks: u32,
    pub nb_cheers: u32,
}

#[derive(Debug, Serialize)]
pub struct ResGraphSchema {
    pub node_schemas: Vec<ResNodeSchema>,
    pub edge_schemas: Vec<ResEdgeSchema>,
}

#[derive(Debug, Serialize)]
pub struct ResNodeSchema {
    #[serde(flatten)]
    pub node_schema: NodeSchema,

    pub properties: Vec<Property>,
}

#[derive(Debug, Serialize)]
pub struct ResEdgeSchema {
    #[serde(flatten)]
    pub edge_schema: EdgeSchema,

    pub properties: Vec<Property>,
}

#[derive(Debug, Serialize)]
pub struct ResGraphData {
    pub nodes: Vec<ResNode>,
    pub edges: Vec<ResEdge>,
}

#[derive(Debug, Serialize)]
pub struct ResNode {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ResEdge {
    pub id: String,
    pub label: String,
    pub from_id: String,
    pub to_id: String,
    pub properties: HashMap<String, serde_json::Value>,
}
