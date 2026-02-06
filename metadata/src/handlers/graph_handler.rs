use crate::dtos::graph_dto::{
    ReqPostEdgeData, ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeData, ReqPostNodeSchema,
};
use crate::extractors::AuthenticatedUser;
use crate::models::graph_model::GraphId;
use crate::state::ApiState;
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
        .map(|it| (StatusCode::OK, Json(it)))
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
        .map(|it| (StatusCode::OK, Json(it)))
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
        .map(|it| (StatusCode::OK, Json(it)))
}

pub async fn post(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<ReqPostGraph>,
) -> impl IntoResponse {
    state
        .graph_service
        .post(user_id, payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn post_node_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<ReqPostNodeSchema>,
) -> impl IntoResponse {
    state
        .graph_service
        .post_node_schema(graph_id, payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn post_edge_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<ReqPostEdgeSchema>,
) -> impl IntoResponse {
    state
        .graph_service
        .post_edge_schema(graph_id, payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
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
        .map(|it| (StatusCode::OK, Json(it)))
}

pub async fn post_node_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<ReqPostNodeData>,
) -> impl IntoResponse {
    state
        .graph_service
        .post_node_data(graph_id, payload)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}

pub async fn post_edge_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<ReqPostEdgeData>,
) -> impl IntoResponse {
    state
        .graph_service
        .post_edge_data(graph_id, payload)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}
