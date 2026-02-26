use crate::{
    application::services::{DataService, SchemaService},
    presentation::errors::AppError,
};
use bric_a_brac_protos::{
    ai::{ai_server::Ai, GenerateDataRequest, GenerateSchemaRequest},
    common::{CreateGraphDataProto, CreateGraphSchemaProto},
};
use tonic::{Request, Response, Status};

pub struct AiService {
    schema_service: SchemaService,
    data_service: DataService,
}

impl AiService {
    pub fn new(schema_service: SchemaService, data_service: DataService) -> Self {
        Self {
            schema_service,
            data_service,
        }
    }
}

#[tonic::async_trait]
impl Ai for AiService {
    #[tracing::instrument(level = "debug", skip(self, request))]
    async fn generate_schema(
        &self,
        request: Request<GenerateSchemaRequest>,
    ) -> Result<Response<CreateGraphSchemaProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(
            file_size = req.file_content.len(),
            file_type = ?req.file_type,
        );

        let schema = self
            .schema_service
            .generate_schema(req.file_content)
            .await?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "debug", skip(self, request))]
    async fn generate_data(
        &self,
        request: Request<GenerateDataRequest>,
    ) -> Result<Response<CreateGraphDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(
            file_size = req.file_content.len(),
            file_type = ?req.file_type,
        );

        let data = self
            .data_service
            .generate_data(
                req.schema.try_into().map_err(|err| AppError::from(err))?,
                req.file_content,
            )
            .await?;

        Ok(Response::new(data.into()))
    }
}
