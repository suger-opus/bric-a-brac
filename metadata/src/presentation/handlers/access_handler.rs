use super::super::{extractors::AuthenticatedUser, state::ApiState};
use crate::{application::dtos::CreateAccessDto, domain::models::GraphId};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn create(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateAccessDto>,
) -> impl IntoResponse {
    state
        .access_service
        .create_access(graph_id, payload)
        .await
        .map(|access| (StatusCode::CREATED, Json(access)))
}
