use crate::handlers::{access_handler, graph_handler, user_handler};
use crate::state::ApiState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/users", post(user_handler::post))
        .route("/users", get(user_handler::get))
        .route("/graphs", get(graph_handler::get_all_metadata))
        .route("/graphs/{graph_id}", get(graph_handler::get_metadata))
        .route("/graphs/{graph_id}/schema", get(graph_handler::get_schema))
        .route("/graphs", post(graph_handler::post))
        .route(
            "/graphs/{graph_id}/schema/nodes",
            post(graph_handler::post_node_schema),
        )
        .route(
            "/graphs/{graph_id}/schema/edges",
            post(graph_handler::post_edge_schema),
        )
        .route("/accesses/graphs/{graph_id}", post(access_handler::post))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
