use super::{GraphIdModel, RoleModel, UserIdModel};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(SessionIdModel);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum SessionStatusModel {
    Active,
    Completed,
    Error,
}

impl std::fmt::Display for SessionStatusModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Completed => write!(f, "completed"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl std::str::FromStr for SessionStatusModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "error" => Ok(Self::Error),
            other => Err(format!("Invalid session status: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionModel {
    pub session_id: SessionIdModel,
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
    pub status: SessionStatusModel,
    pub role: RoleModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateSessionModel {
    pub session_id: SessionIdModel,
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
}
