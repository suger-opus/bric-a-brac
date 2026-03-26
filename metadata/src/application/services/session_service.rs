use crate::{
    application::{
        dtos::{
            CreateSessionDto, CreateSessionMessageDto, SessionDto, SessionIdDto, SessionMessageDto,
            UserIdDto,
        },
        errors::{AppError, RequestError},
    },
    domain::models::{CreateSessionMessageModel, CreateSessionModel, SessionDocumentIdModel},
    infrastructure::{errors::DatabaseError, repositories::SessionRepository},
};
use bric_a_brac_dtos::GraphIdDto;
use sqlx::PgPool;

#[derive(Clone)]
pub struct SessionService {
    pool: PgPool,
    repository: SessionRepository,
}

impl SessionService {
    pub const fn new(pool: PgPool, repository: SessionRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.create_session",
        skip(self, create, user_id),
        err
    )]
    pub async fn create_session(
        &self,
        create: CreateSessionDto,
        user_id: UserIdDto,
    ) -> Result<SessionDto, AppError> {
        let model = CreateSessionModel {
            session_id: SessionIdDto::new().into(),
            graph_id: create.graph_id.into(),
            user_id: user_id.into(),
        };

        let mut txn = self.pool.begin().await?;

        let has_active = self
            .repository
            .has_active_session(&mut txn, model.graph_id)
            .await?;

        if has_active {
            return Err(AppError::Database(DatabaseError::UnexpectedState {
                reason: "An active session already exists for this graph".to_owned(),
            }));
        }

        let session = self.repository.create_session(&mut txn, model).await?;
        txn.commit().await?;

        Ok(SessionDto::from(session))
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get_active_session",
        skip(self, graph_id, user_id),
        err
    )]
    pub async fn get_active_session(
        &self,
        graph_id: GraphIdDto,
        user_id: UserIdDto,
    ) -> Result<Option<SessionDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self
            .repository
            .get_active_session(&mut txn, graph_id.into(), user_id.into())
            .await?;
        txn.commit().await?;

        Ok(session.map(SessionDto::from))
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.get_session",
        skip(self, session_id),
        err
    )]
    pub async fn get_session(&self, session_id: SessionIdDto) -> Result<SessionDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let session = self
            .repository
            .get_session(&mut txn, session_id.into())
            .await?;
        txn.commit().await?;

        Ok(SessionDto::from(session))
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.close_session",
        skip(self, session_id, status),
        err
    )]
    pub async fn close_session(
        &self,
        session_id: SessionIdDto,
        status: &str,
    ) -> Result<SessionDto, AppError> {
        let status_model = status.parse().map_err(|err| RequestError::InvalidInput {
            field: "status".to_owned(),
            issue: format!("Invalid session status: {status}"),
            source: Some(Box::new(err)),
        })?;

        let mut txn = self.pool.begin().await?;
        let session = self
            .repository
            .close_session(&mut txn, session_id.into(), status_model)
            .await?;
        txn.commit().await?;

        Ok(SessionDto::from(session))
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
    ) -> Result<Vec<SessionMessageDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let messages = self
            .repository
            .get_messages(&mut txn, session_id.into())
            .await?;
        txn.commit().await?;

        Ok(messages.into_iter().map(SessionMessageDto::from).collect())
    }

    #[tracing::instrument(
        level = "trace",
        name = "session_service.append_messages",
        skip(self, session_id, messages),
        err
    )]
    pub async fn append_messages(
        &self,
        session_id: SessionIdDto,
        messages: Vec<CreateSessionMessageDto>,
    ) -> Result<Vec<SessionMessageDto>, AppError> {
        let session_id_model = session_id.into();
        let mut txn = self.pool.begin().await?;
        let max_pos = self
            .repository
            .get_max_position(&mut txn, session_id_model)
            .await?;

        let models = messages
            .into_iter()
            .enumerate()
            .map(|(i, dto)| {
                let role = dto.role.parse().map_err(|err| RequestError::InvalidInput {
                    field: "role".to_owned(),
                    issue: format!("Invalid message role: {}", dto.role),
                    source: Some(Box::new(err)),
                })?;
                let tool_calls = dto
                    .tool_calls
                    .map(|s| serde_json::from_str(&s))
                    .transpose()
                    .map_err(|err| RequestError::InvalidInput {
                        field: "tool_calls".to_owned(),
                        issue: "Invalid JSON in tool_calls".to_owned(),
                        source: Some(Box::new(err)),
                    })?;

                Ok(CreateSessionMessageModel {
                    session_id: session_id_model,
                    position: max_pos + i32::try_from(i).unwrap_or_default() + 1,
                    role,
                    content: dto.content,
                    tool_calls,
                    tool_call_id: dto.tool_call_id,
                    document_id: dto
                        .document_id
                        .map(|id| id.parse::<SessionDocumentIdModel>())
                        .transpose()
                        .map_err(|err| RequestError::InvalidInput {
                            field: "document_id".to_owned(),
                            issue: "Invalid document_id".to_owned(),
                            source: Some(Box::new(err)),
                        })?,
                    chunk_index: dto.chunk_index,
                })
            })
            .collect::<Result<Vec<_>, RequestError>>()?;

        let result = self.repository.append_messages(&mut txn, models).await?;
        txn.commit().await?;

        Ok(result.into_iter().map(SessionMessageDto::from).collect())
    }
}
