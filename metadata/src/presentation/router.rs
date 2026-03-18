use super::{
    http::{access_handler, graph_handler, user_handler},
    openapi::ApiDoc,
    state::ApiState,
    tracing::http_tracing_layer,
};
use axum::{
    routing::{get, post},
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
                match ApiDoc::openapi().to_json() {
                    Ok(json_str) => match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(json) => Json(json),
                        Err(_) => Json(serde_json::json!({})),
                    },
                    Err(_) => Json(serde_json::json!({})),
                }
            }),
        )
        .route("/users", post(user_handler::create))
        .route("/users/me", get(user_handler::get_current))
        .route("/graphs", get(graph_handler::get_all_metadata))
        .route("/graphs", post(graph_handler::create_graph))
        .route("/graphs/{graph_id}", get(graph_handler::get_metadata))
        .route("/graphs/{graph_id}/schema", get(graph_handler::get_schema))
        .route("/graphs/{graph_id}/data", get(graph_handler::get_data))
        .route("/accesses/graphs/{graph_id}", post(access_handler::create))
        .layer(http_tracing_layer());
    router.layer(CorsLayer::permissive()).with_state(state)
}
