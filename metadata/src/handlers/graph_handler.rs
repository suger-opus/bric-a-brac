use crate::dtos::graph_dto::PostGraph;
use crate::extractors::AuthenticatedUser;
use crate::models::graph_model::GraphId;
use crate::state::ApiState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn post(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<PostGraph>,
) -> impl IntoResponse {
    state
        .graph_service
        .post(user_id, payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn get_one_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_one_metadata(user_id, graph_id)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}

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
