use super::{GraphIdModel, UserIdModel};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum RoleModel {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

impl std::fmt::Display for RoleModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owner => write!(f, "Owner"),
            Self::Admin => write!(f, "Admin"),
            Self::Editor => write!(f, "Editor"),
            Self::Viewer => write!(f, "Viewer"),
            Self::None => write!(f, "None"),
        }
    }
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
