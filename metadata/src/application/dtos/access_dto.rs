use super::UserIdDto;
use crate::domain::models::{AccessModel, CreateAccessModel, GraphIdModel, RoleModel};
use bric_a_brac_dtos::GraphIdDto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AccessDto {
    pub graph_id: GraphIdDto,
    pub user_id: UserIdDto,
    pub role: RoleDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AccessModel> for AccessDto {
    fn from(access: AccessModel) -> Self {
        AccessDto {
            graph_id: access.graph_id.into(),
            user_id: access.user_id.into(),
            role: access.role.into(),
            created_at: access.created_at,
            updated_at: access.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAccessDto {
    pub user_id: UserIdDto,
    pub role: RoleDto,
}

impl CreateAccessDto {
    pub fn into_domain(self, graph_id: GraphIdModel) -> CreateAccessModel {
        CreateAccessModel {
            graph_id,
            user_id: self.user_id.into(),
            role: self.role.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum RoleDto {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}

impl From<RoleDto> for RoleModel {
    fn from(role: RoleDto) -> Self {
        match role {
            RoleDto::Owner => RoleModel::Owner,
            RoleDto::Admin => RoleModel::Admin,
            RoleDto::Editor => RoleModel::Editor,
            RoleDto::Viewer => RoleModel::Viewer,
            RoleDto::None => RoleModel::None,
        }
    }
}

impl From<RoleModel> for RoleDto {
    fn from(role: RoleModel) -> Self {
        match role {
            RoleModel::Owner => RoleDto::Owner,
            RoleModel::Admin => RoleDto::Admin,
            RoleModel::Editor => RoleDto::Editor,
            RoleModel::Viewer => RoleDto::Viewer,
            RoleModel::None => RoleDto::None,
        }
    }
}
