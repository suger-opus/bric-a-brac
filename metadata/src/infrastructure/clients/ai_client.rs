use crate::{
    application::dtos::CreateGraphSchemaDto, infrastructure::config::AiServerConfig, presentation::errors::GrpcClientError
};
use axum::http::Uri;
use bric_a_brac_protos::{
    ai::{ai_client::AiClient as AiGrpcClient, FileType, GenerateSchemaRequest},
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AiClient {
    config: AiServerConfig,
    client: Arc<Mutex<Option<AiGrpcClient<tonic::transport::Channel>>>>,
}

#[tonic::async_trait]
impl GrpcClient for AiClient {
    type Client = AiGrpcClient<tonic::transport::Channel>;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>> {
        &self.client
    }

    fn service_kind(&self) -> GrpcServiceKind {
        GrpcServiceKind::Ai
    }

    fn url(&self) -> &Uri {
        self.config.url()
    }

    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error> {
        AiGrpcClient::connect(self.url().clone()).await
    }
}

impl AiClient {
    pub fn new(config: AiServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    #[tracing::instrument(level = "debug", skip(self, file_content, file_type))]
    pub async fn generate_schema(
        &self,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchemaDto, GrpcClientError> {
        tracing::debug!(file_content_size = file_content.len(), file_type = %file_type);

        let schema = match self
            .try_generate_schema(file_content.clone(), file_type.clone())
            .await
        {
            Ok(schema) => Ok(schema),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!(error = %err, "Connection error detected, reconnecting");
                    self.reset_connection();
                    self.try_generate_schema(file_content, file_type).await
                } else {
                    Err(err)
                }
            }
        }?;

        Ok(schema)
    }

    #[tracing::instrument(level = "trace", skip(self, file_content, file_type))]
    async fn try_generate_schema(
        &self,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchemaDto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let file_type = match file_type.to_lowercase().as_str() {
            "csv" => FileType::Csv,
            _ => FileType::Txt,
        };

        let request = GenerateSchemaRequest {
            file_content,
            file_type: file_type as i32,
        };
        let response =
            client
                .generate_schema(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Ai,
                    message: "Failed to generate schema using AI service".to_string(),
                    source: err,
                })?;
        let schema_serialize = response.into_inner().schema_json;

        let schema =
            serde_json::from_str::<CreateGraphSchemaDto>(&schema_serialize).map_err(|err| {
                GrpcClientError::Deserialization {
                    service: GrpcServiceKind::Ai,
                    expected_struct: "CreateGraphSchema".to_string(),
                    source: err,
                }
            })?;

        Ok(schema)
    }
}
