use crate::{
    domain::models::CreateGraphSchema,
    infrastructure::config::AiServerConfig,
    presentation::error::{AppError, DomainError, InfraError, ResultExt},
};
use bric_a_brac_protos::ai::{
    ai_client::AiClient as AiGrpcClient, FileType, GenerateSchemaRequest,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AiClient {
    config: AiServerConfig,
    client: Arc<Mutex<Option<AiGrpcClient<tonic::transport::Channel>>>>,
}

impl AiClient {
    pub fn new(config: AiServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn generate_schema(
        &self,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchema, AppError> {
        match self
            .try_generate_schema(file_content.clone(), file_type.clone())
            .await
        {
            Ok(schema) => Ok(schema),
            Err(e) => {
                if e.is_grpc_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_generate_schema(file_content, file_type).await
                } else {
                    Err(e)
                }
            }
        }
    }

    async fn try_generate_schema(
        &self,
        file_content: Vec<u8>,
        file_type: String,
    ) -> Result<CreateGraphSchema, AppError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let file_type = match file_type.to_lowercase().as_str() {
            "csv" => FileType::Csv,
            "txt" => FileType::Txt,
            _ => {
                return Err(AppError::Domain(DomainError::InvalidInput {
                    reason: format!("Unsupported file type: {}", file_type),
                }))
            }
        };
        let request = GenerateSchemaRequest {
            file_content,
            file_type: file_type as i32,
        };

        let response = client
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

    fn reset_connection(&self) {
        if let Ok(mut client_lock) = self.client.lock() {
            *client_lock = None;
            tracing::info!("Reset AI service connection");
        }
    }

    async fn ensure_connection(&self) -> Result<(), AppError> {
        {
            let client_lock = self.client.lock().map_err(|e| {
                AppError::Infra(InfraError::MutexPoisoned {
                    message: format!("Failed to acquire lock: {}", e),
                })
            })?;

            if client_lock.is_some() {
                return Ok(());
            }
        } // Lock dropped here - Holding mutex guards across .await is an anti-pattern

        tracing::info!("Connecting to Ai service at {}", self.config.url());

        let client = AiGrpcClient::connect(self.config.url().clone())
            .await
            .map_err(|e| {
                AppError::Infra(InfraError::GrpcService {
                    service: "ai".to_string(),
                    message: format!("Failed to connect to Ai service: {}", e),
                    source: None,
                })
            })?;

        {
            let mut client_lock = self.client.lock().map_err(|e| {
                AppError::Infra(InfraError::MutexPoisoned {
                    message: format!("Failed to acquire lock: {}", e),
                })
            })?;

            *client_lock = Some(client);
        }

        tracing::info!(
            "Successfully connected to Ai service at {}",
            self.config.url()
        );

        Ok(())
    }

    fn clone_client(&self) -> Result<AiGrpcClient<tonic::transport::Channel>, AppError> {
        let client_lock = self.client.lock().map_err(|e| {
            AppError::Infra(InfraError::MutexPoisoned {
                message: format!("Failed to acquire lock: {}", e),
            })
        })?;

        client_lock
            .as_ref()
            .ok_or_else(|| {
                AppError::Infra(InfraError::ClientNotConnected {
                    message: "Client not connected".to_string(),
                })
            })
            .map(|client| client.clone()) // Clone the client to avoid holding the lock across await
    }
}
