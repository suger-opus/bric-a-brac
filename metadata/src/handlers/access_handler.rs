use crate::extractors::AuthenticatedUser;
use crate::state::ApiState;
use crate::{dtos::access_dto::PostAccess, models::graph_model::GraphId};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn post(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<PostAccess>,
) -> impl IntoResponse {
    state
        .access_service
        .post(payload.user_id, graph_id, payload.role)
        .await
        .map(|it| (StatusCode::CREATED, Json(it)))
}
