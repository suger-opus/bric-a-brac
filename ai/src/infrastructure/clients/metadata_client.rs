use crate::infrastructure::config::MetadataServerConfig;
use anyhow::Context;
use bric_a_brac_protos::metadata::{
    metadata_client::MetadataClient as MetadataGrpcClient, Empty, ValidateSchemaRequest,
    ValidateSchemaResponse,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MetadataClient {
    config: MetadataServerConfig,
    client: Arc<Mutex<Option<MetadataGrpcClient<tonic::transport::Channel>>>>,
}

impl MetadataClient {
    pub fn new(config: MetadataServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_openapi_spec(&self) -> anyhow::Result<serde_json::Value> {
        match self.try_get_openapi_spec().await {
            Ok(spec) => Ok(spec),
            Err(e) => {
                if self.is_connection_error(&e) {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_get_openapi_spec().await
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn validate_schema(
        &self,
        schema_json: String,
    ) -> anyhow::Result<ValidateSchemaResponse> {
        match self.try_validate_schema(schema_json.clone()).await {
            Ok(response) => Ok(response),
            Err(e) => {
                if self.is_connection_error(&e) {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_validate_schema(schema_json).await
                } else {
                    Err(e)
                }
            }
        }
    }

    async fn try_get_openapi_spec(&self) -> anyhow::Result<serde_json::Value> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let response = client
            .get_open_api_spec(Empty {})
            .await
            .context("Failed to fetch OpenAPI spec from Metadata service")?;

        let spec_str = response.into_inner().openapi_json;
        serde_json::from_str(&spec_str).context("Failed to parse OpenAPI spec JSON")
    }

    async fn try_validate_schema(
        &self,
        schema_json: String,
    ) -> anyhow::Result<ValidateSchemaResponse> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let response = client
            .validate_schema(ValidateSchemaRequest { schema_json })
            .await
            .context("Failed to validate schema with Metadata service")?;

        Ok(response.into_inner())
    }

    fn is_connection_error(&self, error: &anyhow::Error) -> bool {
        // Check if error chain contains a tonic::Status with connection-related code
        error.chain().any(|e| {
            if let Some(status) = e.downcast_ref::<tonic::Status>() {
                matches!(
                    status.code(),
                    tonic::Code::Unavailable
                        | tonic::Code::DeadlineExceeded
                        | tonic::Code::Cancelled
                        | tonic::Code::Unknown
                )
            } else {
                false
            }
        })
    }

    fn reset_connection(&self) {
        if let Ok(mut client_lock) = self.client.lock() {
            *client_lock = None;
            tracing::info!("Reset Metadata service connection");
        }
    }

    async fn ensure_connection(&self) -> anyhow::Result<()> {
        {
            let client_lock = self
                .client
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;

            if client_lock.is_some() {
                return Ok(());
            }
        } // Lock dropped here - Holding mutex guards across .await is an anti-pattern

        tracing::info!("Connecting to Metadata service at {}", self.config.url());

        let client = MetadataGrpcClient::connect(self.config.url().clone())
            .await
            .context("Failed to connect to Metadata service")?;

        {
            let mut client_lock = self
                .client
                .lock()
                .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;

            *client_lock = Some(client);
        }

        tracing::info!(
            "Successfully connected to Metadata service at {}",
            self.config.url()
        );

        Ok(())
    }

    fn clone_client(&self) -> anyhow::Result<MetadataGrpcClient<tonic::transport::Channel>> {
        let client_lock = self
            .client
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;

        client_lock
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not connected"))
            .map(|client| client.clone()) // Clone the client to avoid holding the lock across await
    }
}
