use super::RoleModel;
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};

id!(GraphIdModel);

#[derive(Debug)]
pub struct GraphModel {
    pub graph_id: GraphIdModel,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_username: String,
    pub user_role: RoleModel,
}
