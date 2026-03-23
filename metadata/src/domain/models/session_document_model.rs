use super::SessionIdModel;
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(SessionDocumentIdModel);

#[derive(Debug, Clone, Serialize)]
pub struct SessionDocumentModel {
    pub document_id: SessionDocumentIdModel,
    pub session_id: SessionIdModel,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreateSessionDocumentModel {
    pub document_id: SessionDocumentIdModel,
    pub session_id: SessionIdModel,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
}
