use crate::application::dtos::{
    CreateEdgeSchemaDto, CreateGraphSchemaDto, CreateNodeSchemaDto, CreatePropertySchemaDto,
    PropertyMetadataDto, PropertyTypeDto,
};
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
        crate::presentation::http::graph_handler::get_metadata,
        crate::presentation::http::graph_handler::get_schema,
        crate::presentation::http::graph_handler::get_data,
        crate::presentation::http::graph_handler::create_graph,
        crate::presentation::http::graph_handler::generate_schema,
        crate::presentation::http::graph_handler::create_node_schema,
        crate::presentation::http::graph_handler::create_edge_schema,
        crate::presentation::http::graph_handler::insert_node_data,
        crate::presentation::http::graph_handler::insert_edge_data,
        crate::presentation::http::access_handler::create,
    )
)]
pub struct ApiDoc;

#[derive(OpenApi)]
#[openapi(components(schemas(
    CreateGraphSchemaDto,
    CreateNodeSchemaDto,
    CreateEdgeSchemaDto,
    CreatePropertySchemaDto,
    PropertyTypeDto,
    PropertyMetadataDto,
)))]
pub struct GenerateSchemaApiDoc;

// TODO: remove expect and handle error properly
// + edit tests
pub fn get_openapi_generate_schema_doc() -> String {
    GenerateSchemaApiDoc::openapi()
        .to_json()
        .expect("Failed to serialize OpenAPI spec")
}
