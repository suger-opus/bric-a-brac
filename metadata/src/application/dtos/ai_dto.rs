use bric_a_brac_protos::ai::agent_event_proto;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AgentEventDto {
    Text { content: String },
    ToolCall { tool_call_id: String, name: String, arguments: String },
    ToolResult { tool_call_id: String, content: String },
    Done { summary: String },
    Error { message: String },
}

impl AgentEventDto {
    pub fn event_name(&self) -> &'static str {
        match self {
            AgentEventDto::Text { .. } => "text",
            AgentEventDto::ToolCall { .. } => "tool_call",
            AgentEventDto::ToolResult { .. } => "tool_result",
            AgentEventDto::Done { .. } => "done",
            AgentEventDto::Error { .. } => "error",
        }
    }
}

impl From<Option<agent_event_proto::Event>> for AgentEventDto {
    fn from(event: Option<agent_event_proto::Event>) -> Self {
        match event {
            Some(agent_event_proto::Event::Text(t)) => AgentEventDto::Text { content: t.content },
            Some(agent_event_proto::Event::ToolCall(tc)) => AgentEventDto::ToolCall {
                tool_call_id: tc.tool_call_id,
                name: tc.name,
                arguments: tc.arguments,
            },
            Some(agent_event_proto::Event::ToolResult(tr)) => AgentEventDto::ToolResult {
                tool_call_id: tr.tool_call_id,
                content: tr.content,
            },
            Some(agent_event_proto::Event::Done(d)) => AgentEventDto::Done { summary: d.summary },
            Some(agent_event_proto::Event::Error(e)) => AgentEventDto::Error { message: e.message },
            None => AgentEventDto::Error { message: "empty event".to_string() },
        }
    }
}
