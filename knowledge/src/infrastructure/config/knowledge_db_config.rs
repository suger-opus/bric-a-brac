#[derive(Clone, clap::Args, derive_more::Debug)]
#[allow(clippy::struct_field_names)]
pub struct KnowledgeDatabaseConfig {
    /// Knowledge database host
    #[arg(long, env = "KNOWLEDGE_DB_HOST", required = true)]
    knowledge_db_host: String,

    /// Knowledge database port
    #[arg(long, env = "KNOWLEDGE_DB_PORT", required = true)]
    knowledge_db_port: u16,

    /// Knowledge database username
    #[arg(long, env = "KNOWLEDGE_DB_USER", required = true)]
    knowledge_db_user: String,

    /// Knowledge database password
    #[arg(long, env = "KNOWLEDGE_DB_PASSWORD", required = true)]
    knowledge_db_password: String,

    /// Knowledge database name
    #[arg(long, env = "KNOWLEDGE_DB_NAME", required = true)]
    knowledge_db_name: String,

    /// Knowledge database maximum connections
    #[arg(long, env = "KNOWLEDGE_DB_MAX_CONNECTIONS", default_value_t = 10)]
    knowledge_db_max_connections: usize,

    /// Knowledge database fetch size
    #[arg(long, env = "KNOWLEDGE_DB_FETCH_SIZE", default_value_t = 10000)]
    knowledge_db_fetch_size: usize,
}

impl KnowledgeDatabaseConfig {
    pub fn url(&self) -> String {
        format!(
            "bolt://{}:{}",
            self.knowledge_db_host, self.knowledge_db_port
        )
    }

    pub fn user(&self) -> &str {
        &self.knowledge_db_user
    }

    pub fn password(&self) -> &str {
        &self.knowledge_db_password
    }

    pub fn name(&self) -> &str {
        &self.knowledge_db_name
    }

    pub const fn max_connections(&self) -> usize {
        self.knowledge_db_max_connections
    }

    pub const fn fetch_size(&self) -> usize {
        self.knowledge_db_fetch_size
    }
}
