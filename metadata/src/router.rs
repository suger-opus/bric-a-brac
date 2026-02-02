use crate::handlers::{create_edge, create_node, search_graph};
use crate::state::ApiState;
use axum::{routing::post, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::Level;

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/nodes", post(create_node))
        .route("/edges", post(create_edge))
        .route("/search", post(search_graph))
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
