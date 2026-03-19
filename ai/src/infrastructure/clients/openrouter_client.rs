use crate::infrastructure::{
    config::OpenRouterConfig, errors::OpenRouterClientError, http_retry::send_with_retry,
};
use futures_util::StreamExt;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

// --- Request types ---

#[derive(Debug, Serialize)]
struct Plugin {
    id: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    plugins: Vec<Plugin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
    stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_owned(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_owned(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: None,
        }
    }

    #[must_use] 
    pub fn assistant(content: Option<String>, tool_calls: Option<Vec<ToolCall>>) -> Self {
        Self {
            role: "assistant".to_owned(),
            content,
            tool_calls,
            tool_call_id: None,
        }
    }

    pub fn tool(tool_call_id: String, content: impl Into<String>) -> Self {
        Self {
            role: "tool".to_owned(),
            content: Some(content.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
        }
    }
}

// --- Tool types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

// --- Response types (non-streaming) ---

pub struct ChatResult {
    pub raw_content: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
    json_schema: JsonSchemaFormat,
}

#[derive(Debug, Serialize)]
struct JsonSchemaFormat {
    name: String,
    strict: bool,
    schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatMessage {
    content: Option<String>,
    #[allow(dead_code)]
    tool_calls: Option<Vec<ToolCall>>,
}

// --- Streaming response types ---

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
    tool_calls: Option<Vec<StreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct StreamToolCall {
    index: usize,
    id: Option<String>,
    function: Option<StreamFunctionCall>,
}

#[derive(Debug, Deserialize)]
struct StreamFunctionCall {
    name: Option<String>,
    arguments: Option<String>,
}

/// A complete response from a streaming chat call
pub struct StreamChatResult {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}

// --- Client ---

#[derive(Clone)]
pub struct OpenRouterClient {
    api_key: SecretString,
    default_model: String,
    client: reqwest::Client,
}

impl OpenRouterClient {
    #[must_use] 
    pub fn new(config: &OpenRouterConfig) -> Self {
        Self {
            api_key: config.api_key().clone(),
            default_model: config.default_model().to_owned(),
            client: reqwest::Client::new(),
        }
    }

    /// Non-streaming chat with optional structured JSON output
    #[tracing::instrument(
        level = "debug",
        name = "openrouter_client.chat",
        skip(self, messages, schema)
    )]
    pub async fn chat(
        &self,
        messages: Vec<Message>,
        schema: Option<serde_json::Value>,
    ) -> Result<ChatResult, OpenRouterClientError> {
        tracing::debug!(
            message_count = messages.len(),
            has_schema = schema.is_some(),
        );

        let is_structured_output_needed = schema.is_some();

        let response_format = schema.map(|s| ResponseFormat {
            type_: "json_schema".to_owned(),
            json_schema: JsonSchemaFormat {
                name: "schema_generation".to_owned(),
                strict: true,
                schema: s,
            },
        });

        let plugins = match response_format {
            Some(_) => vec![Plugin {
                id: "response-healing".to_owned(),
            }],
            None => vec![],
        };

        let request = ChatRequest {
            model: self.default_model.clone(),
            messages,
            response_format,
            plugins,
            tools: None,
            stream: false,
        };

        let response = send_with_retry("OpenRouter chat", || {
            self.client
                .post("https://openrouter.ai/api/v1/chat/completions")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.api_key.expose_secret()),
                )
                .header("Content-Type", "application/json")
                .json(&request)
        })
        .await?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|err| OpenRouterClientError::ReadResponse {
                    message: "Failed to read OpenRouter API response".to_owned(),
                    source: err,
                })?;

        if !status.is_success() {
            return Err(OpenRouterClientError::NoSuccessResponse {
                status,
                body: response_text,
            });
        }

        let chat_response: ChatResponse = serde_json::from_str(&response_text).map_err(|err| {
            OpenRouterClientError::Deserialization {
                message: "Failed to deserialize ChatResponse from OpenRouter response_text".to_owned(),
                source: err,
            }
        })?;

        let content = chat_response
            .choices
            .first()
            .ok_or_else(|| OpenRouterClientError::ResponseFormat {
                message: "No choices in OpenRouter response".to_owned(),
            })?
            .message
            .content
            .clone()
            .unwrap_or_default();

        let value = if is_structured_output_needed { serde_json::from_str::<serde_json::Value>(&content).map_err(|err| {
            OpenRouterClientError::Deserialization {
                message: "Failed to deserialize content field as JSON value".to_owned(),
                source: err,
            }
        })? } else { serde_json::Value::String(content.clone()) };

        Ok(ChatResult {
            raw_content: content,
            value,
        })
    }

    /// Streaming chat with tool calling support.
    /// Collects the full streamed response into a `StreamChatResult`.
    #[tracing::instrument(
        level = "debug",
        name = "openrouter_client.chat_stream",
        skip(self, messages, tools)
    )]
    pub async fn chat_stream(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ToolDefinition>>,
    ) -> Result<StreamChatResult, OpenRouterClientError> {
        tracing::debug!(
            message_count = messages.len(),
            has_tools = tools.is_some(),
        );

        let request = ChatRequest {
            model: self.default_model.clone(),
            messages,
            response_format: None,
            plugins: vec![],
            tools,
            stream: true,
        };

        let response = send_with_retry("OpenRouter chat_stream", || {
            self.client
                .post("https://openrouter.ai/api/v1/chat/completions")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.api_key.expose_secret()),
                )
                .header("Content-Type", "application/json")
                .json(&request)
        })
        .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(OpenRouterClientError::NoSuccessResponse { status, body });
        }

        // Parse SSE stream
        let mut content_parts: Vec<String> = Vec::new();
        let mut tool_calls_builder: Vec<ToolCallBuilder> = Vec::new();

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|err| OpenRouterClientError::ReadResponse {
                message: "Failed to read stream chunk".to_owned(),
                source: err,
            })?;

            buffer.push_str(&String::from_utf8_lossy(&chunk));

            // Process complete SSE lines
            while let Some(line_end) = buffer.find('\n') {
                let line = buffer[..line_end].trim().to_owned();
                buffer = buffer[line_end + 1..].to_string();

                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if let Ok(stream_response) = serde_json::from_str::<StreamResponse>(data) {
                        for choice in &stream_response.choices {
                            if let Some(text) = &choice.delta.content {
                                content_parts.push(text.clone());
                            }
                            if let Some(tool_calls) = &choice.delta.tool_calls {
                                for tc in tool_calls {
                                    // Grow the builder list if needed
                                    while tool_calls_builder.len() <= tc.index {
                                        tool_calls_builder.push(ToolCallBuilder::default());
                                    }
                                    let builder = &mut tool_calls_builder[tc.index];
                                    if let Some(id) = &tc.id {
                                        builder.id = Some(id.clone());
                                    }
                                    if let Some(func) = &tc.function {
                                        if let Some(name) = &func.name {
                                            builder.name = Some(name.clone());
                                        }
                                        if let Some(args) = &func.arguments {
                                            builder.arguments.push_str(args);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        let content = if content_parts.is_empty() {
            None
        } else {
            Some(content_parts.join(""))
        };

        let tool_calls: Vec<ToolCall> = tool_calls_builder
            .into_iter()
            .filter_map(|b| {
                Some(ToolCall {
                    id: b.id?,
                    type_: "function".to_owned(),
                    function: FunctionCall {
                        name: b.name?,
                        arguments: b.arguments,
                    },
                })
            })
            .collect();

        Ok(StreamChatResult {
            content,
            tool_calls,
        })
    }
}

#[derive(Default)]
struct ToolCallBuilder {
    id: Option<String>,
    name: Option<String>,
    arguments: String,
}
