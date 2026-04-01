use crate::{
    application::{CreateUserDto, UserDto},
    presentation::http::{ApiState, AuthenticatedUser},
};
use crate::presentation::error::PresentationError;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/users",
    tag = "Users",
    request_body = CreateUserDto,
    responses(
        (status = 201, description = "User created successfully", body = UserDto),
        (status = 400, description = "Invalid request payload"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(level = "trace", name = "user_handler.create", skip(state, payload))]
pub async fn create(
    State(state): State<ApiState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<impl IntoResponse, PresentationError> {
    tracing::debug!(email = ?payload.email);

    payload.validate()?;

    state
        .user_service
        .create(payload)
        .await
        .map(|user| (StatusCode::CREATED, Json(user)))
        .map_err(PresentationError::from)
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "Users",
    responses(
        (status = 200, description = "User retrieved successfully", body = UserDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "user_handler.get_current",
    skip(state, user_id)
)]
pub async fn get_current(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(user_id = ?user_id);

    state
        .user_service
        .get(user_id)
        .await
        .map(|user| (StatusCode::OK, Json(user)))
}
