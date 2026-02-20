use super::{
    http::{access_handler, graph_handler, user_handler},
    openapi::ApiDoc,
    state::ApiState,
    tracing::http_tracing_layer,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub fn build(state: ApiState) -> Router {
    let router = Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .route("/users", post(user_handler::create))
        .route("/users/me", get(user_handler::get_current))
        .route("/graphs", get(graph_handler::get_all_metadata))
        .route("/graphs/{graph_id}", get(graph_handler::get_metadata))
        .route("/graphs/{graph_id}/schema", get(graph_handler::get_schema))
        .route("/graphs/{graph_id}/data", get(graph_handler::get_data))
        .route("/graphs", post(graph_handler::create_graph))
        .route(
            "/graphs/{graph_id}/schema/generate",
            post(graph_handler::generate_schema),
        )
        .route(
            "/graphs/{graph_id}/schema/nodes",
            post(graph_handler::create_node_schema),
        )
        .route(
            "/graphs/{graph_id}/schema/edges",
            post(graph_handler::create_edge_schema),
        )
        .route(
            "/graphs/{graph_id}/data/nodes",
            post(graph_handler::insert_node_data),
        )
        .route(
            "/graphs/{graph_id}/data/edges",
            post(graph_handler::insert_edge_data),
        )
        .route("/accesses/graphs/{graph_id}", post(access_handler::create))
        .layer(http_tracing_layer());
    router.layer(CorsLayer::permissive()).with_state(state)
}
