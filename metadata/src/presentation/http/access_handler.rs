use crate::{
    application::dtos::{AccessDto, CreateAccessDto},
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
    path = "/accesses/graphs/{graph_id}",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to grant access to")),
    tag = "Access",
    request_body = CreateAccessDto,
    responses(
        (status = 201, description = "Access created successfully", body = AccessDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "access_handler.create",
    skip(state, graph_id, user_id, payload)
)]
pub async fn create(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateAccessDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .access_service
        .create(graph_id, payload)
        .await
        .map(|access| (StatusCode::CREATED, Json(access)))
}
