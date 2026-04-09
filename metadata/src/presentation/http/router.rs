use super::{
    handlers::{access_handler, chat_handler, graph_handler, session_handler, user_handler},
    http_tracing_layer, ApiDoc, ApiState,
};
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub fn build(state: ApiState) -> Router {
    let router = Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .route(
            "/docs/openapi.json",
            get(|| async {
                ApiDoc::openapi().to_json().map_or_else(
                    |_| Json(serde_json::json!({})),
                    |json_str| {
                        serde_json::from_str::<serde_json::Value>(&json_str)
                            .map_or_else(|_| Json(serde_json::json!({})), Json)
                    },
                )
            }),
        )
        .route("/users", post(user_handler::create))
        .route("/users/me", get(user_handler::get_current))
        .route("/graphs", get(graph_handler::list))
        .route("/graphs", post(graph_handler::create))
        .route("/graphs/{graph_id}", get(graph_handler::get))
        .route("/graphs/{graph_id}", delete(graph_handler::delete))
        .route("/graphs/{graph_id}/schema", get(graph_handler::get_schema))
        .route("/graphs/{graph_id}/data", get(graph_handler::get_data))
        .route("/graphs/{graph_id}/sessions", post(session_handler::create))
        .route("/graphs/{graph_id}/sessions", get(session_handler::list))
        .route(
            "/graphs/{graph_id}/chat",
            post(chat_handler::chat).layer(DefaultBodyLimit::max(50 * 1024 * 1024)),
        )
        .route("/sessions/{session_id}/close", post(session_handler::close))
        .route(
            "/sessions/{session_id}/messages",
            get(session_handler::get_messages),
        )
        .route("/graphs/{graph_id}/accesses", post(access_handler::create))
        .layer(http_tracing_layer());
    router.layer(CorsLayer::permissive()).with_state(state)
}
