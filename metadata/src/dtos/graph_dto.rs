use crate::models::{
    access_model::Role,
    edge_data_model::EdgeDataId,
    edge_schema_model::{EdgeSchema, EdgeSchemaId},
    graph_model::Graph,
    node_data_model::NodeDataId,
    node_schema_model::{NodeSchema, NodeSchemaId},
    property_model::{Property, PropertyMetadata, PropertyType},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Shouldn't the wrapper be on top of proto properties ?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertiesDto(pub HashMap<String, serde_json::Value>);

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

#[derive(Debug, Deserialize)]
pub struct ReqPostNodeData {
    pub node_schema_id: NodeSchemaId,
    pub formatted_label: String,
    pub properties: PropertiesDto,
}

#[derive(Debug, Deserialize)]
pub struct ReqPostEdgeData {
    pub edge_schema_id: EdgeSchemaId,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub formatted_label: String,
    pub properties: PropertiesDto,
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
    pub nodes: Vec<ResNodeSchema>,
    pub edges: Vec<ResEdgeSchema>,
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
    pub nodes: Vec<ResNodeData>,
    pub edges: Vec<ResEdgeData>,
}

#[derive(Debug, Serialize)]
pub struct ResNodeData {
    pub node_data_id: NodeDataId,
    pub formatted_label: String,
    pub properties: PropertiesDto,
}

#[derive(Debug, Serialize)]
pub struct ResEdgeData {
    pub edge_data_id: EdgeDataId,
    pub formatted_label: String,
    pub from_node_data_id: NodeDataId,
    pub to_node_data_id: NodeDataId,
    pub properties: PropertiesDto,
}
