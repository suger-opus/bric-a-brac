mod knowledge_server;
mod metadata_db;
mod metadata_server;

use crate::config::knowledge_server::KnowledgeServerConfig;
use crate::config::metadata_db::MetadataDatabaseConfig;
use crate::config::metadata_server::MetadataServerConfig;
use anyhow::Context;
use clap::Parser;

#[derive(Clone, clap::Parser, derive_more::Debug)]
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
