use crate::{
    application::{
        dtos::{CreateSessionDocumentDto, SessionDocumentDto, SessionDocumentIdDto},
        errors::AppError,
    },
    domain::models::CreateSessionDocumentModel,
    infrastructure::repositories::DocumentRepository,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct DocumentService {
    pool: PgPool,
    repository: DocumentRepository,
}

impl DocumentService {
    pub fn new(pool: PgPool, repository: DocumentRepository) -> Self {
        DocumentService { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "document_service.create_document",
        skip(self, create),
        err
    )]
    pub async fn create_document(
        &self,
        create: CreateSessionDocumentDto,
    ) -> Result<SessionDocumentDto, AppError> {
        let session_id = create
            .session_id
            .parse()
            .map_err(|_| crate::application::errors::RequestError::InvalidInput {
                field: "session_id".to_string(),
                issue: "Invalid session_id".to_string(),
            })?;

        let model = CreateSessionDocumentModel {
            document_id: SessionDocumentIdDto::new().into(),
            session_id,
            filename: create.filename,
            content_hash: create.content_hash,
            content: create.content,
        };

        let mut txn = self.pool.begin().await?;
        let document = self.repository.create_document(&mut txn, model).await?;
        txn.commit().await?;

        Ok(SessionDocumentDto::from(document))
    }

    #[tracing::instrument(
        level = "trace",
        name = "document_service.get_document",
        skip(self, document_id),
        err
    )]
    pub async fn get_document(
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
