use axum::http::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct AiServerConfig {
    /// AI gRPC server URL
    #[arg(long, env = "AI_GRPC_SERVER_URL", required = true)]
    ai_grpc_server_url: Uri,
}

impl AiServerConfig {
    pub const fn url(&self) -> &Uri {
        &self.ai_grpc_server_url
    }
}
