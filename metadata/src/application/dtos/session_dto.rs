use crate::domain::models::{SessionIdModel, SessionMessageModel, SessionModel};
use bric_a_brac_dtos::GraphIdDto;
use bric_a_brac_dtos::utils::ProtoTimestampExt;
use bric_a_brac_id::id;
use bric_a_brac_protos::metadata::{SessionMessageProto, SessionProto};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

id!(SessionIdDto);

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

// ---------------------------------------------------------------------------
// DTOs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSessionDto {
    pub graph_id: GraphIdDto,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionDto {
    pub session_id: SessionIdDto,
    pub graph_id: String,
    pub user_id: String,
    pub status: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SessionModel> for SessionDto {
    fn from(model: SessionModel) -> Self {
        Self {
            session_id: model.session_id.into(),
            graph_id: model.graph_id.to_string(),
            user_id: model.user_id.to_string(),
            status: model.status.to_string(),
            role: model.role.to_string(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionMessageDto {
    pub message_id: String,
    pub session_id: String,
    pub position: i32,
    pub role: String,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<String>,
    pub document_name: Option<String>,
    pub chunk_index: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl From<SessionMessageModel> for SessionMessageDto {
    fn from(model: SessionMessageModel) -> Self {
        Self {
            message_id: model.message_id.to_string(),
            session_id: model.session_id.to_string(),
            position: model.position,
            role: model.role.to_string(),
            content: model.content,
            tool_calls: model.tool_calls,
            tool_call_id: model.tool_call_id,
            document_id: model.document_id.map(|id| id.to_string()),
            document_name: model.document_name,
            chunk_index: model.chunk_index,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug)]
pub struct CreateSessionMessageDto {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<String>,
    pub chunk_index: Option<i32>,
}

// ---------------------------------------------------------------------------
// Dto → Proto conversions (for gRPC responses)
// ---------------------------------------------------------------------------

impl From<SessionDto> for SessionProto {
    fn from(dto: SessionDto) -> Self {
        Self {
            session_id: dto.session_id.to_string(),
            graph_id: dto.graph_id,
            user_id: dto.user_id,
            status: dto.status,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
            updated_at: Option::<Timestamp>::from_chrono(dto.updated_at),
            role: dto.role,
        }
    }
}

impl From<SessionMessageDto> for SessionMessageProto {
    fn from(dto: SessionMessageDto) -> Self {
        Self {
            message_id: dto.message_id,
            session_id: dto.session_id,
            position: dto.position,
            role: dto.role,
            content: dto.content,
            tool_calls: dto.tool_calls.map(|v| v.to_string()),
            tool_call_id: dto.tool_call_id,
            document_id: dto.document_id,
            document_name: dto.document_name,
            chunk_index: dto.chunk_index,
            created_at: Option::<Timestamp>::from_chrono(dto.created_at),
        }
    }
}
