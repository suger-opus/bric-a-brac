use anyhow::Context;
use crate::grpc_client::KnowledgeClient;
use axum::http::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct KnowledgeServerConfig {
    /// Knowledge server URL
    #[arg(long, env = "KNOWLEDGE_SERVER_URL", required = true)]
    knowledge_server_url: Uri,
}

impl KnowledgeServerConfig {
    // #[tracing::instrument(skip(self), fields(url = %self.knowledge_server_url))]
    pub async fn connect(&self) -> anyhow::Result<KnowledgeClient> {
        tracing::debug!("Connecting to Knowledge service");
        
        let knowledge_client = KnowledgeClient::connect(self.knowledge_server_url.clone())
            .await
            .context("Failed to connect to Knowledge service")?;

        tracing::debug!("Knowledge service connection established");
        Ok(knowledge_client)
    }
}
