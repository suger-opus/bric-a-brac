use crate::application::services::SchemaService;
use anyhow::Context;
use bric_a_brac_protos::ai::{ai_server::Ai, GenerateSchemaRequest, GenerateSchemaResponse};
use tonic::{Request, Response, Status};

pub struct AiService {
    schema_service: SchemaService,
}

impl AiService {
    pub fn new(schema_service: SchemaService) -> Self {
        Self { schema_service }
    }
}

#[tonic::async_trait]
impl Ai for AiService {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn generate_schema(
        &self,
        request: Request<GenerateSchemaRequest>,
    ) -> Result<Response<GenerateSchemaResponse>, Status> {
        let req = request.into_inner();
        tracing::info!(
            file_size = req.file_content.len(),
            file_type = ?req.file_type,
            "Received schema generation request"
        );

        match self.schema_service.generate_schema(&req.file_content).await {
            Ok(schema) => {
                let schema_json = serde_json::to_string(&schema)
                    .context("Failed to serialize schema")
                    .map_err(|e| Status::internal(e.to_string()))?;

                tracing::info!("Schema generated successfully");
                Ok(Response::new(GenerateSchemaResponse { schema_json }))
            }
            Err(e) => {
                tracing::error!(error = ?e, "Schema generation failed");
                Err(Status::internal(format!("Schema generation failed: {}", e)))
            }
        }
    }
}
