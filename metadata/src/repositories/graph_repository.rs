use crate::dtos::graph_dto::{GraphMetadata, PostGraph};
use crate::error::ApiError;
use crate::models::{
    access_model::Role,
    graph_model::{Graph, GraphId, Reddit},
    user_model::UserId,
};
use chrono::{DateTime, Utc};
use sqlx::{types::Json, PgConnection};

#[derive(Clone)]
pub struct GraphRepository;

impl GraphRepository {
    pub fn new() -> Self {
        GraphRepository
    }

    pub async fn post(
        &self,
        connection: &mut PgConnection,
        new_graph: PostGraph,
    ) -> Result<Graph, ApiError> {
        let graph_id = GraphId::new();
        let graph = sqlx::query_as!(
            GraphRow,
            r#"
INSERT INTO graphs (graph_id, name, description, is_public)
VALUES ($1, $2, $3, $4)
RETURNING
    graph_id AS "graph_id!:_",
    name,
    description,
    is_public,
    reddit AS "reddit!:_",
    created_at,
    updated_at,
    nb_data_nodes,
    nb_data_edges
            "#,
            graph_id as _,
            new_graph.name,
            new_graph.description,
            new_graph.is_public
        )
        .fetch_one(connection)
        .await?;

        Ok(graph.into())
    }

    pub async fn get_one_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<GraphMetadata, ApiError> {
        let graph = sqlx::query_as!(
            GraphMetadataRow,
            r#"
SELECT
    g.graph_id,
    u.username AS owner_username,
    g.created_at,
    g.updated_at,
    g.name,
    g.description,
    COALESCE(user_access.role, 'None') AS "user_role!:_",
    g.is_public,
    EXISTS(SELECT 1 FROM bookmarks b WHERE b.user_id = $1 AND b.graph_id = g.graph_id) AS "is_bookmarked_by_user!",
    EXISTS(SELECT 1 FROM cheers c WHERE c.user_id = $1 AND c.graph_id = g.graph_id) AS "is_cheered_by_user!",
    g.nb_data_nodes,
    g.nb_data_edges,
    (SELECT COUNT(*)::INT FROM bookmarks b WHERE b.graph_id = g.graph_id) AS "nb_bookmarks!",
    (SELECT COUNT(*)::INT FROM cheers c WHERE c.graph_id = g.graph_id) AS "nb_cheers!"
FROM graphs g
LEFT JOIN accesses user_access ON g.graph_id = user_access.graph_id AND user_access.user_id = $1
JOIN accesses owner_access ON g.graph_id = owner_access.graph_id AND owner_access.role = 'Owner'
JOIN users u ON owner_access.user_id = u.user_id
WHERE g.graph_id = $2
            "#,
            user_id as _,
            graph_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(graph.into())
    }

    pub async fn get_all_metadata(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
    ) -> Result<Vec<GraphMetadata>, ApiError> {
        let graphs = sqlx::query_as!(
            GraphMetadataRow,
            r#"
SELECT
    g.graph_id,
    u.username AS owner_username,
    g.created_at,
    g.updated_at,
    g.name,
    g.description,
    user_access.role AS "user_role!:_",
    g.is_public,
    EXISTS(SELECT 1 FROM bookmarks b WHERE b.user_id = $1 AND b.graph_id = g.graph_id) AS "is_bookmarked_by_user!",
    EXISTS(SELECT 1 FROM cheers c WHERE c.user_id = $1 AND c.graph_id = g.graph_id) AS "is_cheered_by_user!",
    g.nb_data_nodes,
    g.nb_data_edges,
    (SELECT COUNT(*)::INT FROM bookmarks b WHERE b.graph_id = g.graph_id) AS "nb_bookmarks!",
    (SELECT COUNT(*)::INT FROM cheers c WHERE c.graph_id = g.graph_id) AS "nb_cheers!"
FROM graphs g
JOIN accesses user_access ON g.graph_id = user_access.graph_id AND user_access.user_id = $1 AND user_access.role IN ('Owner', 'Admin', 'Editor', 'Viewer')
JOIN accesses owner_access ON g.graph_id = owner_access.graph_id AND owner_access.role = 'Owner'
JOIN users u ON owner_access.user_id = u.user_id
            "#,
            user_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(graphs.into_iter().map(GraphMetadata::from).collect())
    }
}

#[derive(sqlx::FromRow)]
struct GraphRow {
    graph_id: GraphId,
    name: String,
    description: String,
    is_public: bool,
    reddit: Json<Reddit>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    nb_data_nodes: i32,
    nb_data_edges: i32,
}

impl From<GraphRow> for Graph {
    fn from(row: GraphRow) -> Self {
        Self {
            graph_id: row.graph_id,
            name: row.name,
            description: row.description,
            is_public: row.is_public,
            reddit: row.reddit.0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            nb_data_nodes: row.nb_data_nodes as u32,
            nb_data_edges: row.nb_data_edges as u32,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GraphMetadataRow {
    graph_id: GraphId,
    owner_username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    description: String,
    user_role: Role,
    is_public: bool,
    is_bookmarked_by_user: bool,
    is_cheered_by_user: bool,
    nb_data_nodes: i32,
    nb_data_edges: i32,
    nb_bookmarks: i32,
    nb_cheers: i32,
}

impl From<GraphMetadataRow> for GraphMetadata {
    fn from(row: GraphMetadataRow) -> Self {
        Self {
            graph_id: row.graph_id,
            owner_username: row.owner_username,
            created_at: row.created_at,
            updated_at: row.updated_at,
            name: row.name,
            description: row.description,
            user_role: row.user_role,
            is_public: row.is_public,
            is_bookmarked_by_user: row.is_bookmarked_by_user,
            is_cheered_by_user: row.is_cheered_by_user,
            nb_data_nodes: row.nb_data_nodes as u32,
            nb_data_edges: row.nb_data_edges as u32,
            nb_bookmarks: row.nb_bookmarks as u32,
            nb_cheers: row.nb_cheers as u32,
        }
    }
}
