use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bric-à-brac API",
        version = "0.1.0",
        description = "Bric-à-brac REST API to manage graphs"
    ),
    paths(
        crate::presentation::http::user_handler::create,
        crate::presentation::http::user_handler::get_current,
        crate::presentation::http::graph_handler::get_all_metadata,
        crate::presentation::http::graph_handler::create_graph,
        crate::presentation::http::graph_handler::get_metadata,
        crate::presentation::http::graph_handler::get_schema,
        crate::presentation::http::graph_handler::create_schema,
        crate::presentation::http::graph_handler::generate_schema,
        crate::presentation::http::graph_handler::get_data,
        crate::presentation::http::graph_handler::insert_data,
        crate::presentation::http::graph_handler::generate_data,
        crate::presentation::http::access_handler::create,
    )
)]
pub struct ApiDoc;
