use crate::dtos::user_dto::PostUser;
use crate::extractors::AuthenticatedUser;
use crate::state::ApiState;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn post(
    State(state): State<ApiState>,
    Json(payload): Json<PostUser>,
) -> impl IntoResponse {
    state
        .user_service
        .post(&payload)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}

pub async fn get(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .user_service
        .get(user_id)
        .await
        .map(|it| (StatusCode::OK, Json(it)))
}
