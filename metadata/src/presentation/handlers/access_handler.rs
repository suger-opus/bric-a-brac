use super::super::{extractors::AuthenticatedUser, state::ApiState};
use crate::{application::dtos::CreateAccessDto, domain::models::GraphId};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

#[tracing::instrument(level = "trace", skip(state, graph_id, user_id, payload))]
pub async fn create(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateAccessDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, payload = ?payload);

    state
        .access_service
        .create(graph_id, payload)
        .await
        .map(|access| (StatusCode::CREATED, Json(access)))
}
