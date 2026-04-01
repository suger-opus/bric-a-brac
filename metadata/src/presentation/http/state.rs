use crate::{
    application::{AccessService, ChatService, GraphService, SessionService, UserService},
    infrastructure::{
        database, AccessRepository, AiClient, Config, GraphRepository, KnowledgeClient,
        SessionRepository, UserRepository,
    },
};

#[derive(Clone)]
#[allow(clippy::struct_field_names)]
pub struct ApiState {
    pub access_service: AccessService,
    pub graph_service: GraphService,
    pub session_service: SessionService,
    pub user_service: UserService,
    pub chat_service: ChatService,
}

impl ApiState {
    pub async fn build(config: &Config) -> anyhow::Result<Self> {
        let db_pool = database::connect(config.metadata_db()).await?;
        database::migrate(config.metadata_db(), &db_pool).await?;
        let knowledge_client = KnowledgeClient::new(
            config.knowledge_server(),
            config.internal_services_auth_token(),
        )?;
        let ai_client = AiClient::new(config.ai_server(), config.internal_services_auth_token())?;
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
        let chat_service = ChatService::new(ai_client);

        Ok(Self {
            access_service,
            graph_service,
            session_service,
            user_service,
            chat_service,
        })
    }
}
