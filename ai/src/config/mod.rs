mod ai_server;
mod metadata_server;
mod openrouter;

pub use ai_server::*;
pub use metadata_server::*;
pub use openrouter::*;

use anyhow::Context;
use clap::Parser;

#[derive(clap::Parser, derive_more::Debug)]
#[command(about = "AI microservice configuration", long_about = None)]
pub struct Config {
    #[clap(flatten)]
    ai_server: AiServerConfig,

    #[clap(flatten)]
    metadata_server: MetadataServerConfig,

    #[clap(flatten)]
    openrouter: OpenRouterConfig,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Config::try_parse().context("Failed to parse configuration")
    }

    pub fn ai_server(&self) -> &AiServerConfig {
        &self.ai_server
    }

    pub fn metadata_server(&self) -> &MetadataServerConfig {
        &self.metadata_server
    }

    pub fn openrouter(&self) -> &OpenRouterConfig {
        &self.openrouter
    }
}
