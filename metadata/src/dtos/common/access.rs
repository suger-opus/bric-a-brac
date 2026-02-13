use crate::models::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
