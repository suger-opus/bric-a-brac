use neo4rs::{ConfigBuilder, Graph};

#[derive(Clone, clap::Args, derive_more::Debug)]
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

    pub async fn connect(&self) -> anyhow::Result<Graph> {
        let uri = self.url();

        let config = ConfigBuilder::default()
            .uri(&uri)
            .user(&self.knowledge_db_user)
            .password(&self.knowledge_db_password)
            .db(self.knowledge_db_name.as_str())
            .fetch_size(self.knowledge_db_fetch_size)
            .max_connections(self.knowledge_db_max_connections)
            .build()?;

        let graph = Graph::connect(config).await?;

        // Verify connection by running a simple query with timeout
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            graph.run(neo4rs::query("RETURN 1")),
        )
        .await
        .map_err(|_| {
            anyhow::anyhow!("Knowledge database connection timeout - check credentials and network")
        })?
        .map_err(|e| anyhow::anyhow!("Failed to verify knowledge database connection: {}", e))?;

        Ok(graph)
    }
}
