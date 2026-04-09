mod knowledge_db_config;
mod knowledge_server_config;

pub use knowledge_db_config::KnowledgeDatabaseConfig;
pub use knowledge_server_config::KnowledgeServerConfig;

use anyhow::Context;
use clap::Parser;
use secrecy::SecretString;

#[derive(Clone, clap::Parser, Debug)]
#[command(about = "Knowledge microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    knowledge_db: KnowledgeDatabaseConfig,

    /// Shared secret for inter-service gRPC authentication
    #[arg(long, env = "INTERNAL_SERVICES_AUTH_TOKEN", required = true)]
    internal_services_auth_token: SecretString,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Self::try_parse().context("Failed to parse configuration")
    }

    pub const fn knowledge_server(&self) -> &KnowledgeServerConfig {
        &self.knowledge_server
    }

    pub const fn knowledge_db(&self) -> &KnowledgeDatabaseConfig {
        &self.knowledge_db
    }

    pub const fn internal_services_auth_token(&self) -> &SecretString {
        &self.internal_services_auth_token
    }
}
