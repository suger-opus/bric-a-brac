use super::{SessionDocumentIdModel, SessionIdModel};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(SessionMessageIdModel);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum SessionMessageRoleModel {
    System,
    User,
    Assistant,
    Tool,
}

impl std::fmt::Display for SessionMessageRoleModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::System => write!(f, "system"),
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
            Self::Tool => write!(f, "tool"),
        }
    }
}

impl std::str::FromStr for SessionMessageRoleModel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Self::System),
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            "tool" => Ok(Self::Tool),
            other => Err(format!("Invalid message role: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionMessageModel {
    pub message_id: SessionMessageIdModel,
    pub session_id: SessionIdModel,
    pub position: i32,
    pub role: SessionMessageRoleModel,
    pub content: String,
    pub tool_calls: Option<serde_json::Value>,
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
    pub tool_calls: Option<serde_json::Value>,
    pub tool_call_id: Option<String>,
    pub document_id: Option<SessionDocumentIdModel>,
    pub chunk_index: Option<i32>,
}
