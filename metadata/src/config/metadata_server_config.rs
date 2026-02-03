use std::net::{IpAddr, SocketAddr};

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct MetadataServerConfig {
    /// Metadata server host
    #[arg(long, env = "METADATA_SERVER_HOST", required = true)]
    metadata_server_host: IpAddr,

    /// Metadata server port
    #[arg(long, env = "METADATA_SERVER_PORT", required = true)]
    metadata_server_port: u16,
}

impl MetadataServerConfig {
    pub fn url(&self) -> SocketAddr {
        (self.metadata_server_host, self.metadata_server_port).into()
    }
}
