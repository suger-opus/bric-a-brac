use crate::{
    application::services::{
        AccessService, AiService, DocumentService, GraphService, SessionService, UserService,
    },
    infrastructure::{
        clients::{AiClient, KnowledgeClient},
        config::Config,
        database,
        repositories::{
            AccessRepository, DocumentRepository, GraphRepository, SessionRepository,
            UserRepository,
        },
    },
};

#[derive(Clone)]
pub struct ApiState {
    pub access_service: AccessService,
    pub ai_service: AiService,
    pub document_service: DocumentService,
    pub graph_service: GraphService,
    pub session_service: SessionService,
    pub user_service: UserService,
}

impl ApiState {
    pub async fn build(config: &Config) -> anyhow::Result<Self> {
        let db_pool = database::connect(config.metadata_db()).await?;
        database::migrate(config.metadata_db(), &db_pool).await?;
        let knowledge_client = KnowledgeClient::new(config.knowledge_server())?;
        let ai_client = AiClient::new(config.ai_server())?;
        let ai_service = AiService::new(ai_client);

        let access_repository = AccessRepository::new();
        let document_repository = DocumentRepository::new();
        let graph_repository = GraphRepository::new();
        let session_repository = SessionRepository::new();
        let user_repository = UserRepository::new();
        let access_service = AccessService::new(db_pool.clone(), access_repository.clone());
        let document_service = DocumentService::new(db_pool.clone(), document_repository);
        let graph_service = GraphService::new(
            db_pool.clone(),
            graph_repository,
            access_repository,
            knowledge_client,
        );
        let session_service = SessionService::new(db_pool.clone(), session_repository);
        let user_service = UserService::new(db_pool, user_repository);

        Ok(Self {
            access_service,
            ai_service,
            document_service,
            graph_service,
            session_service,
            user_service,
        })
    }
}
