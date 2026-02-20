use std::str::Utf8Error;

use crate::{
    infrastructure::clients::{MetadataClient, OpenRouterClient},
    presentation::errors::AppError,
};

pub struct SchemaService {
    openrouter_client: OpenRouterClient,
    metadata_client: MetadataClient,
}

impl SchemaService {
    pub fn new(openrouter_client: OpenRouterClient, metadata_client: MetadataClient) -> Self {
        Self {
            openrouter_client,
            metadata_client,
        }
    }

    #[tracing::instrument(skip(self, file_content))]
    pub async fn generate_schema(&self, file_content: &[u8]) -> Result<String, AppError> {
        let openapi_spec = self.metadata_client.clone().get_openapi_spec().await?;
        let schema_spec = OpenRouterClient::openai_to_structured_output_schema(&openapi_spec);

        let parsed_data = parse_file(file_content).map_err(|err| AppError::FileParsing {
            message: "Failed to parse file".to_string(),
            source: err,
        })?;
        let system_prompt = build_system_prompt();
        let user_prompt = build_user_prompt(&parsed_data);

        let mut previous_errors = None;
        let mut schema_str = "".to_string();
        for attempt in 1..=1 {
            tracing::info!(attempt, "Generating schema");

            let generated_schema = self
                .openrouter_client
                .chat(
                    &system_prompt,
                    &user_prompt,
                    schema_spec.clone(),
                    previous_errors.as_deref(),
                )
                .await?;
            let generated_schema = rename_attributes_to_properties(generated_schema);

            match self.validate_schema(&generated_schema).await {
                Ok(()) => {
                    tracing::info!("Schema validation successful on attempt {}", attempt);
                    schema_str = serde_json::to_string(&generated_schema).map_err(|err| {
                        AppError::JsonToString {
                            message: "Failed to serialize schema".to_string(),
                            source: err,
                        }
                    })?;
                    break;
                }
                Err(validation_errors) => {
                    tracing::warn!(
                        attempt,
                        errors = %validation_errors,
                        "Schema validation failed"
                    );

                    previous_errors = Some(validation_errors);

                    if attempt == 3 {
                        return Err(AppError::SchemaGeneration {
                            message: format!(
                                "Schema generation failed after 3 attempts. Last errors: {}",
                                previous_errors.unwrap()
                            ),
                        });
                    }
                }
            }
        }

        Ok(schema_str)
    }

    #[tracing::instrument(skip(self, schema))]
    async fn validate_schema(&self, schema: &serde_json::Value) -> Result<(), String> {
        let schema = serde_json::to_string(schema)
            .map_err(|e| format!("Failed to serialize schema: {}", e))?;

        let response = self
            .metadata_client
            .clone()
            .validate_schema(schema)
            .await
            .map_err(|e| format!("gRPC call failed: {}", e))?;

        if response.is_valid {
            Ok(())
        } else {
            let errors = response
                .errors
                .iter()
                .map(|e| format!("  - {}: {}", e.field, e.message))
                .collect::<Vec<_>>()
                .join(
                    "
                ",
                );
            Err(errors)
        }
    }
}

fn parse_file(content: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(content)
}

fn build_system_prompt() -> String {
    r##"You are a graph schema generator assistant. Your task is to analyze data and generate a graph SCHEMA — not the data itself.
    
    A graph schema defines the TYPES of entities and relationships, not specific instances.
    
    Rules:
    - Nodes represent ENTITY TYPES (e.g., 'Person', 'Location', 'Company') — not specific people or places
    - Edges represent RELATIONSHIP TYPES between node types (e.g., 'FRIEND_OF', 'BORN_IN') — not specific relationships
    - Attributes define the PROPERTIES that instances of a node/edge type can have (e.g., 'Name', 'Eye Color', 'Birth Year') — not the actual values
    - label must always be Title Case for nodes/edges (e.g., 'Person', 'FriendOf')
    - formatted_label must always be snake_case (e.g., 'person', 'birth_year', 'friend_of')
    - colors must be a visually distinct hex color per node/edge type
    Think of it like defining a database schema, not inserting rows into it."##.to_string()
}

fn build_user_prompt(data: &str) -> String {
    format!(
        r##"Analyze the following data and generate a graph schema:

{}

Generate a complete graph schema with nodes, edges, and their attributes."##,
        data
    )
}

/// Recursively renames every object key `"attributes"` back to `"properties"`,
/// reversing the rename applied by `openai_to_structured_output_schema` before
/// the response is forwarded to the metadata service.
pub fn rename_attributes_to_properties(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let new_map = map
                .into_iter()
                .map(|(k, v)| {
                    let new_key = if k == "attributes" {
                        "properties".to_string()
                    } else {
                        k
                    };
                    (new_key, rename_attributes_to_properties(v))
                })
                .collect();
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(arr) => serde_json::Value::Array(
            arr.into_iter()
                .map(rename_attributes_to_properties)
                .collect(),
        ),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_rename_attributes_to_properties() {
        let input = json!({
            "edges": [
                {
                    "attributes": [
                        {
                            "formatted_label": "friend_of",
                            "label": "Friend Of",
                            "metadata": { "options": null },
                            "property_type": "Boolean"
                        }
                    ],
                    "color": "#FF5733",
                    "formatted_label": "friend_of",
                    "label": "Friend Of"
                },
                {
                    "attributes": [
                        {
                            "formatted_label": "born_in",
                            "label": "Born In",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#33FF57",
                    "formatted_label": "born_in",
                    "label": "Born In"
                }
            ],
            "nodes": [
                {
                    "attributes": [
                        {
                            "formatted_label": "name",
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "eye_color",
                            "label": "Eye Color",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "birth_year",
                            "label": "Birth Year",
                            "metadata": { "options": null },
                            "property_type": "Number"
                        }
                    ],
                    "color": "#3498DB",
                    "formatted_label": "person",
                    "label": "Person"
                },
                {
                    "attributes": [
                        {
                            "formatted_label": "name",
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "location_name",
                            "label": "Location Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "country",
                            "label": "Country",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#9B59B6",
                    "formatted_label": "location",
                    "label": "Location"
                }
            ]
        });

        let expected = json!({
            "edges": [
                {
                    "properties": [
                        {
                            "formatted_label": "friend_of",
                            "label": "Friend Of",
                            "metadata": { "options": null },
                            "property_type": "Boolean"
                        }
                    ],
                    "color": "#FF5733",
                    "formatted_label": "friend_of",
                    "label": "Friend Of"
                },
                {
                    "properties": [
                        {
                            "formatted_label": "born_in",
                            "label": "Born In",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#33FF57",
                    "formatted_label": "born_in",
                    "label": "Born In"
                }
            ],
            "nodes": [
                {
                    "properties": [
                        {
                            "formatted_label": "name",
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "eye_color",
                            "label": "Eye Color",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "birth_year",
                            "label": "Birth Year",
                            "metadata": { "options": null },
                            "property_type": "Number"
                        }
                    ],
                    "color": "#3498DB",
                    "formatted_label": "person",
                    "label": "Person"
                },
                {
                    "properties": [
                        {
                            "formatted_label": "name",
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "location_name",
                            "label": "Location Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "formatted_label": "country",
                            "label": "Country",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#9B59B6",
                    "formatted_label": "location",
                    "label": "Location"
                }
            ]
        });

        assert_eq!(rename_attributes_to_properties(input), expected);
    }

    #[test]
    fn test_rename_preserves_non_attributes_keys() {
        let input = json!({
            "nodes": [{ "label": "X", "attributes": [], "color": "#000" }]
        });
        let result = rename_attributes_to_properties(input);
        assert!(result.pointer("/nodes/0/properties").is_some());
        assert!(result.pointer("/nodes/0/attributes").is_none());
        assert!(result.pointer("/nodes/0/label").is_some());
        assert!(result.pointer("/nodes/0/color").is_some());
    }
}
