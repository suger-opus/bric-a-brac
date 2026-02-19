use crate::{infrastructure::config::MetadataServerConfig, presentation::errors::GrpcClientError};
use bric_a_brac_protos::{
    metadata::{
        metadata_client::MetadataClient as MetadataGrpcClient, Empty, ValidateSchemaRequest,
        ValidateSchemaResponse,
    },
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use http::Uri;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MetadataClient {
    config: MetadataServerConfig,
    client: Arc<Mutex<Option<MetadataGrpcClient<tonic::transport::Channel>>>>,
}

#[tonic::async_trait]
impl GrpcClient for MetadataClient {
    type Client = MetadataGrpcClient<tonic::transport::Channel>;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>> {
        &self.client
    }

    fn service_kind(&self) -> GrpcServiceKind {
        GrpcServiceKind::Metadata
    }

    fn url(&self) -> &Uri {
        self.config.url()
    }

    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error> {
        MetadataGrpcClient::connect(self.url().clone()).await
    }
}

impl MetadataClient {
    pub fn new(config: MetadataServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_openapi_spec(&self) -> Result<serde_json::Value, GrpcClientError> {
        match self.try_get_openapi_spec().await {
            Ok(spec) => Ok(spec),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", err);
                    self.reset_connection();
                    self.try_get_openapi_spec().await
                } else {
                    Err(err)
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self, schema_json))]
    pub async fn validate_schema(
        &self,
        schema_json: String,
    ) -> Result<ValidateSchemaResponse, GrpcClientError> {
        tracing::debug!(schema_json = %schema_json);

        match self.try_validate_schema(schema_json.clone()).await {
            Ok(response) => Ok(response),
            Err(err) => {
                if err.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", err);
                    self.reset_connection();
                    self.try_validate_schema(schema_json).await
                } else {
                    Err(err)
                }
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn try_get_openapi_spec(&self) -> Result<serde_json::Value, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let response = client.get_open_api_spec(Empty {}).await.map_err(|err| {
            BaseGrpcClientError::Request {
                message: "Failed to get OpenAPI spec from Metadata service".to_string(),
                service: self.service_kind(),
                source: err,
            }
        })?;

        let spec_str = response.into_inner().openapi_json;
        serde_json::from_str(&spec_str).map_err(|err| GrpcClientError::Deserialization {
            service: self.service_kind(),
            expected_struct: "OpenAPI Spec".to_string(),
            source: err,
        })
    }

    #[tracing::instrument(level = "trace", skip(self, schema_json))]
    async fn try_validate_schema(
        &self,
        schema_json: String,
    ) -> Result<ValidateSchemaResponse, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let response = client
            .validate_schema(ValidateSchemaRequest { schema_json })
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                message: "Failed to validate schema with Metadata service".to_string(),
                service: self.service_kind(),
                source: err,
            })?;

        Ok(response.into_inner())
    }
}
