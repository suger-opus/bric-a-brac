use crate::application::services::SchemaService;
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
    #[tracing::instrument(level = "debug", skip(self, request))]
    async fn generate_schema(
        &self,
        request: Request<GenerateSchemaRequest>,
    ) -> Result<Response<GenerateSchemaResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(
            file_size = req.file_content.len(),
            file_type = ?req.file_type,
        );

        let schema = self
            .schema_service
            .generate_schema(&req.file_content)
            .await?;

        Ok(Response::new(GenerateSchemaResponse {
            schema_json: schema,
        }))
    }
}
