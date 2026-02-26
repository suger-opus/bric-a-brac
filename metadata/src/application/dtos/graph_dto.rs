use super::RoleDto;
use crate::domain::models::{CreateGraphModel, GraphIdModel, GraphMetadataModel, RedditModel};
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
    pub reddit: RedditDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
    pub owner_username: String,
    pub user_role: RoleDto,
    pub is_bookmarked_by_user: bool,
    pub is_cheered_by_user: bool,
    pub nb_bookmarks: u32,
    pub nb_cheers: u32,
}

impl From<GraphMetadataModel> for GraphMetadataDto {
    fn from(metadata: GraphMetadataModel) -> Self {
        Self {
            graph_id: metadata.graph_id.into(),
            name: metadata.name,
            description: metadata.description,
            is_public: metadata.is_public,
            reddit: metadata.reddit.into(),
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            nb_data_nodes: metadata.nb_data_nodes,
            nb_data_edges: metadata.nb_data_edges,
            owner_username: metadata.owner_username,
            user_role: metadata.user_role.into(),
            is_bookmarked_by_user: metadata.is_bookmarked_by_user,
            is_cheered_by_user: metadata.is_cheered_by_user,
            nb_bookmarks: metadata.nb_bookmarks,
            nb_cheers: metadata.nb_cheers,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RedditDto {}

impl From<RedditModel> for RedditDto {
    fn from(_reddit: RedditModel) -> Self {
        RedditDto {}
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
        CreateGraphModel {
            graph_id: GraphIdModel::new(),
            name: dto.name,
            description: dto.description,
            is_public: dto.is_public,
        }
    }
}
