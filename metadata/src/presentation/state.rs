use crate::{
    application::services::{AccessService, GraphService, SessionService, UserService},
    infrastructure::{
        clients::KnowledgeClient,
        config::Config,
        database,
        repositories::{AccessRepository, GraphRepository, SessionRepository, UserRepository},
    },
};

#[derive(Clone)]
pub struct ApiState {
    pub access_service: AccessService,
    pub graph_service: GraphService,
    pub session_service: SessionService,
    pub user_service: UserService,
}

impl ApiState {
    pub async fn build(config: &Config) -> anyhow::Result<Self> {
        let db_pool = database::connect(config.metadata_db()).await?;
        database::migrate(config.metadata_db(), &db_pool).await?;
        let knowledge_client = KnowledgeClient::new(config.knowledge_server().clone());

        let access_repository = AccessRepository::new();
        let graph_repository = GraphRepository::new();
        let session_repository = SessionRepository::new();
        let user_repository = UserRepository::new();
        let access_service = AccessService::new(db_pool.clone(), access_repository.clone());
        let graph_service = GraphService::new(
            db_pool.clone(),
            graph_repository,
            access_repository,
            knowledge_client,
        );
        let session_service = SessionService::new(db_pool.clone(), session_repository);
        let user_service = UserService::new(db_pool, user_repository);

        Ok(ApiState {
            access_service,
            graph_service,
            session_service,
            user_service,
        })
    }
}
