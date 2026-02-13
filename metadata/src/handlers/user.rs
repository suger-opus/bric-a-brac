use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    dtos::{CreateUserRequest, UserResponse},
    extractors::AuthenticatedUser,
    state::ApiState,
};

pub async fn create(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    state
        .user_service
        .create(payload.into_domain())
        .await
        .map(|user| (StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn get_current(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .user_service
        .get(user_id)
        .await
        .map(|user| (StatusCode::OK, Json(UserResponse::from(user))))
}
