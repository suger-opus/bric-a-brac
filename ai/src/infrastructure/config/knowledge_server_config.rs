use tonic::transport::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct KnowledgeServerConfig {
    /// Knowledge gRPC server URL
    #[arg(long, env = "KNOWLEDGE_GRPC_SERVER_URL", required = true)]
    knowledge_grpc_server_url: Uri,
}

impl KnowledgeServerConfig {
    pub fn url(&self) -> &Uri {
        &self.knowledge_grpc_server_url
    }
}
