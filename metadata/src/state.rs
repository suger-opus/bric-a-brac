use crate::clients::knowledge_client::KnowledgeClient;
use crate::config::Config;
use crate::repositories::{
    access_repository::AccessRepository, graph_repository::GraphRepository,
    user_repository::UserRepository,
};
use crate::services::{
    access_service::AccessService, graph_service::GraphService, user_service::UserService,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ApiState {
    pub access_service: AccessService,
    pub graph_service: GraphService,
    pub user_service: UserService,
}

impl ApiState {
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
        tracing::info!("Initializing API state");

        let db_pool = config.metadata_db.connect().await?;
        config.metadata_db.migrate(&db_pool).await?;
        let knowledge_client = config.knowledge_server.connect().await?;

        tracing::info!("✓ All services initialized");

        Ok(Self::new(&db_pool, &knowledge_client).await)
    }

    pub async fn new(db_pool: &PgPool, knowledge_client: &KnowledgeClient) -> Self {
        let access_repository = AccessRepository::new();
        let graph_repository = GraphRepository::new();
        let user_repository = UserRepository::new();
        let access_service = AccessService::new(db_pool, &access_repository);
        let graph_service = GraphService::new(
            db_pool,
            &graph_repository,
            &access_repository,
            knowledge_client,
        );
        let user_service = UserService::new(db_pool, &user_repository);

        ApiState {
            access_service,
            graph_service,
            user_service,
        }
    }
}
