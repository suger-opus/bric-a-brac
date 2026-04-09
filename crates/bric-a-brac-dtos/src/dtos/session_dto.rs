use super::{GraphIdDto, RoleDto, UserIdDto};
use crate::{utils::ProtoTimestampExt, DtosConversionError};
use bric_a_brac_id::id;
use bric_a_brac_protos::common::{
    CreateSessionMessageProto, SessionDocumentProto, SessionMessageProto, SessionMessageRoleProto,
    SessionProto, SessionStatusProto,
};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

id!(SessionIdDto);

impl TryFrom<String> for SessionIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
    }
}

id!(SessionMessageIdDto);

impl TryFrom<String> for SessionMessageIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
    }
}

id!(SessionDocumentIdDto);

impl TryFrom<String> for SessionDocumentIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, derive_more::Display)]
pub enum SessionStatusDto {
    #[display("active")]
    Active,
    #[display("completed")]
    Completed,
    #[display("failed")]
    Failed,
}

impl From<SessionStatusProto> for SessionStatusDto {
    fn from(proto: SessionStatusProto) -> Self {
        match proto {
            SessionStatusProto::SessionStatusActive => Self::Active,
            SessionStatusProto::SessionStatusCompleted => Self::Completed,
            SessionStatusProto::SessionStatusError => Self::Failed,
        }
    }
}

impl From<SessionStatusDto> for SessionStatusProto {
    fn from(dto: SessionStatusDto) -> Self {
        match dto {
            SessionStatusDto::Active => Self::SessionStatusActive,
            SessionStatusDto::Completed => Self::SessionStatusCompleted,
            SessionStatusDto::Failed => Self::SessionStatusError,
        }
    }
}

impl TryFrom<i32> for SessionStatusDto {
    type Error = DtosConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(clippy::map_err_ignore)]
        SessionStatusProto::try_from(value)
            .map(Self::from)
            .map_err(|_| Self::Error::UnknownEnumVariant {
                enum_name: "SessionStatusProto".to_owned(),
                value,
            })
    }
}

impl From<SessionStatusDto> for i32 {
    fn from(dto: SessionStatusDto) -> Self {
        Self::from(SessionStatusProto::from(dto))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, derive_more::Display)]
pub enum SessionMessageRoleDto {
    #[display("system")]
    System,
    #[display("user")]
    User,
    #[display("assistant")]
    Assistant,
    #[display("tool")]
    Tool,
}

impl From<SessionMessageRoleDto> for SessionMessageRoleProto {
    fn from(dto: SessionMessageRoleDto) -> Self {
        match dto {
            SessionMessageRoleDto::System => Self::SessionMessageRoleSystem,
            SessionMessageRoleDto::User => Self::SessionMessageRoleUser,
            SessionMessageRoleDto::Assistant => Self::SessionMessageRoleAssistant,
            SessionMessageRoleDto::Tool => Self::SessionMessageRoleTool,
        }
    }
}

impl From<SessionMessageRoleProto> for SessionMessageRoleDto {
    fn from(proto: SessionMessageRoleProto) -> Self {
        match proto {
            SessionMessageRoleProto::SessionMessageRoleSystem => Self::System,
            SessionMessageRoleProto::SessionMessageRoleUser => Self::User,
            SessionMessageRoleProto::SessionMessageRoleAssistant => Self::Assistant,
            SessionMessageRoleProto::SessionMessageRoleTool => Self::Tool,
        }
    }
}

impl TryFrom<i32> for SessionMessageRoleDto {
    type Error = DtosConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(clippy::map_err_ignore)]
        SessionMessageRoleProto::try_from(value)
            .map(Self::from)
            .map_err(|_| Self::Error::UnknownEnumVariant {
                enum_name: "SessionMessageRoleProto".to_owned(),
                value,
            })
    }
}

impl From<SessionMessageRoleDto> for i32 {
    fn from(dto: SessionMessageRoleDto) -> Self {
        Self::from(SessionMessageRoleProto::from(dto))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionDto {
    pub session_id: SessionIdDto,
    pub graph_id: GraphIdDto,
    pub user_id: UserIdDto,
    pub status: SessionStatusDto,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub role: RoleDto,
}

impl TryFrom<SessionProto> for SessionDto {
    type Error = DtosConversionError;

    fn try_from(proto: SessionProto) -> Result<Self, Self::Error> {
        Ok(Self {
            session_id: proto.session_id.try_into()?,
            graph_id: proto.graph_id.try_into()?,
            user_id: proto.user_id.try_into()?,
            status: proto.status.try_into()?,
            created_at: proto.created_at.to_chrono()?,
            updated_at: proto.updated_at.to_chrono()?,
            role: proto.role.try_into()?,
        })
    }
}

impl From<SessionDto> for SessionProto {
    fn from(dto: SessionDto) -> Self {
        Self {
            session_id: dto.session_id.to_string(),
            graph_id: dto.graph_id.to_string(),
            user_id: dto.user_id.to_string(),
            status: dto.status.into(),
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
            role: dto.role.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionMessageDto {
    pub message_id: SessionMessageIdDto,
    pub session_id: SessionIdDto,
    pub position: i32,
    pub role: SessionMessageRoleDto,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub document_id: Option<SessionDocumentIdDto>,
    pub document_name: Option<String>,
    pub chunk_index: Option<i32>,
}

impl TryFrom<SessionMessageProto> for SessionMessageDto {
    type Error = DtosConversionError;

    fn try_from(proto: SessionMessageProto) -> Result<Self, Self::Error> {
        Ok(Self {
            message_id: proto.message_id.try_into()?,
            session_id: proto.session_id.try_into()?,
            position: proto.position,
            role: proto.role.try_into()?,
            content: proto.content,
            tool_calls: proto.tool_calls,
            tool_call_id: proto.tool_call_id,
            created_at: proto.created_at.to_chrono()?,
            document_id: proto.document_id.map(TryFrom::try_from).transpose()?,
            document_name: proto.document_name,
            chunk_index: proto.chunk_index,
        })
    }
}

impl From<SessionMessageDto> for SessionMessageProto {
    fn from(dto: SessionMessageDto) -> Self {
        Self {
            message_id: dto.message_id.to_string(),
            session_id: dto.session_id.to_string(),
            position: dto.position,
            role: dto.role.into(),
            content: dto.content,
            tool_calls: dto.tool_calls,
            tool_call_id: dto.tool_call_id,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            document_id: dto.document_id.map(|id| id.to_string()),
            document_name: dto.document_name,
            chunk_index: dto.chunk_index,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSessionMessageDto {
    pub role: SessionMessageRoleDto,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<SessionDocumentIdDto>,
    pub document_name: Option<String>,
    pub chunk_index: Option<i32>,
}

impl TryFrom<CreateSessionMessageProto> for CreateSessionMessageDto {
    type Error = DtosConversionError;

    fn try_from(proto: CreateSessionMessageProto) -> Result<Self, Self::Error> {
        Ok(Self {
            role: proto.role.try_into()?,
            content: proto.content,
            tool_calls: proto.tool_calls,
            tool_call_id: proto.tool_call_id,
            document_id: proto.document_id.map(TryFrom::try_from).transpose()?,
            document_name: proto.document_name,
            chunk_index: proto.chunk_index,
        })
    }
}

impl From<CreateSessionMessageDto> for CreateSessionMessageProto {
    fn from(dto: CreateSessionMessageDto) -> Self {
        Self {
            role: dto.role.into(),
            content: dto.content,
            tool_calls: dto.tool_calls,
            tool_call_id: dto.tool_call_id,
            document_id: dto.document_id.map(|id| id.to_string()),
            document_name: dto.document_name,
            chunk_index: dto.chunk_index,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionDocumentDto {
    pub document_id: SessionDocumentIdDto,
    pub session_id: SessionIdDto,
    pub filename: String,
    pub content_hash: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<SessionDocumentProto> for SessionDocumentDto {
    type Error = DtosConversionError;

    fn try_from(proto: SessionDocumentProto) -> Result<Self, Self::Error> {
        Ok(Self {
            document_id: proto.document_id.try_into()?,
            session_id: proto.session_id.try_into()?,
            filename: proto.filename,
            content_hash: proto.content_hash,
            content: proto.content,
            created_at: proto.created_at.to_chrono()?,
        })
    }
}

impl From<SessionDocumentDto> for SessionDocumentProto {
    fn from(dto: SessionDocumentDto) -> Self {
        Self {
            document_id: dto.document_id.to_string(),
            session_id: dto.session_id.to_string(),
            filename: dto.filename,
            content_hash: dto.content_hash,
            content: dto.content,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
        }
    }
}
