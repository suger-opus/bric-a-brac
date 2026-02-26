use super::RoleModel;
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(GraphIdModel);

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditModel {}

#[derive(Debug)]
pub struct GraphModel {
    pub graph_id: GraphIdModel,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub reddit: RedditModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
}

#[derive(Debug)]
pub struct CreateGraphModel {
    pub graph_id: GraphIdModel,
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

#[derive(Debug)]
pub struct GraphMetadataModel {
    pub graph_id: GraphIdModel,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub reddit: RedditModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
    pub owner_username: String,
    pub user_role: RoleModel,
    pub is_bookmarked_by_user: bool,
    pub is_cheered_by_user: bool,
    pub nb_bookmarks: u32,
    pub nb_cheers: u32,
}
