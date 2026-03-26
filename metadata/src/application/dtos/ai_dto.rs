use bric_a_brac_protos::ai::agent_event_proto;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AgentEventDto {
    Text {
        content: String,
    },
    ToolCall {
        tool_call_id: String,
        name: String,
        arguments: String,
    },
    ToolResult {
        tool_call_id: String,
        content: String,
    },
    Done {
        summary: String,
    },
    Error {
        message: String,
    },
    Progress {
        content: String,
    },
}

impl AgentEventDto {
    pub const fn event_name(&self) -> &'static str {
        match self {
            Self::Text { .. } => "text",
            Self::ToolCall { .. } => "tool_call",
            Self::ToolResult { .. } => "tool_result",
            Self::Done { .. } => "done",
            Self::Error { .. } => "error",
            Self::Progress { .. } => "progress",
        }
    }
}

impl From<Option<agent_event_proto::Event>> for AgentEventDto {
    fn from(event: Option<agent_event_proto::Event>) -> Self {
        match event {
            Some(agent_event_proto::Event::Text(t)) => Self::Text { content: t.content },
            Some(agent_event_proto::Event::ToolCall(tc)) => Self::ToolCall {
                tool_call_id: tc.tool_call_id,
                name: tc.name,
                arguments: tc.arguments,
            },
            Some(agent_event_proto::Event::ToolResult(tr)) => Self::ToolResult {
                tool_call_id: tr.tool_call_id,
                content: tr.content,
            },
            Some(agent_event_proto::Event::Done(d)) => Self::Done { summary: d.summary },
            Some(agent_event_proto::Event::Error(err)) => Self::Error {
                message: err.message,
            },
            Some(agent_event_proto::Event::Progress(p)) => Self::Progress { content: p.content },
            None => Self::Error {
                message: "empty event".to_owned(),
            },
        }
    }
}
