use axum::http::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct AiServerConfig {
    /// AI server URL
    #[arg(long, env = "AI_SERVER_URL", required = true)]
    ai_server_url: Uri,
}

impl AiServerConfig {
    pub fn url(&self) -> &Uri {
        &self.ai_server_url
    }
}
