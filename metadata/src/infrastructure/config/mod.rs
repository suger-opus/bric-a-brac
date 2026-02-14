mod knowledge_server_config;
mod metadata_db_config;
mod metadata_server_config;

pub use knowledge_server_config::KnowledgeServerConfig;
pub use metadata_db_config::MetadataDatabaseConfig;
pub use metadata_server_config::MetadataServerConfig;

use anyhow::Context;
use clap::Parser;

#[derive(clap::Parser, derive_more::Debug)]
#[command(about = "Metadata microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    pub metadata_server: MetadataServerConfig,

    #[clap(flatten)]
    pub knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    pub metadata_db: MetadataDatabaseConfig,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Config::try_parse().context("Failed to parse configuration")
    }
}
