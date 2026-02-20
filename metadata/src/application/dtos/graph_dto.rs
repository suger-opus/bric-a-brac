use super::{EdgeDataDto, EdgeSchemaDto, NodeDataDto, NodeSchemaDto, RoleDto};
use crate::{
    application::dtos::{CreateEdgeSchemaDto, CreateNodeSchemaDto},
    domain::models::{
        CreateGraph, CreateGraphSchema, GraphData, GraphId, GraphMetadata, GraphSchema, Reddit,
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateGraphDto {
    #[validate(length(min = 1, max = 100))]
    #[schema(min_length = 1, max_length = 100)]
    pub name: String,

    #[validate(length(min = 1, max = 10000))]
    #[schema(min_length = 1, max_length = 10000)]
    pub description: String,

    pub is_public: bool,
}

impl CreateGraphDto {
    pub fn into_domain(self) -> CreateGraph {
        CreateGraph {
            name: self.name,
            description: self.description,
            is_public: self.is_public,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RedditDto {}

impl From<Reddit> for RedditDto {
    fn from(_reddit: Reddit) -> Self {
        RedditDto {}
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GraphMetadataDto {
    pub graph_id: GraphId,
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

impl From<GraphMetadata> for GraphMetadataDto {
    fn from(metadata: GraphMetadata) -> Self {
        Self {
            graph_id: metadata.graph_id,
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
pub struct GraphSchemaDto {
    pub nodes: Vec<NodeSchemaDto>,
    pub edges: Vec<EdgeSchemaDto>,
}

impl From<GraphSchema> for GraphSchemaDto {
    fn from(schema: GraphSchema) -> Self {
        Self {
            nodes: schema.nodes.into_iter().map(NodeSchemaDto::from).collect(),
            edges: schema.edges.into_iter().map(EdgeSchemaDto::from).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateGraphSchemaDto {
    #[validate(nested)]
    pub nodes: Vec<CreateNodeSchemaDto>,
    #[validate(nested)]
    pub edges: Vec<CreateEdgeSchemaDto>,
}

impl CreateGraphSchemaDto {
    pub fn into_domain(self) -> CreateGraphSchema {
        CreateGraphSchema {
            nodes: self
                .nodes
                .into_iter()
                .map(|node| node.into_domain())
                .collect(),
            edges: self
                .edges
                .into_iter()
                .map(|edge| edge.into_domain())
                .collect(),
        }
    }
}

impl From<CreateGraphSchema> for CreateGraphSchemaDto {
    fn from(schema: CreateGraphSchema) -> Self {
        Self {
            nodes: schema
                .nodes
                .into_iter()
                .map(CreateNodeSchemaDto::from)
                .collect(),
            edges: schema
                .edges
                .into_iter()
                .map(CreateEdgeSchemaDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GraphDataDto {
    pub nodes: Vec<NodeDataDto>,
    pub edges: Vec<EdgeDataDto>,
}

impl From<GraphData> for GraphDataDto {
    fn from(data: GraphData) -> Self {
        Self {
            nodes: data.nodes.into_iter().map(NodeDataDto::from).collect(),
            edges: data.edges.into_iter().map(EdgeDataDto::from).collect(),
        }
    }
}
