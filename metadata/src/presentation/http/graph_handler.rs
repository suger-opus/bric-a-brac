use crate::{
    application::dtos::{
        CreateEdgeDataDto, CreateEdgeSchemaDto, CreateGraphDto, CreateNodeDataDto,
        CreateNodeSchemaDto,
    },
    domain::models::GraphId,
    presentation::{
        extractors::{AuthenticatedUser, MultipartFileUpload},
        state::ApiState,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

#[tracing::instrument(level = "trace", skip(state, user_id))]
pub async fn get_all_metadata(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(user_id = ?user_id);

    state
        .graph_service
        .get_all_metadata(user_id)
        .await
        .map(|graphs| (StatusCode::OK, Json(graphs)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id))]
pub async fn get_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_metadata(graph_id, user_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id))]
pub async fn get_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_schema(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id))]
pub async fn get_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_data(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[tracing::instrument(level = "trace", skip(state, user_id, payload))]
pub async fn create_graph(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphDto>,
) -> impl IntoResponse {
    tracing::debug!(user_id = ?user_id, payload = ?payload);

    state
        .graph_service
        .create_graph(user_id, payload)
        .await
        .map(|graph| (StatusCode::CREATED, Json(graph)))
}

#[tracing::instrument(
    level = "trace",
    skip(state, graph_id, user_id, file_content, file_type)
)]
pub async fn generate_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    MultipartFileUpload(file_content, file_type): MultipartFileUpload,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, file_type = ?file_type);

    state
        .graph_service
        .generate_schema(graph_id, file_content, file_type)
        .await
        .map(|schema| (StatusCode::OK, Json(schema)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id, payload))]
pub async fn create_node_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateNodeSchemaDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, payload = ?payload);

    state
        .graph_service
        .create_node_schema(graph_id, payload)
        .await
        .map(|node| (StatusCode::CREATED, Json(node)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id, payload))]
pub async fn create_edge_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateEdgeSchemaDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, payload = ?payload);

    state
        .graph_service
        .create_edge_schema(graph_id, payload)
        .await
        .map(|edge| (StatusCode::CREATED, Json(edge)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id, payload))]
pub async fn insert_node_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateNodeDataDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, payload = ?payload);

    state
        .graph_service
        .insert_node_data(graph_id, payload)
        .await
        .map(|node| (StatusCode::OK, Json(node)))
}

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id, payload))]
pub async fn insert_edge_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateEdgeDataDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, payload = ?payload);

    state
        .graph_service
        .insert_edge_data(graph_id, payload)
        .await
        .map(|edge| (StatusCode::OK, Json(edge)))
}
