use crate::{
    application::{AppError, CreateSessionDocumentDto},
    domain::{
        CreateSessionDocumentModel, CreateSessionMessageModel, CreateSessionModel, RoleModel,
        UserIdModel,
    },
    infrastructure::{AccessRepository, SessionRepository},
};
use bric_a_brac_dtos::{
    CreateSessionMessageDto, GraphIdDto, SessionDocumentDto, SessionDocumentIdDto, SessionDto,
    SessionIdDto, SessionMessageDto, SessionStatusDto, UserIdDto,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct SessionService {
    pool: PgPool,
    repository: SessionRepository,
    access_repository: AccessRepository,
}

impl SessionService {
    pub const fn new(
        pool: PgPool,
        repository: SessionRepository,
        access_repository: AccessRepository,
    ) -> Self {
        Self {
            pool,
            repository,
            access_repository,
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.create",
        skip(self, graph_id, user_id),
        err
    )]
    pub async fn create(
        &self,
        graph_id: GraphIdDto,
        user_id: UserIdDto,
    ) -> Result<SessionDto, AppError> {
        let model = CreateSessionModel {
            session_id: SessionIdDto::new().into(),
            graph_id: graph_id.into(),
            user_id: user_id.into(),
        };

        let mut txn = self.pool.begin().await?;

        // Check user has at least viewer access to create a session
        let role = self
            .access_repository
            .get_role(&mut txn, model.graph_id, model.user_id)
            .await?;
        if !role.has_at_least(&RoleModel::Viewer) {
            return Err(AppError::Forbidden);
        }

        let has_active = self
            .repository
            .has_active_session(&mut txn, model.graph_id)
            .await?;

        if has_active {
            return Err(AppError::ActiveSessionAlreadyExists);
        }

        let session = self.repository.create(&mut txn, model).await?;
        txn.commit().await?;

        Ok(session.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.list",
        skip(self, graph_id, user_id),
        err
    )]
    pub async fn list(
        &self,
        graph_id: GraphIdDto,
        user_id: UserIdDto,
    ) -> Result<Vec<SessionDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let sessions = self
            .repository
            .list(&mut txn, graph_id.into(), user_id.into())
            .await?;
        txn.commit().await?;

        Ok(sessions.into_iter().map(From::from).collect())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get",
        skip(self, session_id),
        err
    )]
    pub async fn get(
        &self,
        session_id: SessionIdDto,
        user_id: UserIdDto,
    ) -> Result<SessionDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self.repository.get(&mut txn, session_id.into()).await?;

        Self::require_owner(session.user_id, user_id.into())?;

        txn.commit().await?;

        Ok(session.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.close",
        skip(self, session_id, status),
        err
    )]
    pub async fn close(
        &self,
        session_id: SessionIdDto,
        user_id: UserIdDto,
        status: SessionStatusDto,
    ) -> Result<SessionDto, AppError> {
        let mut txn = self.pool.begin().await?;

        // Verify session ownership
        let session = self.repository.get(&mut txn, session_id.into()).await?;
        Self::require_owner(session.user_id, user_id.into())?;

        let session = self
            .repository
            .close(&mut txn, session_id.into(), status.into())
            .await?;
        txn.commit().await?;

        Ok(session.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get_messages",
        skip(self, session_id),
        err
    )]
    pub async fn get_messages(
        &self,
        session_id: SessionIdDto,
        user_id: UserIdDto,
    ) -> Result<Vec<SessionMessageDto>, AppError> {
        let mut txn = self.pool.begin().await?;

        let session = self.repository.get(&mut txn, session_id.into()).await?;
        Self::require_owner(session.user_id, user_id.into())?;

        let messages = self
            .repository
            .get_messages(&mut txn, session_id.into())
            .await?;
        txn.commit().await?;

        Ok(messages.into_iter().map(From::from).collect())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.create_document",
        skip(self, doc),
        err
    )]
    pub async fn create_document(
        &self,
        user_id: UserIdDto,
        doc: CreateSessionDocumentDto,
    ) -> Result<SessionDocumentDto, AppError> {
        let model = CreateSessionDocumentModel {
            document_id: SessionDocumentIdDto::new().into(),
            session_id: doc.session_id.into(),
            filename: doc.filename,
            content_hash: doc.content_hash,
            content: doc.content,
        };

        let mut txn = self.pool.begin().await?;

        let session = self.repository.get(&mut txn, model.session_id).await?;
        Self::require_owner(session.user_id, user_id.into())?;

        let document = self.repository.create_document(&mut txn, model).await?;
        txn.commit().await?;

        Ok(SessionDocumentDto::from(document))
    }

    /// Verify the session belongs to the given user.
    fn require_owner(session_user_id: UserIdModel, user_id: UserIdModel) -> Result<(), AppError> {
        if session_user_id != user_id {
            return Err(AppError::Forbidden);
        }
        Ok(())
    }

    // === Internal methods (for trusted gRPC service-to-service calls, no user auth) ===

    pub async fn get_session_internal(
        &self,
        session_id: SessionIdDto,
    ) -> Result<SessionDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self.repository.get(&mut txn, session_id.into()).await?;
        txn.commit().await?;

        Ok(session.into())
    }

    pub async fn get_messages_internal(
        &self,
        session_id: SessionIdDto,
    ) -> Result<Vec<SessionMessageDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let messages = self
            .repository
            .get_messages(&mut txn, session_id.into())
            .await?;
        txn.commit().await?;

        Ok(messages.into_iter().map(From::from).collect())
    }

    pub async fn append_messages_internal(
        &self,
        session_id: SessionIdDto,
        messages: Vec<CreateSessionMessageDto>,
    ) -> Result<Vec<SessionMessageDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let max_pos = self
            .repository
            .get_max_position(&mut txn, session_id.into())
            .await?;

        let models = messages
            .into_iter()
            .enumerate()
            .map(|(i, dto)| CreateSessionMessageModel {
                session_id: session_id.into(),
                position: max_pos + i32::try_from(i).unwrap_or_default() + 1,
                role: dto.role.into(),
                content: dto.content,
                tool_calls: dto.tool_calls,
                tool_call_id: dto.tool_call_id,
                document_id: dto.document_id.map(From::from),
                chunk_index: dto.chunk_index,
            })
            .collect();

        let result = self.repository.append_messages(&mut txn, models).await?;
        txn.commit().await?;

        Ok(result.into_iter().map(From::from).collect())
    }

    pub async fn get_document_internal(
        &self,
        document_id: SessionDocumentIdDto,
    ) -> Result<SessionDocumentDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let document = self
            .repository
            .get_document(&mut txn, document_id.into())
            .await?;
        txn.commit().await?;

        Ok(SessionDocumentDto::from(document))
    }
}
