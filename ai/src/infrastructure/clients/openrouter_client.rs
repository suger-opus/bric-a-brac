use std::collections::HashSet;

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
        let schemas = openapi_spec
            .get("components")
            .and_then(|c| c.get("schemas"))
            .and_then(|s| s.as_object())
            .cloned()
            .unwrap_or_default();

        if schemas.is_empty() {
            return serde_json::Value::Object(serde_json::Map::new());
        }

        // Find all referenced schema names
        let mut referenced = HashSet::new();
        for (_name, schema) in &schemas {
            collect_schema_refs(schema, &mut referenced);
        }

        // Root schema is the one not referenced by any other schema
        let root_name = schemas
            .keys()
            .find(|name| !referenced.contains(name.as_str()))
            .cloned();

        let root_name = match root_name {
            Some(name) => name,
            None => return serde_json::Value::Object(serde_json::Map::new()),
        };

        let root_schema = schemas.get(&root_name).cloned().unwrap();
        resolve_schema(&root_schema, &schemas)
    }
}

fn collect_schema_refs(value: &serde_json::Value, refs: &mut HashSet<String>) {
    match value {
        serde_json::Value::Object(obj) => {
            if let Some(ref_val) = obj.get("$ref") {
                if let Some(ref_str) = ref_val.as_str() {
                    if let Some(name) = ref_str.strip_prefix("#/components/schemas/") {
                        refs.insert(name.to_string());
                    }
                }
            }
            for (_, v) in obj {
                collect_schema_refs(v, refs);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                collect_schema_refs(v, refs);
            }
        }
        _ => {}
    }
}

fn resolve_schema(
    schema: &serde_json::Value,
    all_schemas: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Value {
    match schema {
        serde_json::Value::Object(obj) => {
            // Resolve $ref first
            if let Some(ref_val) = obj.get("$ref") {
                if let Some(ref_str) = ref_val.as_str() {
                    if let Some(name) = ref_str.strip_prefix("#/components/schemas/") {
                        if let Some(referenced) = all_schemas.get(name) {
                            return resolve_schema(referenced, all_schemas);
                        }
                    }
                }
            }

            let schema_type = obj.get("type").and_then(|t| t.as_str());

            if schema_type == Some("object") {
                let mut result = serde_json::Map::new();
                result.insert(
                    "type".to_string(),
                    serde_json::Value::String("object".to_string()),
                );
                result.insert(
                    "additionalProperties".to_string(),
                    serde_json::Value::Bool(false),
                );

                let properties = obj.get("properties").and_then(|p| p.as_object());
                let required = obj.get("required").and_then(|r| r.as_array());

                if let Some(props) = properties {
                    // If required is specified, only keep required properties.
                    // Otherwise, keep all properties and make them all required.
                    let required_keys: Vec<String> = if let Some(req) = required {
                        req.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    } else {
                        props.keys().cloned().collect()
                    };

                    // Rename the domain field "properties" → "attributes" to avoid
                    // clashing with the JSON Schema keyword "properties".
                    let rename = |k: &str| {
                        if k == "properties" {
                            "attributes".to_string()
                        } else {
                            k.to_string()
                        }
                    };

                    result.insert(
                        "required".to_string(),
                        serde_json::Value::Array(
                            required_keys
                                .iter()
                                .map(|k| serde_json::Value::String(rename(k)))
                                .collect(),
                        ),
                    );

                    let mut resolved_props = serde_json::Map::new();
                    for key in &required_keys {
                        if let Some(prop_schema) = props.get(key) {
                            resolved_props
                                .insert(rename(key), resolve_schema(prop_schema, all_schemas));
                        }
                    }
                    result.insert(
                        "properties".to_string(),
                        serde_json::Value::Object(resolved_props),
                    );
                }

                serde_json::Value::Object(result)
            } else {
                // Non-object: copy all fields except 'example', resolving nested values
                let mut result = serde_json::Map::new();
                for (key, value) in obj {
                    if key == "example" {
                        continue;
                    }
                    result.insert(key.clone(), resolve_schema(value, all_schemas));
                }
                serde_json::Value::Object(result)
            }
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(|v| resolve_schema(v, all_schemas)).collect())
        }
        other => other.clone(),
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
              "CreateEdgeSchemaDto": {
                "properties": {
                  "color": {
                    "pattern": "^#[0-9A-Fa-f]{6}$",
                    "type": "string"
                  },
                  "label": {
                    "maxLength": 25,
                    "minLength": 1,
                    "type": "string"
                  },
                  "properties": {
                    "items": {
                      "$ref": "#/components/schemas/CreatePropertySchemaDto"
                    },
                    "type": "array"
                  }
                },
                "required": [
                  "label",
                  "color",
                  "properties"
                ],
                "type": "object"
              },
              "CreateGraphSchemaDto": {
                "properties": {
                  "edges": {
                    "items": {
                      "$ref": "#/components/schemas/CreateEdgeSchemaDto"
                    },
                    "type": "array"
                  },
                  "nodes": {
                    "items": {
                      "$ref": "#/components/schemas/CreateNodeSchemaDto"
                    },
                    "type": "array"
                  }
                },
                "required": [
                  "nodes",
                  "edges"
                ],
                "type": "object"
              },
              "CreateNodeSchemaDto": {
                "properties": {
                  "color": {
                    "pattern": "^#[0-9A-Fa-f]{6}$",
                    "type": "string"
                  },
                  "label": {
                    "maxLength": 25,
                    "minLength": 1,
                    "type": "string"
                  },
                  "properties": {
                    "items": {
                      "$ref": "#/components/schemas/CreatePropertySchemaDto"
                    },
                    "type": "array"
                  }
                },
                "required": [
                  "label",
                  "color",
                  "properties"
                ],
                "type": "object"
              },
              "CreatePropertyMetadataDto": {
                "properties": {
                  "options": {
                    "items": {
                      "$ref": "#/components/schemas/OptionString"
                    },
                    "type": [
                      "array",
                      "null"
                    ]
                  }
                },
                "type": "object"
              },
              "CreatePropertySchemaDto": {
                "properties": {
                  "edge_schema_id": {
                    "oneOf": [
                      {
                        "type": "null"
                      },
                      {
                        "$ref": "#/components/schemas/EdgeSchemaId"
                      }
                    ]
                  },
                  "label": {
                    "maxLength": 25,
                    "minLength": 1,
                    "type": "string"
                  },
                  "metadata": {
                    "$ref": "#/components/schemas/CreatePropertyMetadataDto"
                  },
                  "node_schema_id": {
                    "oneOf": [
                      {
                        "type": "null"
                      },
                      {
                        "$ref": "#/components/schemas/NodeSchemaId"
                      }
                    ]
                  },
                  "property_type": {
                    "$ref": "#/components/schemas/PropertyTypeDto"
                  }
                },
                "required": [
                  "label",
                  "property_type",
                  "metadata"
                ],
                "type": "object"
              },
              "EdgeSchemaId": {
                "format": "uuid",
                "type": "string"
              },
              "NodeSchemaId": {
                "format": "uuid",
                "type": "string"
              },
              "OptionString": {
                "maxLength": 50,
                "minLength": 1,
                "type": "string"
              },
              "PropertyMetadataDto": {
                "properties": {
                  "options": {
                    "items": {
                      "type": "string"
                    },
                    "type": [
                      "array",
                      "null"
                    ]
                  }
                },
                "type": "object"
              },
              "PropertyTypeDto": {
                "enum": [
                  "Number",
                  "String",
                  "Boolean",
                  "Select"
                ],
                "type": "string"
              }
            }
          },
          "info": {
            "description": "",
            "license": {
              "name": ""
            },
            "title": "metadata",
            "version": "0.1.0"
          },
          "openapi": "3.1.0",
          "paths": {

          }
        });

        let result = OpenRouterClient::openai_to_structured_output_schema(&openapi_spec);

        let property_schema = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["label", "property_type", "metadata"],
            "properties": {
                "label": { "type": "string", "minLength": 1, "maxLength": 25 },
                "property_type": { "type": "string", "enum": ["Number", "String", "Boolean", "Select"] },
                "metadata": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["options"],
                    "properties": {
                        "options": { "type": ["array", "null"], "items": { "minLength": 1, "maxLength": 50, "type": "string" } }
                    }
                }
            }
        });

        let node_schema = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["label", "color", "attributes"],
            "properties": {
                "label": { "type": "string", "minLength": 1, "maxLength": 25 },
                "color": { "type": "string", "pattern": "^#[0-9A-Fa-f]{6}$" },
                "attributes": { "type": "array", "items": property_schema.clone() }
            }
        });

        let edge_schema = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["label", "color", "attributes"],
            "properties": {
                "label": { "type": "string", "minLength": 1, "maxLength": 25 },
                "color": { "type": "string", "pattern": "^#[0-9A-Fa-f]{6}$" },
                "attributes": { "type": "array", "items": property_schema }
            }
        });

        let expected = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["nodes", "edges"],
            "properties": {
                "nodes": { "type": "array", "items": node_schema },
                "edges": { "type": "array", "items": edge_schema }
            }
        });

        assert_eq!(result, expected);
    }
}
