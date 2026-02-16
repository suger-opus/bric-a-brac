use crate::application::dtos::{
    CreateEdgeSchemaDto, CreateNodeSchemaDto, CreatePropertySchemaDto, PropertyMetadataDto,
    PropertyTypeDto,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Metadata API",
        version = "0.1.0",
        description = "Graph metadata microservice for managing schemas and data"
    ),
    components(schemas(
        CreateNodeSchemaDto,
        CreateEdgeSchemaDto,
        CreatePropertySchemaDto,
        PropertyTypeDto,
        PropertyMetadataDto,
    ))
)]
pub struct ApiDoc;

/// Get the OpenAPI specification as JSON
pub fn get_openapi_json() -> String {
    ApiDoc::openapi().to_json().expect("Failed to serialize OpenAPI spec")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_generation() {
        let json = get_openapi_json();
        assert!(json.contains("CreateNodeSchemaDto"));
        assert!(json.contains("CreateEdgeSchemaDto"));
        assert!(json.contains("CreatePropertySchemaDto"));
    }
}
