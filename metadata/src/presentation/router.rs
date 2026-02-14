use super::{
    handlers::{access_handler, graph_handler, user_handler},
    state::ApiState,
};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/users", post(user_handler::create))
        .route("/users/me", get(user_handler::get_current))
        .route("/graphs", get(graph_handler::get_all_metadata))
        .route("/graphs/{graph_id}", get(graph_handler::get_metadata))
        .route("/graphs/{graph_id}/schema", get(graph_handler::get_schema))
        .route("/graphs/{graph_id}/data", get(graph_handler::get_data))
        .route("/graphs", post(graph_handler::create_graph))
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
        .layer(CorsLayer::permissive())
        .with_state(state)
}
