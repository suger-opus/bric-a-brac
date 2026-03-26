use super::RoleDto;
use crate::domain::models::{CreateGraphModel, GraphIdModel, GraphMetadataModel};
use bric_a_brac_dtos::GraphIdDto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<GraphIdModel> for GraphIdDto {
    fn from(graph_id: GraphIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<GraphIdDto> for GraphIdModel {
    fn from(graph_id: GraphIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GraphMetadataDto {
    pub graph_id: GraphIdDto,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_username: String,
    pub user_role: RoleDto,
}

impl From<GraphMetadataModel> for GraphMetadataDto {
    fn from(metadata: GraphMetadataModel) -> Self {
        Self {
            graph_id: metadata.graph_id.into(),
            name: metadata.name,
            description: metadata.description,
            is_public: metadata.is_public,
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            owner_username: metadata.owner_username,
            user_role: metadata.user_role.into(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateGraphDto {
    #[validate(length(min = 3, max = 100))]
    #[schema(min_length = 3, max_length = 100)]
    pub name: String,

    #[validate(length(min = 0, max = 10000))]
    #[schema(min_length = 0, max_length = 10000)]
    pub description: String,

    pub is_public: bool,
}

impl From<CreateGraphDto> for CreateGraphModel {
    fn from(dto: CreateGraphDto) -> Self {
        Self {
            graph_id: GraphIdModel::new(),
            name: dto.name,
            description: dto.description,
            is_public: dto.is_public,
        }
    }
}
