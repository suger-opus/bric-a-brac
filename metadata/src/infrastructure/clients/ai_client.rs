use crate::{
    domain::models::CreateGraphSchema,
    infrastructure::config::AiServerConfig,
    presentation::error::{AppError, DomainError, InfraError, ResultExt},
};
use bric_a_brac_protos::ai::{
    ai_client::AiClient as AiGrpcClient, FileType, GenerateSchemaRequest,
};

#[derive(Clone)]
pub struct AiClient {
    client: AiGrpcClient<tonic::transport::Channel>,
}

impl AiClient {
    pub async fn connect(config: &AiServerConfig) -> Result<Self, AppError> {
        let client = AiGrpcClient::connect(config.url().clone())
            .await
            .map_err(AppError::from)
            .context("Failed to connect to Ai service")?;

        Ok(Self { client })
    }

    pub async fn generate_schema(
        &self,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchema, AppError> {
        let file_type = match file_type.to_lowercase().as_str() {
            "csv" => FileType::Csv,
            "txt" => FileType::Txt,
            _ => {
                return Err(AppError::Domain(
                    DomainError::InvalidInput {
                        reason: format!("Unsupported file type: {}", file_type),
                    },
                ))
            }
        };
        let request = GenerateSchemaRequest {
            file_content,
            file_type: file_type as i32,
        };

        let response = self
            .client
            .clone()
            .generate_schema(request)
            .await
            .context("Failed to generate schema in Ai service")?;
        let schema_serialize = response.into_inner().schema_json;

        let schema = serde_json::from_str::<CreateGraphSchema>(&schema_serialize).map_err(|e| {
            InfraError::GrpcService {
                service: "ai".to_string(),
                message: format!("Failed to parse schema JSON: {}", e),
                source: None,
            }
        })?;

        Ok(schema)
    }
}
