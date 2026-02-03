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
        .route("/graphs", post(graph_handler::post))
        .route("/graphs/{graph_id}", get(graph_handler::get_one_metadata))
        .route("/graphs/filter", get(graph_handler::get_all_metadata))
        .route("/graphs/{graph_id}/accesses", post(access_handler::post))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
