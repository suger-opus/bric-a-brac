use crate::domain::models::{Access, CreateAccess, GraphId, Role, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum RoleDto {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

impl From<RoleDto> for Role {
    fn from(role: RoleDto) -> Self {
        match role {
            RoleDto::Owner => Role::Owner,
            RoleDto::Admin => Role::Admin,
            RoleDto::Editor => Role::Editor,
            RoleDto::Viewer => Role::Viewer,
            RoleDto::None => Role::None,
        }
    }
}

impl From<Role> for RoleDto {
    fn from(role: Role) -> Self {
        match role {
            Role::Owner => RoleDto::Owner,
            Role::Admin => RoleDto::Admin,
            Role::Editor => RoleDto::Editor,
            Role::Viewer => RoleDto::Viewer,
            Role::None => RoleDto::None,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccessDto {
    pub user_id: UserId,
    pub role: RoleDto,
}

impl CreateAccessDto {
    pub fn into_domain(self, graph_id: GraphId) -> CreateAccess {
        CreateAccess {
            graph_id,
            user_id: self.user_id,
            role: self.role.into(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccessDto {
    pub graph_id: GraphId,
    pub user_id: UserId,
    pub role: RoleDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Access> for AccessDto {
    fn from(access: Access) -> Self {
        AccessDto {
            graph_id: access.graph_id,
            user_id: access.user_id,
            role: access.role.into(),
            created_at: access.created_at,
            updated_at: access.updated_at,
        }
    }
}
