use crate::{
    dtos::{EdgeDataResponse, EdgeSchemaResponse, NodeDataResponse, NodeSchemaResponse, RoleDto},
    models::{GraphData, GraphId, GraphMetadata, GraphSchema, Reddit},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RedditDto {}

impl From<Reddit> for RedditDto {
    fn from(_reddit: Reddit) -> Self {
        RedditDto {}
    }
}

#[derive(Debug, Serialize)]
pub struct GraphMetadataResponse {
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

impl From<GraphMetadata> for GraphMetadataResponse {
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

#[derive(Debug, Serialize)]
pub struct GraphSchemaResponse {
    pub nodes: Vec<NodeSchemaResponse>,
    pub edges: Vec<EdgeSchemaResponse>,
}

impl From<GraphSchema> for GraphSchemaResponse {
    fn from(schema: GraphSchema) -> Self {
        Self {
            nodes: schema
                .nodes
                .into_iter()
                .map(NodeSchemaResponse::from)
                .collect(),
            edges: schema
                .edges
                .into_iter()
                .map(EdgeSchemaResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GraphDataResponse {
    pub nodes: Vec<NodeDataResponse>,
    pub edges: Vec<EdgeDataResponse>,
}

impl From<GraphData> for GraphDataResponse {
    fn from(data: GraphData) -> Self {
        Self {
            nodes: data.nodes.into_iter().map(NodeDataResponse::from).collect(),
            edges: data.edges.into_iter().map(EdgeDataResponse::from).collect(),
        }
    }
}
