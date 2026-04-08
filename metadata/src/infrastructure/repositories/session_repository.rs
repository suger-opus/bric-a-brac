use crate::{
    domain::{
        CreateSessionDocumentModel, CreateSessionMessageModel, CreateSessionModel, GraphIdModel,
        RoleModel, SessionDocumentIdModel, SessionDocumentModel, SessionIdModel,
        SessionMessageIdModel, SessionMessageModel, SessionMessageRoleModel, SessionModel,
        SessionStatusModel, UserIdModel,
    },
    infrastructure::InfraError,
};
use chrono::{DateTime, Utc};
use sqlx::PgConnection;

#[derive(Clone, Default)]
pub struct SessionRepository;

impl SessionRepository {
    pub const fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.has_active_session",
        skip(self, connection, graph_id),
        err
    )]
    pub async fn has_active_session(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
    ) -> Result<bool, InfraError> {
        tracing::debug!(graph_id = ?graph_id);

        let row = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM sessions WHERE graph_id = $1 AND status = 'active') AS "exists!""#,
            graph_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row)
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.list",
        skip(self, connection, graph_id, user_id),
        err
    )]
    pub async fn list(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        user_id: UserIdModel,
    ) -> Result<Vec<SessionModel>, InfraError> {
        tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

        let rows: Vec<SessionRow> = sqlx::query_as!(
            SessionRow,
            r#"
SELECT
    s.session_id,
    s.graph_id,
    s.user_id,
    s.status AS "status!:_",
    COALESCE(a.role, 'none'::role_type) AS "role!:_",
    s.created_at,
    s.updated_at
FROM sessions s
LEFT JOIN accesses a ON s.user_id = a.user_id AND s.graph_id = a.graph_id
WHERE s.graph_id = $1 AND s.user_id = $2
ORDER BY s.updated_at DESC
            "#,
            graph_id as _,
            user_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(rows.into_iter().map(SessionRow::into).collect())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.create",
        skip(self, connection, create_session),
        err
    )]
    pub async fn create(
        &self,
        connection: &mut PgConnection,
        create_session: CreateSessionModel,
    ) -> Result<SessionModel, InfraError> {
        tracing::debug!(session_id = ?create_session.session_id, graph_id = ?create_session.graph_id);

        let row = sqlx::query_as!(
            SessionRow,
            r#"
WITH inserted AS (
    INSERT INTO sessions (session_id, graph_id, user_id)
    VALUES ($1, $2, $3)
    RETURNING session_id, graph_id, user_id, status, created_at, updated_at
)
SELECT
    i.session_id,
    i.graph_id,
    i.user_id,
    i.status AS "status!:_",
    COALESCE(a.role, 'none'::role_type) AS "role!:_",
    i.created_at,
    i.updated_at
FROM inserted i
LEFT JOIN accesses a ON i.user_id = a.user_id AND i.graph_id = a.graph_id
            "#,
            create_session.session_id as _,
            create_session.graph_id as _,
            create_session.user_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.get",
        skip(self, connection, session_id),
        err
    )]
    pub async fn get(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<SessionModel, InfraError> {
        tracing::debug!(session_id = ?session_id);

        let row = sqlx::query_as!(
            SessionRow,
            r#"
SELECT
    s.session_id,
    s.graph_id,
    s.user_id,
    s.status AS "status!:_",
    COALESCE(a.role, 'none'::role_type) AS "role!:_",
    s.created_at,
    s.updated_at
FROM sessions s
LEFT JOIN accesses a ON s.user_id = a.user_id AND s.graph_id = a.graph_id
WHERE s.session_id = $1
            "#,
            session_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.close",
        skip(self, connection, session_id, status),
        err
    )]
    pub async fn close(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
        status: SessionStatusModel,
    ) -> Result<SessionModel, InfraError> {
        tracing::debug!(session_id = ?session_id, status = ?status);

        let row = sqlx::query_as!(
            SessionRow,
            r#"
WITH updated AS (
    UPDATE sessions
    SET status = $2, updated_at = CURRENT_TIMESTAMP
    WHERE session_id = $1
    RETURNING session_id, graph_id, user_id, status, created_at, updated_at
)
SELECT
    u.session_id,
    u.graph_id,
    u.user_id,
    u.status AS "status!:_",
    COALESCE(a.role, 'none'::role_type) AS "role!:_",
    u.created_at,
    u.updated_at
FROM updated u
LEFT JOIN accesses a ON u.user_id = a.user_id AND u.graph_id = a.graph_id
            "#,
            session_id as _,
            status as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.get_messages",
        skip(self, connection, session_id),
        err
    )]
    pub async fn get_messages(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<Vec<SessionMessageModel>, InfraError> {
        tracing::debug!(session_id = ?session_id);

        let rows = sqlx::query_as!(
            SessionMessageRow,
            r#"
SELECT
    sm.message_id,
    sm.session_id,
    sm.position,
    sm.role AS "role!:_",
    sm.content,
    sm.tool_calls,
    sm.tool_call_id,
    sm.document_id,
    sd.filename AS "document_name?",
    sm.chunk_index,
    sm.created_at
FROM session_messages sm
LEFT JOIN session_documents sd ON sm.document_id = sd.document_id
WHERE sm.session_id = $1
ORDER BY sm.position ASC
            "#,
            session_id as _,
        )
        .fetch_all(connection)
        .await?;

        Ok(rows.into_iter().map(SessionMessageRow::into).collect())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.get_max_position",
        skip(self, connection, session_id),
        err
    )]
    pub async fn get_max_position(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<i32, InfraError> {
        tracing::debug!(session_id = ?session_id);

        let row = sqlx::query_scalar!(
            r#"SELECT COALESCE(MAX(position), -1) AS "max!" FROM session_messages WHERE session_id = $1"#,
            session_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row)
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.append_messages",
        skip(self, connection, messages),
        err
    )]
    pub async fn append_messages(
        &self,
        connection: &mut PgConnection,
        messages: Vec<CreateSessionMessageModel>,
    ) -> Result<Vec<SessionMessageModel>, InfraError> {
        let mut result = Vec::with_capacity(messages.len());

        for msg in messages {
            let row = sqlx::query_as!(
                SessionMessageRow,
                r#"
WITH inserted AS (
INSERT INTO session_messages (message_id, session_id, position, role, content, tool_calls, tool_call_id, document_id, chunk_index)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING *
)
SELECT
    i.message_id,
    i.session_id,
    i.position,
    i.role AS "role!:_",
    i.content,
    i.tool_calls,
    i.tool_call_id,
    i.document_id,
    sd.filename AS "document_name?",
    i.chunk_index,
    i.created_at
FROM inserted i
LEFT JOIN session_documents sd ON i.document_id = sd.document_id
                "#,
                SessionMessageIdModel::new() as _,
                msg.session_id as _,
                msg.position,
                msg.role as _,
                msg.content,
                msg.tool_calls,
                msg.tool_call_id,
                msg.document_id as _,
                msg.chunk_index,
            )
            .fetch_one(&mut *connection)
            .await?;

            result.push(row.into());
        }

        Ok(result)
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.create_document",
        skip(self, connection, create),
        err
    )]
    pub async fn create_document(
        &self,
        connection: &mut PgConnection,
        create: CreateSessionDocumentModel,
    ) -> Result<SessionDocumentModel, InfraError> {
        let row = sqlx::query_as!(
            DocumentRow,
            r#"
INSERT INTO session_documents (document_id, session_id, filename, content_hash, content)
VALUES ($1, $2, $3, $4, $5)
RETURNING
    document_id,
    session_id,
    filename,
    content_hash,
    content,
    created_at
            "#,
            create.document_id as _,
            create.session_id as _,
            create.filename,
            create.content_hash,
            create.content,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.get_document",
        skip(self, connection, document_id),
        err
    )]
    pub async fn get_document(
        &self,
        connection: &mut PgConnection,
        document_id: SessionDocumentIdModel,
    ) -> Result<SessionDocumentModel, InfraError> {
        let row = sqlx::query_as!(
            DocumentRow,
            r#"
SELECT
    document_id,
    session_id,
    filename,
    content_hash,
    content,
    created_at
FROM session_documents
WHERE document_id = $1
            "#,
            document_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct SessionRow {
    session_id: SessionIdModel,
    graph_id: GraphIdModel,
    user_id: UserIdModel,
    status: SessionStatusModel,
    role: RoleModel,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<SessionRow> for SessionModel {
    fn from(row: SessionRow) -> Self {
        Self {
            session_id: row.session_id,
            graph_id: row.graph_id,
            user_id: row.user_id,
            status: row.status,
            role: row.role,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SessionMessageRow {
    message_id: SessionMessageIdModel,
    session_id: SessionIdModel,
    position: i32,
    role: SessionMessageRoleModel,
    content: String,
    tool_calls: Option<String>,
    tool_call_id: Option<String>,
    document_id: Option<uuid::Uuid>,
    document_name: Option<String>,
    chunk_index: Option<i32>,
    created_at: DateTime<Utc>,
}

impl From<SessionMessageRow> for SessionMessageModel {
    fn from(row: SessionMessageRow) -> Self {
        Self {
            message_id: row.message_id,
            session_id: row.session_id,
            position: row.position,
            role: row.role,
            content: row.content,
            tool_calls: row.tool_calls,
            tool_call_id: row.tool_call_id,
            document_id: row.document_id.map(SessionDocumentIdModel::from),
            document_name: row.document_name,
            chunk_index: row.chunk_index,
            created_at: row.created_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct DocumentRow {
    document_id: SessionDocumentIdModel,
    session_id: SessionIdModel,
    filename: String,
    content_hash: String,
    content: String,
    created_at: DateTime<Utc>,
}

impl From<DocumentRow> for SessionDocumentModel {
    fn from(row: DocumentRow) -> Self {
        Self {
            document_id: row.document_id,
            session_id: row.session_id,
            filename: row.filename,
            content_hash: row.content_hash,
            content: row.content,
            created_at: row.created_at,
        }
    }
}
