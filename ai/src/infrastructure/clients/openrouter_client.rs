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
    response_format: ResponseFormat,
    plugins: Vec<Plugin>,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
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
        skip(self, system_prompt, user_prompt, schema, previous_error)
    )]
    pub async fn chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        schema: serde_json::Value,
        previous_error: Option<&str>,
    ) -> Result<serde_json::Value, OpenRouterClientError> {
        let user_content = if let Some(errors) = previous_error {
            format!(
                "{}\n\nPrevious attempt had validation errors:\n{}",
                user_prompt, errors
            )
        } else {
            user_prompt.to_string()
        };

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_content,
            },
        ];

        let response_format = ResponseFormat {
            type_: "json_schema".to_string(),
            json_schema: JsonSchemaFormat {
                name: "schema_generation".to_string(),
                strict: true,
                schema,
            },
        };

        let request = ChatRequest {
            model: self.default_model.clone(),
            messages,
            response_format,
            plugins: vec![Plugin {
                id: "response-healing".to_string(),
            }],
        };

        tracing::debug!(
            "Calling OpenRouter API with request: {}",
            serde_json::to_string_pretty(&request).unwrap()
        );

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

        tracing::debug!(
            "Response: {}",
            serde_json::to_string_pretty(
                &serde_json::from_str::<serde_json::Value>(&response_text).unwrap()
            )
            .unwrap()
        );

        if !status.is_success() {
            return Err(OpenRouterClientError::NoSuccessResponse {
                status,
                body: response_text,
            });
        }

        let chat_response: ChatResponse = serde_json::from_str(&response_text)
            .map_err(|err| OpenRouterClientError::Deserialization { source: err })?;

        let content = chat_response
            .choices
            .first()
            .ok_or_else(|| OpenRouterClientError::Response {
                reason: "No choices in OpenRouter response".to_string(),
            })?
            .message
            .content
            .clone();

        serde_json::from_str(&content)
            .map_err(|err| OpenRouterClientError::Deserialization { source: err })
    }

    #[tracing::instrument(level = "debug", skip(openapi_spec))]
    pub fn openai_to_structured_output_schema(
        openapi_spec: &serde_json::Value,
    ) -> serde_json::Value {
        let mut schema = openapi_spec
            .get("components")
            .and_then(|c| c.get("schemas"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));

        // Remove $schema field if present
        if let Some(obj) = schema.as_object_mut() {
            for (_name, schema_def) in obj.iter_mut() {
                if let Some(schema_obj) = schema_def.as_object_mut() {
                    schema_obj.remove("$schema");
                    // Rename definitions to $defs if present
                    if let Some(definitions) = schema_obj.remove("definitions") {
                        schema_obj.insert("$defs".to_string(), definitions);
                    }
                    // Add additionalProperties: false for strict mode
                    if schema_obj.get("type").and_then(|t| t.as_str()) == Some("object") {
                        schema_obj
                            .entry("additionalProperties".to_string())
                            .or_insert(serde_json::Value::Bool(false));
                    }
                }
            }
        }

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transform_schema() {
        let openapi_spec = json!({
            "components": {
                "schemas": {
                    "CreateNodeSchemaDto": {
                        "type": "object",
                        "properties": {
                            "label": {"type": "string"}
                        },
                        "definitions": {
                            "SomeType": {}
                        }
                    }
                }
            }
        });

        let result = OpenRouterClient::openai_to_structured_output_schema(&openapi_spec);
        let schema = result.get("CreateNodeSchemaDto").unwrap();

        assert!(schema.get("$defs").is_some());
        assert!(schema.get("definitions").is_none());
        assert_eq!(schema.get("additionalProperties"), Some(&json!(false)));
    }
}
