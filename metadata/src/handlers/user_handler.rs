use crate::dtos::user_dto::PostUser;
use crate::models::user_model::UserId;
use crate::state::ApiState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    user_id: UserId,
}

pub async fn post(
    State(state): State<ApiState>,
    Json(payload): Json<PostUser>,
) -> impl IntoResponse {
    state
        .user_service
        .post(payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn get(
    State(state): State<ApiState>,
    Query(query): Query<UserQuery>,
) -> impl IntoResponse {
    state
        .user_service
        .get(query.user_id)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}
