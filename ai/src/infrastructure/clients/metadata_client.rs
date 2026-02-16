use crate::infrastructure::config::MetadataServerConfig;
use anyhow::Context;
use bric_a_brac_protos::metadata::{
    metadata_client::MetadataClient as MetadataGrpcClient, Empty, ValidateSchemaRequest,
    ValidateSchemaResponse,
};

#[derive(Clone)]
pub struct MetadataClient {
    client: MetadataGrpcClient<tonic::transport::Channel>,
}

impl MetadataClient {
    pub async fn connect(config: &MetadataServerConfig) -> anyhow::Result<Self> {
        let client = MetadataGrpcClient::connect(config.url().clone())
            .await
            .context("Failed to connect to Metadata service")?;

        Ok(Self { client })
    }

    pub async fn get_openapi_spec(&mut self) -> anyhow::Result<serde_json::Value> {
        let response = self
            .client
            .get_open_api_spec(Empty {})
            .await
            .context("Failed to fetch OpenAPI spec from Metadata service")?;

        let spec_str = response.into_inner().openapi_json;
        serde_json::from_str(&spec_str).context("Failed to parse OpenAPI spec JSON")
    }

    pub async fn validate_schema(
        &mut self,
        schema_json: String,
    ) -> anyhow::Result<ValidateSchemaResponse> {
        let response = self
            .client
            .validate_schema(ValidateSchemaRequest { schema_json })
            .await
            .context("Failed to validate schema with Metadata service")?;

        Ok(response.into_inner())
    }
}
