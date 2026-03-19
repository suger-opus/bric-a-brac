use crate::domain::models::{
    CreateSessionMessageModel, CreateSessionModel, SessionIdModel, SessionMessageModel,
    SessionMessageRoleModel, SessionModel,
};
use bric_a_brac_dtos::{DtosConversionError, GraphIdDto};
use bric_a_brac_protos::metadata::{
    NewSessionMessageProto, SessionMessageProto, SessionProto,
};
use crate::application::dtos::UserIdDto;
use prost_types::Timestamp;
use bric_a_brac_dtos::utils::ProtoTimestampExt;
use std::str::FromStr;

// --- SessionModel → SessionProto ---

impl From<SessionModel> for SessionProto {
    fn from(model: SessionModel) -> Self {
        Self {
            session_id: model.session_id.to_string(),
            graph_id: model.graph_id.to_string(),
            user_id: model.user_id.to_string(),
            status: model.status.to_string(),
            created_at: Option::<Timestamp>::from_chrono(model.created_at),
            updated_at: Option::<Timestamp>::from_chrono(model.updated_at),
        }
    }
}

// --- SessionMessageModel → SessionMessageProto ---

impl From<SessionMessageModel> for SessionMessageProto {
    fn from(model: SessionMessageModel) -> Self {
        Self {
            message_id: model.message_id.to_string(),
            session_id: model.session_id.to_string(),
            position: model.position,
            role: model.role.to_string(),
            content: model.content,
            tool_calls: model.tool_calls.map(|v| v.to_string()),
            tool_call_id: model.tool_call_id,
            created_at: Option::<Timestamp>::from_chrono(model.created_at),
        }
    }
}

// --- Proto → CreateSessionModel ---

pub fn create_session_from_proto(
    graph_id: String,
    user_id: String,
) -> Result<CreateSessionModel, DtosConversionError> {
    Ok(CreateSessionModel {
        session_id: SessionIdModel::new(),
        graph_id: GraphIdDto::from_str(&graph_id)
            .map_err(|e| DtosConversionError::Uuid { source: e })?
            .into(),
        user_id: UserIdDto::from_str(&user_id)
            .map_err(|e| DtosConversionError::Uuid { source: e })?
            .into(),
    })
}

// --- NewSessionMessageProto → CreateSessionMessageModel ---

pub fn create_messages_from_proto(
    session_id: SessionIdModel,
    start_position: i32,
    messages: Vec<NewSessionMessageProto>,
) -> Result<Vec<CreateSessionMessageModel>, DtosConversionError> {
    messages
        .into_iter()
        .enumerate()
        .map(|(i, msg)| {
            let role = msg
                .role
                .parse::<SessionMessageRoleModel>()
                .map_err(|_| DtosConversionError::Enum {
                    name: "SessionMessageRole".to_string(),
                    value: 0,
                })?;
            let tool_calls = msg
                .tool_calls
                .map(|s| serde_json::from_str(&s))
                .transpose()
                .map_err(|_| DtosConversionError::NoField {
                    field_name: "tool_calls".to_string(),
                })?;

            Ok(CreateSessionMessageModel {
                session_id,
                position: start_position + i as i32 + 1,
                role,
                content: msg.content,
                tool_calls,
                tool_call_id: msg.tool_call_id,
            })
        })
        .collect()
}
