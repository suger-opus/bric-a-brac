use crate::{
    dtos::{AccessResponse, CreateAccessRequest},
    extractors::AuthenticatedUser,
    models::GraphId,
    state::ApiState,
};
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
    Json(payload): Json<CreateAccessRequest>,
) -> impl IntoResponse {
    state
        .access_service
        .create_access(payload.into_domain(graph_id))
        .await
        .map(|access| (StatusCode::CREATED, Json(AccessResponse::from(access))))
}
