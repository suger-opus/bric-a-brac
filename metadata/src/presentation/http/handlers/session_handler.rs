use crate::presentation::http::{ApiState, AuthenticatedUser};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bric_a_brac_dtos::{GraphIdDto, SessionDto, SessionIdDto, SessionMessageDto};

#[utoipa::path(
    post,
    path = "/graphs/{graph_id}/sessions",
    params(("graph_id" = String, Path, description = "Graph ID")),
    tag = "Sessions",
    responses(
        (status = 201, description = "Session created", body = SessionDto),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Active session already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.create",
    skip(state, user_id, graph_id)
)]
pub async fn create(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(graph_id): Path<GraphIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .create(graph_id, user_id)
        .await
        .map(|s| (StatusCode::CREATED, Json(s)))
}

#[utoipa::path(
    get,
    path = "/graphs/{graph_id}/sessions",
    params(("graph_id" = String, Path, description = "Graph ID")),
    tag = "Sessions",
    responses(
        (status = 200, description = "User sessions for this graph", body = [SessionDto]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "session_handler.list",
    skip(state, user_id, graph_id)
)]
pub async fn list(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(graph_id): Path<GraphIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .list(graph_id, user_id)
        .await
        .map(|sessions| (StatusCode::OK, Json(sessions)))
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
    skip(state, user_id, session_id)
)]
pub async fn close(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(session_id): Path<SessionIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .close(
            session_id,
            user_id,
            bric_a_brac_dtos::SessionStatusDto::Completed,
        )
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
    skip(state, user_id, session_id)
)]
pub async fn get_messages(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(session_id): Path<SessionIdDto>,
) -> impl IntoResponse {
    state
        .session_service
        .get_messages(session_id, user_id)
        .await
        .map(|msgs| (StatusCode::OK, Json(msgs)))
}
