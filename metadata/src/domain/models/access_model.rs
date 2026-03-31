use super::{GraphIdModel, UserIdModel};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, sqlx::Type, derive_more::Display)]
#[sqlx(type_name = "role_type", rename_all = "lowercase")]
pub enum RoleModel {
    #[display("owner")]
    Owner,
    #[display("admin")]
    Admin,
    #[display("editor")]
    Editor,
    #[display("viewer")]
    Viewer,
    #[display("none")]
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
