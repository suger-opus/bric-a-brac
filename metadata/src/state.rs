use crate::config::Config;
use crate::grpc_client::KnowledgeClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ApiState {
    pub knowledge_client: KnowledgeClient,
    pub db_pool: PgPool,
}

impl ApiState {
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        tracing::info!("Initializing API state");
        
        let db_pool = config.metadata_db.connect().await?;
        config.metadata_db.migrate(&db_pool).await?;
        let knowledge_client = config.knowledge_server.connect().await?;
        
        tracing::info!("✓ All services initialized");

        Ok(ApiState {
            knowledge_client,
            db_pool,
        })
    }
}
