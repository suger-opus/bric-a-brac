use crate::{application::dtos::CreateGraphSchemaDto, presentation::openapi};
use bric_a_brac_protos::metadata::{
    metadata_server::Metadata, Empty, FormatLabelRequest, FormatLabelResponse, OpenApiSpecResponse,
    ValidateSchemaRequest, ValidateSchemaResponse, ValidationError,
};
use tonic::{Request, Response, Status};
use validator::Validate;

#[derive(Debug)]
pub struct MetadataService;

impl MetadataService {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl Metadata for MetadataService {
    #[tracing::instrument(level = "debug", skip(self, _request))]
    async fn get_open_api_spec(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<OpenApiSpecResponse>, Status> {
        let openapi_json = openapi::get_openapi_generate_schema_doc();
        Ok(Response::new(OpenApiSpecResponse { openapi_json }))
    }

    #[tracing::instrument(level = "debug", skip(self, request))]
    async fn validate_schema(
        &self,
        request: Request<ValidateSchemaRequest>,
    ) -> Result<Response<ValidateSchemaResponse>, Status> {
        let schema_json = &request.into_inner().schema_json;

        match serde_json::from_str::<CreateGraphSchemaDto>(schema_json) {
            Ok(dto) => match dto.validate() {
                Ok(_) => Ok(Response::new(ValidateSchemaResponse {
                    is_valid: true,
                    errors: vec![],
                })),
                Err(validation_errors) => {
                    let errors = validation_errors
                        .field_errors()
                        .iter()
                        .map(|(field, errs)| {
                            errs.iter().map(|err| ValidationError {
                                field: field.to_string(),
                                message: format!(
                                    "{}",
                                    err.message
                                        .clone()
                                        .unwrap_or_else(|| "Validation error".into())
                                ),
                            })
                        })
                        .flatten()
                        .collect();
                    Ok(Response::new(ValidateSchemaResponse {
                        is_valid: false,
                        errors,
                    }))
                }
            },
            Err(e) => Ok(Response::new(ValidateSchemaResponse {
                is_valid: false,
                errors: vec![ValidationError {
                    field: "schema".to_string(),
                    message: format!("Invalid JSON: {}", e),
                }],
            })),
        }
    }

    #[tracing::instrument(level = "debug", skip(self, request))]
    async fn format_label(
        &self,
        request: Request<FormatLabelRequest>,
    ) -> Result<Response<FormatLabelResponse>, Status> {
        let label = request.into_inner().label;

        // Apply business rules for label formatting
        // Convert to lowercase, replace spaces with underscores, remove special chars
        let formatted = label
            .to_lowercase()
            .replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect();

        Ok(Response::new(FormatLabelResponse {
            formatted_label: formatted,
        }))
    }
}
