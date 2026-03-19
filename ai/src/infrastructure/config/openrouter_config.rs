use secrecy::SecretString;

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct OpenRouterConfig {
    /// OpenRouter API key
    #[arg(long, env = "OPENROUTER_API_KEY", required = true)]
    openrouter_api_key: SecretString,

    /// Default model to use for LLM requests
    #[arg(
        long,
        env = "OPENROUTER_DEFAULT_MODEL",
        default_value = "qwen/qwen-2.5-72b-instruct"
    )]
    openrouter_default_model: String,

    /// Model to use for embedding requests
    #[arg(
        long,
        env = "OPENROUTER_EMBEDDING_MODEL",
        default_value = "openai/text-embedding-3-small"
    )]
    openrouter_embedding_model: String,
}

impl OpenRouterConfig {
    pub fn api_key(&self) -> &SecretString {
        &self.openrouter_api_key
    }

    pub fn default_model(&self) -> &str {
        &self.openrouter_default_model
    }

    pub fn embedding_model(&self) -> &str {
        &self.openrouter_embedding_model
    }
}
