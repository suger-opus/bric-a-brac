use std::net::{IpAddr, SocketAddr};

#[derive(clap::Args, derive_more::Debug)]
#[allow(clippy::struct_field_names)]
pub struct MetadataServerConfig {
    /// Metadata server host
    #[arg(long, env = "METADATA_SERVER_HOST", required = true)]
    metadata_server_host: IpAddr,

    /// Metadata HTTP server port
    #[arg(long, env = "METADATA_HTTP_SERVER_PORT", required = true)]
    metadata_http_server_port: u16,

    /// Metadata gRPC server port
    #[arg(long, env = "METADATA_GRPC_SERVER_PORT", required = true)]
    metadata_grpc_server_port: u16,
}

impl MetadataServerConfig {
    pub fn http_url(&self) -> SocketAddr {
        (self.metadata_server_host, self.metadata_http_server_port).into()
    }

    pub fn grpc_url(&self) -> SocketAddr {
        (self.metadata_server_host, self.metadata_grpc_server_port).into()
    }
}
