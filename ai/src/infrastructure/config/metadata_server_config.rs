use tonic::transport::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct MetadataServerConfig {
    /// Metadata gRPC server URL
    #[arg(long, env = "METADATA_GRPC_SERVER_URL", required = true)]
    metadata_grpc_server_url: Uri,
}

impl MetadataServerConfig {
    pub fn url(&self) -> &Uri {
        &self.metadata_grpc_server_url
    }
}
