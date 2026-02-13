use crate::handlers;
use crate::state::ApiState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/users", post(handlers::user::create))
        .route("/users/me", get(handlers::user::get_current))
        .route("/graphs", get(handlers::graph::get_all_metadata))
        .route("/graphs/{graph_id}", get(handlers::graph::get_metadata))
        .route(
            "/graphs/{graph_id}/schema",
            get(handlers::graph::get_schema),
        )
        .route("/graphs/{graph_id}/data", get(handlers::graph::get_data))
        .route("/graphs", post(handlers::graph::create_graph))
        .route(
            "/graphs/{graph_id}/schema/nodes",
            post(handlers::graph::create_node_schema),
        )
        .route(
            "/graphs/{graph_id}/schema/edges",
            post(handlers::graph::create_edge_schema),
        )
        .route(
            "/graphs/{graph_id}/data/nodes",
            post(handlers::graph::insert_node_data),
        )
        .route(
            "/graphs/{graph_id}/data/edges",
            post(handlers::graph::insert_edge_data),
        )
        .route(
            "/accesses/graphs/{graph_id}",
            post(handlers::access::create),
        )
        .layer(CorsLayer::permissive())
        .with_state(state)
}
