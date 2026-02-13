use crate::{
    dtos::{
        CreateEdgeDataRequest, CreateEdgeSchemaRequest, CreateGraphRequest, CreateNodeDataRequest,
        CreateNodeSchemaRequest, EdgeDataResponse, EdgeSchemaResponse, GraphDataResponse,
        GraphMetadataResponse, GraphSchemaResponse, NodeDataResponse, NodeSchemaResponse,
    },
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

pub async fn get_all_metadata(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_all_metadata(user_id)
        .await
        .map(|graphs| {
            (
                StatusCode::OK,
                Json(
                    graphs
                        .into_iter()
                        .map(GraphMetadataResponse::from)
                        .collect::<Vec<_>>(),
                ),
            )
        })
}

pub async fn get_metadata(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_metadata(user_id, graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(GraphMetadataResponse::from(graph))))
}

pub async fn get_schema(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_schema(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(GraphSchemaResponse::from(graph))))
}

pub async fn get_data(
    State(state): State<ApiState>,
    Path(graph_id): Path<GraphId>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
) -> impl IntoResponse {
    state
        .graph_service
        .get_data(graph_id)
        .await
        .map(|graph| (StatusCode::OK, Json(GraphDataResponse::from(graph))))
}

pub async fn create_graph(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGraphRequest>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_graph(user_id, payload.into_domain())
        .await
        .map(|graph| {
            (
                StatusCode::CREATED,
                Json(GraphMetadataResponse::from(graph)),
            )
        })
}

pub async fn create_node_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateNodeSchemaRequest>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_node_schema(graph_id, payload.into_domain())
        .await
        .map(|node| (StatusCode::CREATED, Json(NodeSchemaResponse::from(node))))
}

pub async fn create_edge_schema(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateEdgeSchemaRequest>,
) -> impl IntoResponse {
    state
        .graph_service
        .create_edge_schema(graph_id, payload.into_domain())
        .await
        .map(|edge| (StatusCode::CREATED, Json(EdgeSchemaResponse::from(edge))))
}

pub async fn insert_node_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateNodeDataRequest>,
) -> impl IntoResponse {
    state
        .graph_service
        .insert_node_data(graph_id, payload.into_domain())
        .await
        .map(|node| (StatusCode::OK, Json(NodeDataResponse::from(node))))
}

pub async fn insert_edge_data(
    State(state): State<ApiState>,
    AuthenticatedUser { user_id: _ }: AuthenticatedUser,
    Path(graph_id): Path<GraphId>,
    Json(payload): Json<CreateEdgeDataRequest>,
) -> impl IntoResponse {
    state
        .graph_service
        .insert_edge_data(graph_id, payload.into_domain())
        .await
        .map(|edge| (StatusCode::OK, Json(EdgeDataResponse::from(edge))))
}
