use crate::models::{access_model::Role, graph_model::GraphId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PostGraph {
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphMetadata {
    pub graph_id: GraphId,
    pub owner_username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub user_role: Role,
    pub is_public: bool,
    pub is_bookmarked_by_user: bool,
    pub is_cheered_by_user: bool,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
    pub nb_bookmarks: u32,
    pub nb_cheers: u32,
}
