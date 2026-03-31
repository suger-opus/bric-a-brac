use crate::{
    application::{CreateGraphDto, GraphMetadataDto},
    presentation::http::{ApiState, AuthenticatedUser},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bric_a_brac_dtos::{GraphDataDto, GraphIdDto, GraphSchemaDto};

#[utoipa::path(
    get,
    path = "/graphs",
    tag = "Graphs",
    responses(
        (status = 200, description = "List of graphs retrieved successfully", body = [GraphMetadataDto]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.get_all_metadata",
    skip(state, user_id)
)]
pub async fn get_all_metadata(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(user_id = ?user_id);

    state
        .graph_service
        .get_all_metadata(user_id)
        .await
        .map(|graphs| (StatusCode::OK, Json(graphs)))
}

#[utoipa::path(
    post,
    path = "/graphs",
    tag = "Graphs",
    request_body = CreateGraphDto,
    responses(
        (status = 201, description = "Graph created successfully", body = GraphMetadataDto),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.create_graph",
    skip(state, user_id, payload)
)]
pub async fn create_graph(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphDto>,
) -> impl IntoResponse {
    tracing::debug!(user_id = ?user_id);

    state
        .graph_service
        .create_graph(user_id, payload)
        .await
        .map(|graph| (StatusCode::CREATED, Json(graph)))
}

#[utoipa::path(
    get,
    path = "/graphs/{graph_id}",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to retrieve")),
    tag = "Graphs",
    responses(
        (status = 200, description = "Graph metadata retrieved successfully", body = GraphMetadataDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.get_metadata",
    skip(state, graph_id, user_id)
)]
pub async fn get_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_metadata(graph_id, user_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[utoipa::path(
    get,
    path = "/graphs/{graph_id}/schema",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to retrieve schema for")),
    tag = "Graphs",
    responses(
        (status = 200, description = "Graph schema retrieved successfully", body = GraphSchemaDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.get_schema",
    skip(state, graph_id, user_id)
)]
pub async fn get_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_schema(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[utoipa::path(
    get,
    path = "/graphs/{graph_id}/data",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to retrieve data for")),
    tag = "Graphs",
    responses(
        (status = 200, description = "Graph data retrieved successfully", body = GraphDataDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.get_data",
    skip(state, graph_id, _user_id)
)]
pub async fn get_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id);

    state
        .graph_service
        .get_data(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[utoipa::path(
    delete,
    path = "/graphs/{graph_id}",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to delete")),
    tag = "Graphs",
    responses(
        (status = 204, description = "Graph deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.delete_graph",
    skip(state, graph_id, _user_id)
)]
pub async fn delete_graph(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id);

    state
        .graph_service
        .delete_graph(graph_id)
        .await
        .map(|()| StatusCode::NO_CONTENT)
}
