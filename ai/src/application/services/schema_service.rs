use crate::{
    infrastructure::clients::{Message, OpenRouterClient},
    presentation::errors::{AppError, OpenRouterClientError},
};
use bric_a_brac_dtos::{generate_graph_schema_doc, CreateGraphSchemaDto};
use std::collections::HashSet;
use validator::Validate;

pub struct SchemaService {
    openrouter_client: OpenRouterClient,
}

impl SchemaService {
    pub fn new(openrouter_client: OpenRouterClient) -> Self {
        Self { openrouter_client }
    }

    #[tracing::instrument(
        level = "trace",
        name = "schema_service.generate_schema",
        skip(self, file_content)
    )]
    pub async fn generate_schema(
        &self,
        file_content: Vec<u8>,
    ) -> Result<CreateGraphSchemaDto, AppError> {
        let openapi_spec = generate_graph_schema_doc();
        let schema_spec = openai_to_structured_output_schema(&openapi_spec);

        let parsed_content =
            std::str::from_utf8(&file_content).map_err(|err| AppError::FileParsing {
                message: "Failed to parse file".to_string(),
                source: err,
            })?;
        let system_prompt = build_system_prompt();
        let user_prompt = build_user_prompt(&parsed_content);

        const MAX_ATTEMPTS: u8 = 3;
        let mut attempt: u8 = 0;
        let mut messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        loop {
            attempt += 1;

            let result = self
                .openrouter_client
                .chat(messages.clone(), Some(schema_spec.clone()))
                .await?;
            let raw_content = result.raw_content.clone();
            let generated_schema = rename_attributes_to_properties(result.value);

            match self.validate_schema(&generated_schema) {
                Ok(schema) => break Ok(schema),
                Err(err) if attempt < MAX_ATTEMPTS => {
                    tracing::warn!(attempt, error = %err, "Schema validation failed, retrying");
                    // Give the model its own previous response so it can see what to fix
                    messages.push(Message {
                        role: "assistant".to_string(),
                        content: raw_content,
                    });
                    messages.push(Message {
                        role: "user".to_string(),
                        content: build_correction_prompt(&err.to_string()),
                    });
                }
                Err(err) => break Err(err),
            }
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "schema_service.validate_schema",
        skip(self, schema)
    )]
    fn validate_schema(
        &self,
        schema: &serde_json::Value,
    ) -> Result<CreateGraphSchemaDto, AppError> {
        let schema =
            serde_json::from_str::<CreateGraphSchemaDto>(&schema.to_string()).map_err(|err| {
                OpenRouterClientError::ResponseConversion {
                    message: "Failed to parse generated schema".to_string(),
                    source: err,
                }
            })?;

        schema
            .validate()
            .map_err(|err| OpenRouterClientError::ResponseValidation { source: err })?;

        Ok(schema)
    }
}

fn build_system_prompt() -> String {
    r##"You are a graph schema generator assistant. Your task is to analyze data and generate a graph SCHEMA — not the data itself.

A graph schema defines the TYPES of entities and relationships, not specific instances.

Rules:
- Nodes represent ENTITY TYPES (e.g., 'Person', 'Location', 'Company') — not specific people or places
- Edges represent RELATIONSHIP TYPES between node types (e.g., 'Friend Of', 'Born In') — not specific relationships
- Attributes define the PROPERTIES that instances of a node/edge type can have (e.g., 'Name', 'Eye Color', 'Birth Year') — not the actual values
- Colors must be a visually distinct hex color per node/edge type
- If a property_type is Select, then metadata.options must be a non-empty list of possible values. Otherwise, metadata.options must be null.

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

fn build_correction_prompt(errors: &str) -> String {
    format!(
        r##"Your previous response contained validation errors that must be fixed:

{}

Please return a corrected response that resolves all of these issues."##,
        errors
    )
}

pub fn openai_to_structured_output_schema(openapi_spec: &serde_json::Value) -> serde_json::Value {
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
    let Some((_, root_schema)) = schemas
        .iter()
        .find(|(name, _)| !referenced.contains(name.as_str()))
    else {
        return serde_json::Value::Object(serde_json::Map::new());
    };

    resolve_schema(root_schema, &schemas)
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
    fn test_openai_to_structured_output_schema() {
        let openapi_spec = generate_graph_schema_doc();
        let result = openai_to_structured_output_schema(&openapi_spec);

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

    #[test]
    fn test_rename_attributes_to_properties() {
        let input = json!({
            "edges": [
                {
                    "attributes": [
                        {
                            "label": "Friend Of",
                            "metadata": { "options": null },
                            "property_type": "Boolean"
                        }
                    ],
                    "color": "#FF5733",
                    "label": "Friend Of"
                },
                {
                    "attributes": [
                        {
                            "label": "Born In",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#33FF57",
                    "label": "Born In"
                }
            ],
            "nodes": [
                {
                    "attributes": [
                        {
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Eye Color",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Birth Year",
                            "metadata": { "options": null },
                            "property_type": "Number"
                        }
                    ],
                    "color": "#3498DB",
                    "label": "Person"
                },
                {
                    "attributes": [
                        {
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Location Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Country",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#9B59B6",
                    "label": "Location"
                }
            ]
        });

        let expected = json!({
            "edges": [
                {
                    "properties": [
                        {
                            "label": "Friend Of",
                            "metadata": { "options": null },
                            "property_type": "Boolean"
                        }
                    ],
                    "color": "#FF5733",
                    "label": "Friend Of"
                },
                {
                    "properties": [
                        {
                            "label": "Born In",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#33FF57",
                    "label": "Born In"
                }
            ],
            "nodes": [
                {
                    "properties": [
                        {
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Eye Color",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Birth Year",
                            "metadata": { "options": null },
                            "property_type": "Number"
                        }
                    ],
                    "color": "#3498DB",
                    "label": "Person"
                },
                {
                    "properties": [
                        {
                            "label": "Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Location Name",
                            "metadata": { "options": null },
                            "property_type": "String"
                        },
                        {
                            "label": "Country",
                            "metadata": { "options": null },
                            "property_type": "String"
                        }
                    ],
                    "color": "#9B59B6",
                    "label": "Location"
                }
            ]
        });

        assert_eq!(rename_attributes_to_properties(input), expected);
    }

    #[test]
    fn test_rename_attributes_to_properties_preserves_attributes() {
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
