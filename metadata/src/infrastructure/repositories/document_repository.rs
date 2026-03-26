use crate::{
    domain::models::{
        CreateSessionDocumentModel, SessionDocumentIdModel, SessionDocumentModel, SessionIdModel,
    },
    infrastructure::errors::DatabaseError,
};
use chrono::{DateTime, Utc};
use sqlx::PgConnection;

#[derive(Clone, Default)]
pub struct DocumentRepository;

impl DocumentRepository {
    pub const fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "document_repository.create_document",
        skip(self, connection, create),
        err
    )]
    pub async fn create_document(
        &self,
        connection: &mut PgConnection,
        create: CreateSessionDocumentModel,
    ) -> Result<SessionDocumentModel, DatabaseError> {
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
        name = "document_repository.get_document",
        skip(self, connection, document_id),
        err
    )]
    pub async fn get_document(
        &self,
        connection: &mut PgConnection,
        document_id: SessionDocumentIdModel,
    ) -> Result<SessionDocumentModel, DatabaseError> {
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
