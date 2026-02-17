use axum::http::Uri;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct KnowledgeServerConfig {
    /// Knowledge server URL
    #[arg(long, env = "KNOWLEDGE_SERVER_URL", required = true)]
    knowledge_server_url: Uri,
}

impl KnowledgeServerConfig {
    pub fn url(&self) -> &Uri {
        &self.knowledge_server_url
    }
}
