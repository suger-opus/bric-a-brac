use super::{
    handlers::{access_handler, graph_handler, user_handler},
    openapi,
    state::ApiState,
};
use axum::{
    http::header,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tower_http::{classify::ServerErrorsFailureClass, cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

async fn openapi_spec() -> Response {
    let json = openapi::get_openapi_json();
    ([(header::CONTENT_TYPE, "application/json")], json).into_response()
}

pub fn build(state: ApiState) -> Router {
    Router::new()
        .route("/openapi.json", get(openapi_spec))
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &axum::http::Request<axum::body::Body>| {
                    let request_id = Uuid::new_v4();
                    tracing::info_span!("http_request", id = ?request_id)
                })
                .on_request(
                    |request: &axum::http::Request<axum::body::Body>, span: &tracing::Span| {
                        tracing::info!(parent: span,
                            method = ?request.method(),
                            uri = ?request.uri(),
                            headers = ?request.headers(),
                            "Request received"
                        );
                    },
                )
                .on_response(
                    |response: &axum::http::Response<axum::body::Body>,
                     latency: std::time::Duration,
                     span: &tracing::Span| {
                        tracing::info!(parent: span,
                            status = ?response.status(),
                            latency = ?latency,
                            headers = ?response.headers(),
                            "Response sent"
                        );
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass,
                     latency: std::time::Duration,
                     span: &tracing::Span| {
                        tracing::error!(parent: span,
                            latency = ?latency,
                            error = ?error,
                            "Request failed"
                        );
                    },
                ),
        )
        .layer(CorsLayer::permissive())
        .with_state(state)
}
