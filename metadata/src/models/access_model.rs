use crate::models::{graph_model::GraphId, user_model::UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

#[derive(Debug, Clone, Serialize)]
pub struct Access {
    pub graph_id: GraphId,
    pub user_id: UserId,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
