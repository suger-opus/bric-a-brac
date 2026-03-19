use crate::{
    domain::models::{
        CreateSessionMessageModel, CreateSessionModel, GraphIdModel, RoleModel, SessionIdModel,
        SessionMessageIdModel, SessionMessageModel, SessionMessageRoleModel, SessionModel,
        SessionStatusModel,
    },
    infrastructure::errors::DatabaseError,
};
use chrono::{DateTime, Utc};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct SessionRepository;

impl SessionRepository {
    pub fn new() -> Self {
        SessionRepository
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.has_active_session",
        skip(self, connection, graph_id)
    )]
    pub async fn has_active_session(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
    ) -> Result<bool, DatabaseError> {
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
        name = "session_repository.create_session",
        skip(self, connection, create_session)
    )]
    pub async fn create_session(
        &self,
        connection: &mut PgConnection,
        create_session: CreateSessionModel,
    ) -> Result<SessionModel, DatabaseError> {
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
    COALESCE(a.role, 'None'::role_type) AS "role!:_",
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
        name = "session_repository.get_session",
        skip(self, connection, session_id)
    )]
    pub async fn get_session(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<SessionModel, DatabaseError> {
        tracing::debug!(session_id = ?session_id);

        let row = sqlx::query_as!(
            SessionRow,
            r#"
SELECT
    s.session_id,
    s.graph_id,
    s.user_id,
    s.status AS "status!:_",
    COALESCE(a.role, 'None'::role_type) AS "role!:_",
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
        name = "session_repository.close_session",
        skip(self, connection, session_id, status)
    )]
    pub async fn close_session(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
        status: SessionStatusModel,
    ) -> Result<SessionModel, DatabaseError> {
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
    COALESCE(a.role, 'None'::role_type) AS "role!:_",
    u.created_at,
    u.updated_at
FROM updated u
LEFT JOIN accesses a ON u.user_id = a.user_id AND u.graph_id = a.graph_id
            "#,
            session_id as _,
            status.to_string(),
        )
        .fetch_one(connection)
        .await?;

        Ok(row.into())
    }

    #[tracing::instrument(
        level = "debug",
        name = "session_repository.get_messages",
        skip(self, connection, session_id)
    )]
    pub async fn get_messages(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<Vec<SessionMessageModel>, DatabaseError> {
        tracing::debug!(session_id = ?session_id);

        let rows = sqlx::query_as!(
            SessionMessageRow,
            r#"
SELECT
    message_id,
    session_id,
    position,
    role AS "role!:_",
    content,
    tool_calls,
    tool_call_id,
    created_at
FROM session_messages
WHERE session_id = $1
ORDER BY position ASC
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
        skip(self, connection, session_id)
    )]
    pub async fn get_max_position(
        &self,
        connection: &mut PgConnection,
        session_id: SessionIdModel,
    ) -> Result<i32, DatabaseError> {
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
        skip(self, connection, messages)
    )]
    pub async fn append_messages(
        &self,
        connection: &mut PgConnection,
        messages: Vec<CreateSessionMessageModel>,
    ) -> Result<Vec<SessionMessageModel>, DatabaseError> {
        let mut result = Vec::with_capacity(messages.len());

        for msg in messages {
            let row = sqlx::query_as!(
                SessionMessageRow,
                r#"
INSERT INTO session_messages (message_id, session_id, position, role, content, tool_calls, tool_call_id)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING
    message_id,
    session_id,
    position,
    role AS "role!:_",
    content,
    tool_calls,
    tool_call_id,
    created_at
                "#,
                SessionMessageIdModel::new() as _,
                msg.session_id as _,
                msg.position,
                msg.role.to_string(),
                msg.content,
                msg.tool_calls,
                msg.tool_call_id,
            )
            .fetch_one(&mut *connection)
            .await?;

            result.push(row.into());
        }

        Ok(result)
    }
}

// --- Row types ---

#[derive(sqlx::FromRow)]
struct SessionRow {
    session_id: SessionIdModel,
    graph_id: GraphIdModel,
    user_id: crate::domain::models::UserIdModel,
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
    tool_calls: Option<serde_json::Value>,
    tool_call_id: Option<String>,
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
            created_at: row.created_at,
        }
    }
}
