use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "knowledge")]
#[command(about = "Knowledge microservice - Private graph database management", long_about = None)]
pub struct Config {
    /// Knowledge server host
    #[arg(long, env = "KNOWLEDGE_HOST", default_value = "0.0.0.0")]
    pub knowledge_host: String,

    /// Knowledge server port
    #[arg(long, env = "KNOWLEDGE_PORT", default_value_t = 50051)]
    pub knowledge_port: u16,

    /// Graph database host
    #[arg(long, env = "DATABASE_HOST", required = true)]
    pub database_host: String,

    /// Graph database port
    #[arg(long, env = "DATABASE_PORT", required = true)]
    pub database_port: u16,

    /// Graph database username
    #[arg(long, env = "DATABASE_USER", required = true)]
    pub database_user: String,

    /// Graph database password
    #[arg(long, env = "DATABASE_PASSWORD", required = true)]
    pub database_password: String,

    /// Graph database name
    #[arg(long, env = "DATABASE_NAME", required = true)]
    pub database_name: String,
}

impl Config {
    pub fn load() -> Self {
        Config::parse()
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.knowledge_host, self.knowledge_port)
    }

    pub fn database_uri(&self) -> String {
        format!("bolt://{}:{}", self.database_host, self.database_port)
    }
}
