use super::{GraphId, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Access {
    pub graph_id: GraphId,
    pub user_id: UserId,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateAccess {
    pub graph_id: GraphId,
    pub user_id: UserId,
    pub role: Role,
}
