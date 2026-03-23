use crate::domain::models::{SessionDocumentIdModel, SessionDocumentModel};
use bric_a_brac_dtos::utils::ProtoTimestampExt;
use bric_a_brac_id::id;
use bric_a_brac_protos::metadata::SessionDocumentProto;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::Serialize;

id!(SessionDocumentIdDto);

impl From<SessionDocumentIdModel> for SessionDocumentIdDto {
    fn from(id: SessionDocumentIdModel) -> Self {
        Self::from(*id.as_ref())
    }
}

impl From<SessionDocumentIdDto> for SessionDocumentIdModel {
    fn from(id: SessionDocumentIdDto) -> Self {
        Self::from(*id.as_ref())
    }
}

#[derive(Debug)]
pub struct CreateSessionDocumentDto {
    pub session_id: String,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct SessionDocumentDto {
    pub document_id: SessionDocumentIdDto,
    pub session_id: String,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl From<SessionDocumentModel> for SessionDocumentDto {
    fn from(model: SessionDocumentModel) -> Self {
        Self {
            document_id: model.document_id.into(),
            session_id: model.session_id.to_string(),
            filename: model.filename,
            content_hash: model.content_hash,
            content: model.content,
            created_at: model.created_at,
        }
    }
}

impl From<SessionDocumentDto> for SessionDocumentProto {
    fn from(dto: SessionDocumentDto) -> Self {
        Self {
            document_id: dto.document_id.to_string(),
            session_id: dto.session_id,
            filename: dto.filename,
            content_hash: dto.content_hash,
            content: dto.content,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
        }
    }
}
