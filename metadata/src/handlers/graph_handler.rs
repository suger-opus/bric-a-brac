use crate::dtos::graph_dto::PostGraph;
use crate::models::{graph_model::GraphId, user_model::UserId};
use crate::state::ApiState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostGraphQuery {
    user_id: UserId,
}

#[derive(Debug, Deserialize)]
pub struct GetOneGraphQuery {
    user_id: UserId,
}

#[derive(Debug, Deserialize)]
pub struct GetAllGraphQuery {
    user_id: UserId,
}

pub async fn post(
    State(state): State<ApiState>,
    Query(query): Query<PostGraphQuery>,
    Json(payload): Json<PostGraph>,
) -> impl IntoResponse {
    state
        .graph_service
        .post(query.user_id, payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn get_one_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    Query(query): Query<GetOneGraphQuery>,
) -> impl IntoResponse {
    state
        .graph_service
        .get_one_metadata(query.user_id, graph_id)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}

pub async fn get_all_metadata(
    State(state): State<ApiState>,
    Query(query): Query<GetAllGraphQuery>,
) -> impl IntoResponse {
    state
        .graph_service
        .get_all_metadata(query.user_id)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}
