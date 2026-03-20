mod ai_server_config;
mod knowledge_server_config;
mod metadata_db_config;
mod metadata_server_config;

pub use ai_server_config::AiServerConfig;
pub use knowledge_server_config::KnowledgeServerConfig;
pub use metadata_db_config::MetadataDatabaseConfig;
pub use metadata_server_config::MetadataServerConfig;

use anyhow::Context;
use clap::Parser;

#[derive(clap::Parser, derive_more::Debug)]
#[command(about = "Metadata microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    metadata_server: MetadataServerConfig,

    #[clap(flatten)]
    knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    ai_server: AiServerConfig,

    #[clap(flatten)]
    metadata_db: MetadataDatabaseConfig,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Config::try_parse().context("Failed to parse configuration")
    }

    pub fn metadata_server(&self) -> &MetadataServerConfig {
        &self.metadata_server
    }

    pub fn knowledge_server(&self) -> &KnowledgeServerConfig {
        &self.knowledge_server
    }

    pub fn ai_server(&self) -> &AiServerConfig {
        &self.ai_server
    }

    pub fn metadata_db(&self) -> &MetadataDatabaseConfig {
        &self.metadata_db
    }
}
