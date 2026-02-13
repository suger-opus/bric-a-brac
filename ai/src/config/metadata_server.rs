#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct MetadataServerConfig {
    /// Metadata server URL
    #[arg(long, env = "METADATA_SERVER_URL", required = true)]
    #[debug("{}", self.metadata_server_url)]
    metadata_server_url: String,
}

impl MetadataServerConfig {
    pub fn url(&self) -> &str {
        &self.metadata_server_url
    }
}
