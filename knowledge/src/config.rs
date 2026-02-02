mod knowledge_db;
mod knowledge_server;

use crate::config::knowledge_db::KnowledgeDatabaseConfig;
use crate::config::knowledge_server::KnowledgeServerConfig;
use anyhow::Context;
use clap::Parser;

#[derive(Clone, clap::Parser, Debug)]
#[command(about = "Knowledge microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    pub knowledge_server: KnowledgeServerConfig,

    #[clap(flatten)]
    pub knowledge_db: KnowledgeDatabaseConfig,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Config::try_parse().context("Failed to parse configuration")
    }
}
