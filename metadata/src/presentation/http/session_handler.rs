use crate::{
    application::dtos::{CreateSessionDto, SessionDto, SessionIdDto, SessionMessageDto},
    presentation::{extractors::AuthenticatedUser, state::ApiState},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bric_a_brac_dtos::GraphIdDto;

#[utoipa::path(
    post,
    path = "/sessions",
    tag = "Sessions",
    request_body = CreateSessionDto,
    responses(
        (status = 201, description = "Session created", body = SessionDto),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.create",
    skip(state, user_id, payload)
)]
pub async fn create(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateSessionDto>,
) -> impl IntoResponse {
    state
        .session_service
        .create_session(payload, user_id)
        .await
        .map(|s| (StatusCode::CREATED, Json(s)))
}

#[utoipa::path(
    get,
    path = "/graphs/{graph_id}/active-session",
    params(("graph_id" = String, Path, description = "Graph ID")),
    tag = "Sessions",
    responses(
        (status = 200, description = "Active session found", body = SessionDto),
        (status = 204, description = "No active session"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.get_active",
    skip(state, user_id, graph_id)
)]
pub async fn get_active(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(graph_id): Path<GraphIdDto>,
) -> impl IntoResponse {
    match state
        .session_service
        .get_active_session(graph_id, user_id)
        .await
    {
        Ok(Some(session)) => (StatusCode::OK, Json(session)).into_response(),
        Ok(None) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => err.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/sessions/{session_id}",
    params(("session_id" = SessionIdDto, Path, description = "Session ID")),
    tag = "Sessions",
    responses(
        (status = 200, description = "Session retrieved", body = SessionDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.get",
    skip(state, _user_id, session_id)
)]
pub async fn get(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
    Path(session_id): Path<SessionIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .get_session(session_id)
        .await
        .map(|s| (StatusCode::OK, Json(s)))
}

#[utoipa::path(
    post,
    path = "/sessions/{session_id}/close",
    params(("session_id" = SessionIdDto, Path, description = "Session ID")),
    tag = "Sessions",
    responses(
        (status = 200, description = "Session closed", body = SessionDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.close",
    skip(state, _user_id, session_id)
)]
pub async fn close(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
    Path(session_id): Path<SessionIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .close_session(session_id, "completed")
        .await
        .map(|s| (StatusCode::OK, Json(s)))
}

#[utoipa::path(
    get,
    path = "/sessions/{session_id}/messages",
    params(("session_id" = SessionIdDto, Path, description = "Session ID")),
    tag = "Sessions",
    responses(
        (status = 200, description = "Messages retrieved", body = [SessionMessageDto]),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.get_messages",
    skip(state, _user_id, session_id)
)]
pub async fn get_messages(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
    Path(session_id): Path<SessionIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .get_messages(session_id)
        .await
        .map(|msgs| (StatusCode::OK, Json(msgs)))
}
