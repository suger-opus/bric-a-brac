use super::{GraphIdModel, RoleModel, UserIdModel};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(SessionIdModel);
id!(SessionMessageIdModel);
id!(SessionDocumentIdModel);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, sqlx::Type, derive_more::Display)]
#[sqlx(type_name = "session_status_type")]
pub enum SessionStatusModel {
    #[display("active")]
    Active,
    #[display("completed")]
    Completed,
    #[display("error")]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, sqlx::Type, derive_more::Display)]
#[sqlx(type_name = "session_message_role_type")]
pub enum SessionMessageRoleModel {
    #[display("system")]
    System,
    #[display("user")]
    User,
    #[display("assistant")]
    Assistant,
    #[display("tool")]
    Tool,
}

pub struct SessionModel {
    pub session_id: SessionIdModel,
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
    pub status: SessionStatusModel,
    pub role: RoleModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(clippy::struct_field_names)]
pub struct CreateSessionModel {
    pub session_id: SessionIdModel,
    pub graph_id: GraphIdModel,
    pub user_id: UserIdModel,
}

pub struct SessionMessageModel {
    pub message_id: SessionMessageIdModel,
    pub session_id: SessionIdModel,
    pub position: i32,
    pub role: SessionMessageRoleModel,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<SessionDocumentIdModel>,
    pub document_name: Option<String>,
    pub chunk_index: Option<i32>,
    pub created_at: DateTime<Utc>,
}

pub struct CreateSessionMessageModel {
    pub session_id: SessionIdModel,
    pub position: i32,
    pub role: SessionMessageRoleModel,
    pub content: String,
    pub tool_calls: Option<String>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<SessionDocumentIdModel>,
    pub chunk_index: Option<i32>,
}

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
