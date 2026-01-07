use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "metadata")]
#[command(about = "Metadata microservice - Public API & Knowledge service client", long_about = None)]
pub struct Config {
    /// Metadata server host
    #[arg(long, env = "METADATA_HOST", default_value = "0.0.0.0")]
    pub metadata_host: String,

    /// Metadata server port
    #[arg(long, env = "METADATA_PORT", default_value_t = 8080)]
    pub metadata_port: u16,

    /// Knowledge server host
    #[arg(long, env = "KNOWLEDGE_HOST", default_value = "0.0.0.0")]
    pub knowledge_host: String,

    /// Knowledge server port
    #[arg(long, env = "KNOWLEDGE_PORT", default_value_t = 50051)]
    pub knowledge_port: u16,
}

impl Config {
    pub fn load() -> Self {
        Config::parse()
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.metadata_host, self.metadata_port)
    }

    pub fn knowledge_uri(&self) -> String {
        format!("http://{}:{}", self.knowledge_host, self.knowledge_port)
    }
}
