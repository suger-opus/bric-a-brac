use crate::{
    CreateEdgeSchemaDto, CreateGraphSchemaDto, CreateNodeSchemaDto, CreatePropertySchemaDto,
    PropertyMetadataDto, PropertyTypeDto,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(
    CreateGraphSchemaDto,
    CreateNodeSchemaDto,
    CreateEdgeSchemaDto,
    CreatePropertySchemaDto,
    PropertyTypeDto,
    PropertyMetadataDto,
)))]
struct GenerateGraphSchemaDoc;

pub fn generate_graph_schema_doc() -> serde_json::Value {
    match GenerateGraphSchemaDoc::openapi().to_json() {
        Ok(json_str) => match serde_json::from_str::<serde_json::Value>(&json_str) {
            Ok(json) => json,
            Err(_) => serde_json::json!({}),
        },
        Err(_) => serde_json::json!({}),
    }
}
