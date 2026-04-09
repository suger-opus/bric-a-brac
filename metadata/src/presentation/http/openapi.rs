#![allow(clippy::needless_for_each)]
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bric-à-brac API",
        version = "0.1.0",
        description = "Bric-à-brac REST API to manage graphs"
    ),
    paths(
        super::handlers::user_handler::create,
        super::handlers::user_handler::get_current,
        super::handlers::graph_handler::list,
        super::handlers::graph_handler::create,
        super::handlers::graph_handler::get,
        super::handlers::graph_handler::get_schema,
        super::handlers::graph_handler::get_data,
        super::handlers::access_handler::create,
        super::handlers::session_handler::create,
        super::handlers::session_handler::list,
        super::handlers::session_handler::close,
        super::handlers::session_handler::get_messages,
    )
)]
pub struct ApiDoc;
