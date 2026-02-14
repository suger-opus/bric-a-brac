use super::{EdgeData, EdgeSchema, NodeData, NodeSchema, Role};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(GraphId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Reddit {}

#[derive(Debug)]
pub struct Graph {
    pub graph_id: GraphId,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub reddit: Reddit,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
}

#[derive(Debug)]
pub struct CreateGraph {
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

#[derive(Debug)]
pub struct GraphMetadata {
    pub graph_id: GraphId,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub reddit: Reddit,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
    pub owner_username: String,
    pub user_role: Role,
    pub is_bookmarked_by_user: bool,
    pub is_cheered_by_user: bool,
    pub nb_bookmarks: u32,
    pub nb_cheers: u32,
}

#[derive(Debug)]
pub struct GraphSchema {
    pub nodes: Vec<NodeSchema>,
    pub edges: Vec<EdgeSchema>,
}

pub struct GraphData {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}
