use crate::infrastructure::config::OpenRouterConfig;
use anyhow::Context;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
    json_schema: JsonSchemaFormat,
}

#[derive(Serialize)]
struct JsonSchemaFormat {
    name: String,
    strict: bool,
    schema: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Deserialize)]
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

    pub async fn chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        schema: serde_json::Value,
        previous_error: Option<&str>,
    ) -> anyhow::Result<serde_json::Value> {
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
        };

        tracing::debug!("Calling OpenRouter API with model: {}", self.default_model);

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
            .context("Failed to call OpenRouter API")?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .context("Failed to read response body")?;

        if !status.is_success() {
            anyhow::bail!(
                "OpenRouter API returned error {}: {}",
                status,
                response_text
            );
        }

        tracing::debug!("OpenRouter response: {}", response_text);

        let chat_response: ChatResponse =
            serde_json::from_str(&response_text).context("Failed to parse OpenRouter response")?;

        let content = chat_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in OpenRouter response"))?
            .message
            .content
            .clone();

        serde_json::from_str(&content).context("Failed to parse generated schema JSON")
    }

    pub fn openai_to_structured_output_schema(
        openapi_spec: &serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let mut schema = openapi_spec
            .get("components")
            .and_then(|c| c.get("schemas"))
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No schemas in OpenAPI spec"))?;

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

        Ok(schema)
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

        let result = OpenRouterClient::openai_to_structured_output_schema(&openapi_spec).unwrap();
        let schema = result.get("CreateNodeSchemaDto").unwrap();

        assert!(schema.get("$defs").is_some());
        assert!(schema.get("definitions").is_none());
        assert_eq!(schema.get("additionalProperties"), Some(&json!(false)));
    }
}
