use crate::handlers::{graph_handler, user_handler};
use crate::state::ApiState;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Level;

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/users", post(user_handler::post))
        .route("/users", get(user_handler::get))
        .route("/graphs", post(graph_handler::post))
        .route("/graphs/graph_id", get(graph_handler::get_one_metadata))
        .route("/graphs/filter", get(graph_handler::get_all_metadata))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri().path(),
                        version = ?request.version(),
                        request_id = %request_id,
                    )
                })
                .on_response(
                    tower_http::trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                        .include_headers(false),
                ),
        )
        .with_state(state)
}
