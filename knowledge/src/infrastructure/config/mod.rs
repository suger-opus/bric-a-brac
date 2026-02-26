mod knowledge_db_config;
mod knowledge_server_config;

pub use knowledge_db_config::KnowledgeDatabaseConfig;
pub use knowledge_server_config::KnowledgeServerConfig;

use anyhow::Context;
use clap::Parser;

#[derive(Clone, clap::Parser, Debug)]
#[command(about = "Knowledge microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    knowledge_db: KnowledgeDatabaseConfig,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Config::try_parse().context("Failed to parse configuration")
    }

    pub fn knowledge_server(&self) -> &KnowledgeServerConfig {
        &self.knowledge_server
    }

    pub fn knowledge_db(&self) -> &KnowledgeDatabaseConfig {
        &self.knowledge_db
    }
}
