use crate::{
    domain::models::{
        CreateSessionMessageModel, CreateSessionModel, SessionIdModel, SessionMessageModel,
        SessionModel, SessionStatusModel,
    },
    application::errors::AppError,
    infrastructure::{errors::DatabaseError, repositories::SessionRepository},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct SessionService {
    pool: PgPool,
    repository: SessionRepository,
}

impl SessionService {
    pub fn new(pool: PgPool, repository: SessionRepository) -> Self {
        SessionService { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.create_session",
        skip(self, create)
    )]
    pub async fn create_session(
        &self,
        create: CreateSessionModel,
    ) -> Result<SessionModel, AppError> {
        let mut txn = self.pool.begin().await?;

        // Check if there's already an active session for this graph
        let has_active = self
            .repository
            .has_active_session(&mut txn, create.graph_id)
            .await?;

        if has_active {
            return Err(AppError::Database(DatabaseError::UnexpectedState {
                reason: "An active session already exists for this graph".to_string(),
            }));
        }

        let session = self.repository.create_session(&mut txn, create).await?;
        txn.commit().await?;

        Ok(session)
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get_session",
        skip(self, session_id)
    )]
    pub async fn get_session(
        &self,
        session_id: SessionIdModel,
    ) -> Result<SessionModel, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self.repository.get_session(&mut txn, session_id).await?;
        txn.commit().await?;

        Ok(session)
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.close_session",
        skip(self, session_id, status)
    )]
    pub async fn close_session(
        &self,
        session_id: SessionIdModel,
        status: SessionStatusModel,
    ) -> Result<SessionModel, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self
            .repository
            .close_session(&mut txn, session_id, status)
            .await?;
        txn.commit().await?;

        Ok(session)
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get_messages",
        skip(self, session_id)
    )]
    pub async fn get_messages(
        &self,
        session_id: SessionIdModel,
    ) -> Result<Vec<SessionMessageModel>, AppError> {
        let mut txn = self.pool.begin().await?;
        let messages = self.repository.get_messages(&mut txn, session_id).await?;
        txn.commit().await?;

        Ok(messages)
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.append_messages",
        skip(self, session_id, messages)
    )]
    pub async fn append_messages(
        &self,
        session_id: SessionIdModel,
        messages: Vec<CreateSessionMessageModel>,
    ) -> Result<Vec<SessionMessageModel>, AppError> {
        let mut txn = self.pool.begin().await?;
        let max_pos = self
            .repository
            .get_max_position(&mut txn, session_id)
            .await?;

        // Re-number messages starting from max_pos + 1
        let messages: Vec<CreateSessionMessageModel> = messages
            .into_iter()
            .enumerate()
            .map(|(i, mut msg)| {
                msg.position = max_pos + i as i32 + 1;
                msg
            })
            .collect();

        let result = self.repository.append_messages(&mut txn, messages).await?;
        txn.commit().await?;

        Ok(result)
    }
}
