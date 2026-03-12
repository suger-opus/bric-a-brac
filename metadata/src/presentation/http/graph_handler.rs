use crate::{
    application::dtos::{CreateGraphDto, GraphMetadataDto},
    presentation::{
        extractors::{AuthenticatedUser, MultipartFileUpload},
        state::ApiState,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bric_a_brac_dtos::{
    CreateGraphDataDto, CreateGraphSchemaDto, GraphDataDto, GraphIdDto, GraphSchemaDto,
};

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
    post,
    path = "/graphs/{graph_id}/schema",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to create schema for")),
    tag = "Graphs",
    request_body = CreateGraphSchemaDto,
    responses(
        (status = 201, description = "Graph schema created successfully", body = GraphSchemaDto),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.create_schema",
    skip(state, graph_id, user_id, payload)
)]
pub async fn create_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphSchemaDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .create_schema(graph_id, payload)
        .await
        .map(|schema| (StatusCode::CREATED, Json(schema)))
}

#[utoipa::path(
    post,
    path = "/graphs/{graph_id}/schema/generate",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to generate schema for")),
    tag = "Graphs",
    request_body(
        content_type = "multipart/form-data",
        description = "File containing graph data to analyze for schema generation",
        content = MultipartFileUpload
    ),
    responses(
        (status = 200, description = "Graph schema generated successfully", body = CreateGraphSchemaDto),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.generate_schema",
    skip(state, graph_id, user_id, file_content, file_type)
)]
pub async fn generate_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    MultipartFileUpload {
        file_content,
        file_type,
    }: MultipartFileUpload,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, file_content_len = %file_content.len(), file_type = ?file_type);

    state
        .graph_service
        .generate_schema(graph_id, file_content, file_type)
        .await
        .map(|schema| (StatusCode::OK, Json(schema)))
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
    skip(state, graph_id, user_id)
)]
pub async fn get_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .get_data(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(graph)))
}

#[utoipa::path(
    post,
    path = "/graphs/{graph_id}/data",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to add node data to")),
    tag = "Graphs",
    request_body = CreateGraphDataDto,
    responses(
        (status = 200, description = "Node data inserted successfully", body = GraphDataDto),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.insert_data",
    skip(state, graph_id, user_id, payload)
)]
pub async fn insert_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphDataDto>,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

    state
        .graph_service
        .insert_data(graph_id, payload)
        .await
        .map(|node| (StatusCode::OK, Json(node)))
}

#[utoipa::path(
    post,
    path = "/graphs/{graph_id}/data/generate",
    params(("graph_id" = GraphIdDto, Path, description = "ID of the graph to generate data for")),
    tag = "Graphs",
    request_body(
        content_type = "multipart/form-data",
        description = "File containing graph data to analyze for generation",
        content = MultipartFileUpload
    ),
    responses(
        (status = 200, description = "Graph data generated successfully", body = CreateGraphDataDto),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Graph not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[tracing::instrument(
    level = "trace",
    name = "graph_handler.generate_data",
    skip(state, graph_id, user_id, file_content, file_type)
)]
pub async fn generate_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    MultipartFileUpload {
        file_content,
        file_type,
    }: MultipartFileUpload,
) -> impl IntoResponse {
    tracing::debug!(graph_id = ?graph_id, user_id = ?user_id, file_content_len = %file_content.len(), file_type = ?file_type);

    state
        .graph_service
        .generate_data(graph_id, file_content, file_type)
        .await
        .map(|data| (StatusCode::OK, Json(data)))
}
