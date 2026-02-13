use crate::{
    dtos::RoleDto,
    models::{Access, GraphId, UserId},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessResponse {
    pub graph_id: GraphId,
    pub user_id: UserId,
    pub role: RoleDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Access> for AccessResponse {
    fn from(access: crate::models::Access) -> Self {
        AccessResponse {
            graph_id: access.graph_id,
            user_id: access.user_id,
            role: access.role.into(),
            created_at: access.created_at,
            updated_at: access.updated_at,
        }
    }
}
