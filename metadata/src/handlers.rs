use axum::{extract::State, Json};

use crate::conversions::dto_properties_to_proto;
use crate::dto::{CreateEdgeRequest, CreateNodeRequest, GraphDataResponse, SearchRequest};
use crate::error::AppError;
use crate::state::AppState;

pub async fn create_node(
    State(state): State<AppState>,
    Json(req): Json<CreateNodeRequest>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let mut client = state.knowledge_client.clone();

    let proto_props = dto_properties_to_proto(req.properties);

    let result = client
        .insert_node(req.graph_id, req.label, proto_props)
        .await?;

    Ok(Json(GraphDataResponse::from(result)))
}

pub async fn create_edge(
    State(state): State<AppState>,
    Json(req): Json<CreateEdgeRequest>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let mut client = state.knowledge_client.clone();

    let proto_props = dto_properties_to_proto(req.properties);

    let result = client
        .insert_edge(req.from_id, req.to_id, req.label, proto_props)
        .await?;

    Ok(Json(GraphDataResponse::from(result)))
}

pub async fn search_graph(
    State(state): State<AppState>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<GraphDataResponse>, AppError> {
    let mut client = state.knowledge_client.clone();

    let node_props = req
        .node_properties
        .map(dto_properties_to_proto)
        .unwrap_or_default();

    let edge_props = req
        .edge_properties
        .map(dto_properties_to_proto)
        .unwrap_or_default();

    let result = client
        .search(
            req.graph_id,
            req.node_label,
            node_props,
            req.edge_label,
            edge_props,
            req.include_edges,
        )
        .await?;

    Ok(Json(GraphDataResponse::from(result)))
}
