use crate::domain::{
    SessionDocumentIdModel, SessionDocumentModel, SessionIdModel, SessionMessageIdModel,
    SessionMessageModel, SessionMessageRoleModel, SessionModel, SessionStatusModel,
};
use bric_a_brac_dtos::{
    GraphIdDto, SessionDocumentDto, SessionDocumentIdDto, SessionDto, SessionIdDto,
    SessionMessageDto, SessionMessageIdDto, SessionMessageRoleDto, SessionStatusDto,
};
use serde::Deserialize;
use utoipa::ToSchema;

impl From<SessionIdModel> for SessionIdDto {
    fn from(id: SessionIdModel) -> Self {
        Self::from(*id.as_ref())
    }
}

impl From<SessionIdDto> for SessionIdModel {
    fn from(id: SessionIdDto) -> Self {
        Self::from(*id.as_ref())
    }
}

impl From<SessionMessageIdModel> for SessionMessageIdDto {
    fn from(id: SessionMessageIdModel) -> Self {
        Self::from(*id.as_ref())
    }
}

impl From<SessionMessageIdDto> for SessionMessageIdModel {
    fn from(id: SessionMessageIdDto) -> Self {
        Self::from(*id.as_ref())
    }
}

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

impl From<SessionStatusDto> for SessionStatusModel {
    fn from(role: SessionStatusDto) -> Self {
        match role {
            SessionStatusDto::Active => Self::Active,
            SessionStatusDto::Completed => Self::Completed,
            SessionStatusDto::Failed => Self::Error,
        }
    }
}

impl From<SessionStatusModel> for SessionStatusDto {
    fn from(role: SessionStatusModel) -> Self {
        match role {
            SessionStatusModel::Active => Self::Active,
            SessionStatusModel::Completed => Self::Completed,
            SessionStatusModel::Error => Self::Failed,
        }
    }
}

impl From<SessionMessageRoleDto> for SessionMessageRoleModel {
    fn from(role: SessionMessageRoleDto) -> Self {
        match role {
            SessionMessageRoleDto::System => Self::System,
            SessionMessageRoleDto::User => Self::User,
            SessionMessageRoleDto::Assistant => Self::Assistant,
            SessionMessageRoleDto::Tool => Self::Tool,
        }
    }
}

impl From<SessionMessageRoleModel> for SessionMessageRoleDto {
    fn from(role: SessionMessageRoleModel) -> Self {
        match role {
            SessionMessageRoleModel::System => Self::System,
            SessionMessageRoleModel::User => Self::User,
            SessionMessageRoleModel::Assistant => Self::Assistant,
            SessionMessageRoleModel::Tool => Self::Tool,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSessionDto {
    pub graph_id: GraphIdDto,
}

#[derive(Debug)]
pub struct CreateSessionDocumentDto {
    pub session_id: SessionIdDto,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
}

impl From<SessionDocumentModel> for SessionDocumentDto {
    fn from(model: SessionDocumentModel) -> Self {
        Self {
            document_id: model.document_id.into(),
            session_id: model.session_id.into(),
            filename: model.filename,
            content_hash: model.content_hash,
            content: model.content,
            created_at: model.created_at,
        }
    }
}

impl From<SessionModel> for SessionDto {
    fn from(model: SessionModel) -> Self {
        Self {
            session_id: model.session_id.into(),
            graph_id: model.graph_id.into(),
            user_id: model.user_id.into(),
            status: model.status.into(),
            role: model.role.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<SessionMessageModel> for SessionMessageDto {
    fn from(model: SessionMessageModel) -> Self {
        Self {
            message_id: model.message_id.into(),
            session_id: model.session_id.into(),
            position: model.position,
            role: model.role.into(),
            content: model.content,
            tool_calls: model.tool_calls,
            tool_call_id: model.tool_call_id,
            document_id: model.document_id.map(From::from),
            document_name: model.document_name,
            chunk_index: model.chunk_index,
            created_at: model.created_at,
        }
    }
}
