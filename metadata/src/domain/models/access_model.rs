use super::{GraphIdModel, UserIdModel};
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum RoleModel {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AccessModel {
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
    pub role: RoleModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateAccessModel {
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
    pub role: RoleModel,
}
