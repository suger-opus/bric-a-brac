mod ai_server_config;
mod knowledge_server_config;
mod metadata_server_config;
mod openrouter_config;

pub use ai_server_config::AiServerConfig;
pub use knowledge_server_config::KnowledgeServerConfig;
pub use metadata_server_config::MetadataServerConfig;
pub use openrouter_config::OpenRouterConfig;

use anyhow::Context;
use clap::Parser;
use secrecy::SecretString;

#[derive(clap::Parser, derive_more::Debug)]
#[command(about = "AI microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    ai_server: AiServerConfig,

    #[clap(flatten)]
    metadata_server: MetadataServerConfig,

    #[clap(flatten)]
    knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    openrouter: OpenRouterConfig,

    /// Shared secret for inter-service gRPC authentication
    #[arg(long, env = "INTERNAL_SERVICES_AUTH_TOKEN", required = true)]
    internal_services_auth_token: SecretString,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Self::try_parse().context("Failed to parse configuration")
    }

    pub const fn ai_server(&self) -> &AiServerConfig {
        &self.ai_server
    }

    pub const fn metadata_server(&self) -> &MetadataServerConfig {
        &self.metadata_server
    }

    pub const fn knowledge_server(&self) -> &KnowledgeServerConfig {
        &self.knowledge_server
    }

    pub const fn openrouter(&self) -> &OpenRouterConfig {
        &self.openrouter
    }

    pub const fn internal_services_auth_token(&self) -> &SecretString {
        &self.internal_services_auth_token
    }
}
