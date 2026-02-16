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

pub async fn get_all_metadata(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_all_metadata(user_id)
        .await
        .map(|graphs| (StatusCode::OK, Json(graphs)))
}

pub async fn get_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_metadata(user_id, graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

pub async fn get_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_schema(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

pub async fn get_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_data(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

pub async fn create_graph(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphDto>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_graph(user_id, payload)
        .await
        .map(|graph| (StatusCode::CREATED, Json(graph)))
}

pub async fn generate_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    MultipartFileUpload(file_content, file_type): MultipartFileUpload,
) -> Result<impl IntoResponse, impl IntoResponse> {
    state
        .graph_service
        .generate_schema(graph_id, file_content, file_type)
        .await
        .map(|schema| (StatusCode::OK, Json(schema)))
}

pub async fn create_node_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateNodeSchemaDto>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_node_schema(graph_id, payload)
        .await
        .map(|node| (StatusCode::CREATED, Json(node)))
}

pub async fn create_edge_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateEdgeSchemaDto>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_edge_schema(graph_id, payload)
        .await
        .map(|edge| (StatusCode::CREATED, Json(edge)))
}

pub async fn insert_node_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateNodeDataDto>,
) -> impl IntoResponse {
    state
        .graph_service
        .insert_node_data(graph_id, payload)
        .await
        .map(|node| (StatusCode::OK, Json(node)))
}

pub async fn insert_edge_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateEdgeDataDto>,
) -> impl IntoResponse {
    state
        .graph_service
        .insert_edge_data(graph_id, payload)
        .await
        .map(|edge| (StatusCode::OK, Json(edge)))
}
