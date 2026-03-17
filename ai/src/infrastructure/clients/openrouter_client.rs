use crate::{
    infrastructure::config::OpenRouterConfig, presentation::errors::OpenRouterClientError,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct Plugin {
    id: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    response_format: Option<ResponseFormat>,
    plugins: Vec<Plugin>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

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
    content: String,
}

#[derive(Clone)]
pub struct OpenRouterClient {
    api_key: SecretString,
    default_model: String,
    client: reqwest::Client,
}

impl OpenRouterClient {
    pub fn new(config: &OpenRouterConfig) -> Self {
        Self {
            api_key: config.api_key().clone(),
            default_model: config.default_model().to_string(),
            client: reqwest::Client::new(),
        }
    }

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
            type_: "json_schema".to_string(),
            json_schema: JsonSchemaFormat {
                name: "schema_generation".to_string(),
                strict: true,
                schema: s,
            },
        });

        let plugins = match response_format {
            Some(_) => vec![Plugin {
                id: "response-healing".to_string(),
            }],
            None => vec![],
        };

        let request = ChatRequest {
            model: self.default_model.clone(),
            messages,
            response_format,
            plugins,
        };
        // tracing::debug!(
        //     "Sending request to OpenRouter API: {}",
        //     serde_json::to_string_pretty(&request).unwrap_or_default()
        // );

        let response = self
            .client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", &self.api_key.expose_secret()),
            )
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|err| OpenRouterClientError::Request {
                message: "Failed to call OpenRouter API".to_string(),
                source: err,
            })?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|err| OpenRouterClientError::ReadResponse {
                    message: "Failed to read OpenRouter API response".to_string(),
                    source: err,
                })?;
        // tracing::debug!(
        //     "Received response from OpenRouter API: {}",
        //     serde_json::to_string_pretty(
        //         &serde_json::from_str::<serde_json::Value>(&response_text).unwrap_or_default()
        //     )
        //     .unwrap_or_default()
        // );

        if !status.is_success() {
            return Err(OpenRouterClientError::NoSuccessResponse {
                status,
                body: response_text,
            });
        }

        let chat_response: ChatResponse = serde_json::from_str(&response_text).map_err(|err| {
            OpenRouterClientError::Deserialization {
                message: "Failed to deserialize ChatResponse from OpenRouter response_text"
                    .to_string(),
                source: err,
            }
        })?;

        let content = chat_response
            .choices
            .first()
            .ok_or_else(|| OpenRouterClientError::ResponseFormat {
                message: "No choices in OpenRouter response".to_string(),
            })?
            .message
            .content
            .clone();

        let value = match is_structured_output_needed {
            true => serde_json::from_str::<serde_json::Value>(&content).map_err(|err| {
                OpenRouterClientError::Deserialization {
                    message: "Failed to deserialize content field as JSON value".to_string(),
                    source: err,
                }
            })?,
            false => serde_json::Value::String(content.clone()),
        };

        Ok(ChatResult {
            raw_content: content,
            value,
        })
    }
}
