use crate::{
    infrastructure::clients::{Message, OpenRouterClient},
    presentation::errors::{AppError, OpenRouterClientError},
};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateGraphDataDto, CreateNodeDataDto, GraphSchemaDto,
    MetadataOptionString, NodeDataIdDto, PropertiesDataDto, PropertyTypeDto,
};
use serde_json::json;
use std::collections::HashMap;

pub struct DataService {
    openrouter_client: OpenRouterClient,
}

impl DataService {
    pub fn new(openrouter_client: OpenRouterClient) -> Self {
        Self { openrouter_client }
    }

    #[tracing::instrument(
        level = "trace",
        name = "data_service.generate_data",
        skip(self, schema, file_content)
    )]
    pub async fn generate_data(
        &self,
        schema: GraphSchemaDto,
        file_content: Vec<u8>,
    ) -> Result<CreateGraphDataDto, AppError> {
        let parsed_content =
            std::str::from_utf8(&file_content).map_err(|err| AppError::FileParsing {
                message: "Failed to parse file".to_string(),
                source: err,
            })?;

        let data_schema = schema_to_data_json_schema(&schema);
        let system_prompt = build_system_prompt();
        let user_prompt = build_user_prompt(parsed_content, &schema);

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
                .chat(messages.clone(), Some(data_schema.clone()))
                .await?;
            let raw_json = result.raw_content.clone();

            let parse_result =
                json_to_graph_data(result.value, &schema).and_then(|graph_data| {
                    match graph_data.validate_against_schema(&schema) {
                        Ok(()) => Ok(graph_data),
                        Err(errors) => {
                            let items: Vec<String> = errors
                                .iter()
                                .enumerate()
                                .map(|(i, e)| format!("  {}. {}", i + 1, e))
                                .collect();
                            Err(format!(
                                "The generated data does not conform to the schema ({} error(s)):\n{}",
                                errors.len(),
                                items.join("\n")
                            ))
                        }
                    }
                });

            match parse_result {
                Ok(graph_data) => break Ok(graph_data),
                Err(err) if attempt < MAX_ATTEMPTS => {
                    tracing::warn!(attempt, error = %err, "Data generation failed, retrying");
                    messages.push(Message {
                        role: "assistant".to_string(),
                        content: raw_json,
                    });
                    messages.push(Message {
                        role: "user".to_string(),
                        content: build_correction_prompt(&err),
                    });
                }
                Err(err) => {
                    break Err(AppError::OpenRouterClient(
                        OpenRouterClientError::DataGeneration { message: err },
                    ))
                }
            }
        }
    }
}

fn build_system_prompt() -> String {
    r##"You are a graph data extractor assistant. Extract entities and relationships from a document and return compact JSON.

Output format: for each node and edge type, output a "columns" array (the exact column names) and a "rows" array of arrays, where each row contains values in the same order as "columns".

Rules:
- Output ONLY the types listed in the schema
- For nodes: the first column is always "id" — assign unique string IDs like "n1", "n2", ...
- For edges: the first two columns are always "from" and "to" — they must reference node IDs you defined in the "id" column
- No self-loops: "from" and "to" must be different node IDs
- Do not invent data not present or strongly implied by the document
- If a value is unknown or absent: use "" for strings, 0 for numbers, false for booleans
- If a type has no instances, return an empty "rows" array
- Use the EXACT column names specified in the schema description"##
        .to_string()
}

fn build_user_prompt(document: &str, schema: &GraphSchemaDto) -> String {
    let mut lines = vec![
        "Schema column definitions (use these EXACT column names and order):".to_string(),
        String::new(),
    ];

    for node in &schema.nodes {
        let type_key = node.key.to_string();
        let label = node.label.to_string();
        let mut col_specs = vec![r#""id" (unique string node ID)"#.to_string()];
        let mut sorted_props: Vec<_> = node.properties.iter().collect();
        sorted_props.sort_by_key(|p| p.key.to_string());
        for prop in &sorted_props {
            let type_str = prop_type_display(&prop.property_type, &prop.metadata.options);
            col_specs.push(format!(r#""{}" ({}, label: {})"#, prop.key, type_str, prop.label));
        }
        lines.push(format!("Node \"{}\" ({}): {}", type_key, label, col_specs.join(" | ")));
    }

    lines.push(String::new());

    for edge in &schema.edges {
        let type_key = edge.key.to_string();
        let label = edge.label.to_string();
        let mut col_specs = vec![
            r#""from" (node ID)"#.to_string(),
            r#""to" (node ID)"#.to_string(),
        ];
        let mut sorted_props: Vec<_> = edge.properties.iter().collect();
        sorted_props.sort_by_key(|p| p.key.to_string());
        for prop in &sorted_props {
            let type_str = prop_type_display(&prop.property_type, &prop.metadata.options);
            col_specs.push(format!(r#""{}" ({}, label: {})"#, prop.key, type_str, prop.label));
        }
        lines.push(format!("Edge \"{}\" ({}): {}", type_key, label, col_specs.join(" | ")));
    }

    lines.push(String::new());
    lines.push("Document:".to_string());
    lines.push(String::new());
    lines.push(document.to_string());

    lines.join("\n")
}

fn build_correction_prompt(errors: &str) -> String {
    format!(
        r##"Your previous response contained errors that must be fixed:

{}

Return the corrected JSON. Make sure every "from" and "to" in edges references a valid node ID from the "id" column in the nodes section."##,
        errors
    )
}

fn prop_type_display(prop_type: &PropertyTypeDto, options: &Option<Vec<MetadataOptionString>>) -> String {
    match prop_type {
        PropertyTypeDto::Number => "Number".to_string(),
        PropertyTypeDto::Boolean => "Boolean".to_string(),
        PropertyTypeDto::String => "String".to_string(),
        PropertyTypeDto::Select => match options {
            Some(opts) => format!("Select: {}", opts.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(", ")),
            None => "Select".to_string(),
        },
    }
}

fn schema_to_data_json_schema(schema: &GraphSchemaDto) -> serde_json::Value {
    let type_schema = json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["columns", "rows"],
        "properties": {
            "columns": {
                "type": "array",
                "items": {"type": "string"}
            },
            "rows": {
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {
                        "anyOf": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"}
                        ]
                    }
                }
            }
        }
    });

    let mut node_type_schemas = serde_json::Map::new();
    let mut node_required: Vec<serde_json::Value> = Vec::new();
    for node in &schema.nodes {
        let type_key = node.key.to_string();
        node_required.push(json!(type_key));
        node_type_schemas.insert(type_key, type_schema.clone());
    }

    let mut edge_type_schemas = serde_json::Map::new();
    let mut edge_required: Vec<serde_json::Value> = Vec::new();
    for edge in &schema.edges {
        let type_key = edge.key.to_string();
        edge_required.push(json!(type_key));
        edge_type_schemas.insert(type_key, type_schema.clone());
    }

    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["nodes", "edges"],
        "properties": {
            "nodes": {
                "type": "object",
                "additionalProperties": false,
                "required": node_required,
                "properties": node_type_schemas
            },
            "edges": {
                "type": "object",
                "additionalProperties": false,
                "required": edge_required,
                "properties": edge_type_schemas
            }
        }
    })
}

fn json_to_graph_data(
    json: serde_json::Value,
    schema: &GraphSchemaDto,
) -> Result<CreateGraphDataDto, String> {
    // tracing::debug!(json = %json);

    let nodes_obj = json
        .get("nodes")
        .and_then(|v| v.as_object())
        .ok_or("Missing 'nodes' object in response")?;

    let edges_obj = json
        .get("edges")
        .and_then(|v| v.as_object())
        .ok_or("Missing 'edges' object in response")?;

    let mut nodes: Vec<CreateNodeDataDto> = Vec::new();
    let mut edges: Vec<CreateEdgeDataDto> = Vec::new();
    let mut node_id_map: HashMap<String, NodeDataIdDto> = HashMap::new();

    // Helper: parse the "columns" array from a type value
    let parse_columns = |type_val: &serde_json::Value, label: &str| -> Result<Vec<String>, String> {
        type_val
            .get("columns")
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("Type '{}' missing 'columns' array", label))?
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or_else(|| format!("Column name in '{}' is not a string", label))
                    .map(|s| s.to_string())
            })
            .collect()
    };

    // Process nodes in schema order so edge ID lookups are stable
    for node_schema in &schema.nodes {
        let type_key = node_schema.key.to_string();
        let type_val = match nodes_obj.get(&type_key) {
            Some(v) => v,
            None => continue,
        };

        let columns = parse_columns(type_val, &type_key)?;
        let id_idx = columns
            .iter()
            .position(|c| c == "id")
            .ok_or_else(|| format!("Node type '{}' columns missing 'id'", type_key))?;
        let rows = type_val
            .get("rows")
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("Node type '{}' missing 'rows' array", type_key))?;

        for row in rows {
            let row_arr = row
                .as_array()
                .ok_or_else(|| format!("Row in node type '{}' is not an array", type_key))?;

            let ai_id = row_arr
                .get(id_idx)
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("Node of type '{}' missing 'id' at column index {}", type_key, id_idx))?
                .to_string();

            let node_id = NodeDataIdDto::new();
            node_id_map.insert(ai_id, node_id);

            let mut values = HashMap::new();
            for (idx, col) in columns.iter().enumerate() {
                if col == "id" {
                    continue;
                }
                if let Some(v) = row_arr.get(idx) {
                    values.insert(col.clone(), v.clone());
                }
            }

            nodes.push(CreateNodeDataDto {
                node_data_id: node_id,
                key: type_key.clone().into(),
                properties: PropertiesDataDto { values },
            });
        }
    }

    // Process edges
    for edge_schema in &schema.edges {
        let type_key = edge_schema.key.to_string();
        let type_val = match edges_obj.get(&type_key) {
            Some(v) => v,
            None => continue,
        };

        let columns = parse_columns(type_val, &type_key)?;
        let from_idx = columns
            .iter()
            .position(|c| c == "from")
            .ok_or_else(|| format!("Edge type '{}' columns missing 'from'", type_key))?;
        let to_idx = columns
            .iter()
            .position(|c| c == "to")
            .ok_or_else(|| format!("Edge type '{}' columns missing 'to'", type_key))?;
        let rows = type_val
            .get("rows")
            .and_then(|v| v.as_array())
            .ok_or_else(|| format!("Edge type '{}' missing 'rows' array", type_key))?;

        for row in rows {
            let row_arr = row
                .as_array()
                .ok_or_else(|| format!("Row in edge type '{}' is not an array", type_key))?;

            let from_ai_id = row_arr
                .get(from_idx)
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("Edge of type '{}' missing 'from' at column index {}", type_key, from_idx))?;
            let to_ai_id = row_arr
                .get(to_idx)
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("Edge of type '{}' missing 'to' at column index {}", type_key, to_idx))?;

            let valid_ids: Vec<&str> = node_id_map.keys().map(|s| s.as_str()).collect();

            let from_id = *node_id_map.get(from_ai_id).ok_or_else(|| {
                format!(
                    "Edge type '{}': 'from' references unknown node ID '{}'. Valid IDs: [{}]",
                    type_key,
                    from_ai_id,
                    valid_ids.join(", ")
                )
            })?;
            let to_id = *node_id_map.get(to_ai_id).ok_or_else(|| {
                format!(
                    "Edge type '{}': 'to' references unknown node ID '{}'. Valid IDs: [{}]",
                    type_key,
                    to_ai_id,
                    valid_ids.join(", ")
                )
            })?;

            let mut values = HashMap::new();
            for (idx, col) in columns.iter().enumerate() {
                if col == "from" || col == "to" {
                    continue;
                }
                if let Some(v) = row_arr.get(idx) {
                    values.insert(col.clone(), v.clone());
                }
            }

            edges.push(CreateEdgeDataDto {
                key: type_key.clone().into(),
                from_node_data_id: from_id,
                to_node_data_id: to_id,
                properties: PropertiesDataDto { values },
            });
        }
    }

    Ok(CreateGraphDataDto { nodes, edges })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Minimal GraphSchemaDto fixture shared across tests.
    // Nodes (in schema order): ESVhRs9k (Object), dudFcexv (Person)
    // Edges (in schema order): Oq9afK3f (Records)
    fn make_schema() -> GraphSchemaDto {
        serde_json::from_value(json!({
            "nodes": [
                {
                    "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Object",
                    "key": "ESVhRs9k",
                    "color": "#C70039",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5e0b2ada8a3",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                            "edge_schema_id": null,
                            "label": "Name",
                            "key": "po86zGND",
                            "property_type": "String",
                            "metadata": { "options": null },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f5f426c41dd0",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f5025ebb9e27",
                            "edge_schema_id": null,
                            "label": "Type",
                            "key": "JboctBKk",
                            "property_type": "Select",
                            "metadata": { "options": ["Lighthouse", "Logbook"] },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                },
                {
                    "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Person",
                    "key": "dudFcexv",
                    "color": "#28B463",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f579dd147c29",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Name",
                            "key": "B1ixXXAx",
                            "property_type": "String",
                            "metadata": { "options": null },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        },
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f591c4ec345c",
                            "node_schema_id": "019c9503-34c7-71a0-aee8-f4ea514266e0",
                            "edge_schema_id": null,
                            "label": "Years of Experience",
                            "key": "h2GIMoa9",
                            "property_type": "Number",
                            "metadata": { "options": null },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                }
            ],
            "edges": [
                {
                    "edge_schema_id": "019c9503-34c7-71a0-aee8-f51dcfde6050",
                    "graph_id": "019c9503-0dda-7553-b3c2-dc516f490a1a",
                    "label": "Records",
                    "key": "Oq9afK3f",
                    "color": "#FF5733",
                    "created_at": "2026-02-25T13:35:41.255685Z",
                    "updated_at": "2026-02-25T13:35:41.255685Z",
                    "properties": [
                        {
                            "property_schema_id": "019c9503-34ca-7610-b62f-f6178554dfdd",
                            "node_schema_id": null,
                            "edge_schema_id": "019c9503-34c7-71a0-aee8-f51dcfde6050",
                            "label": "Frequency",
                            "key": "QVL3enQS",
                            "property_type": "Select",
                            "metadata": { "options": ["Daily", "Weekly", "Monthly"] },
                            "created_at": "2026-02-25T13:35:41.255685Z",
                            "updated_at": "2026-02-25T13:35:41.255685Z"
                        }
                    ]
                }
            ]
        }))
        .unwrap()
    }

    #[test]
    fn test_schema_to_data_json_schema() {
        let schema = make_schema();
        let result = schema_to_data_json_schema(&schema);

        let type_schema = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["columns", "rows"],
            "properties": {
                "columns": {
                    "type": "array",
                    "items": {"type": "string"}
                },
                "rows": {
                    "type": "array",
                    "items": {
                        "type": "array",
                        "items": {
                            "anyOf": [
                                {"type": "string"},
                                {"type": "number"},
                                {"type": "boolean"}
                            ]
                        }
                    }
                }
            }
        });

        let expected = json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["nodes", "edges"],
            "properties": {
                "nodes": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["ESVhRs9k", "dudFcexv"],
                    "properties": {
                        "ESVhRs9k": type_schema.clone(),
                        "dudFcexv": type_schema.clone()
                    }
                },
                "edges": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["Oq9afK3f"],
                    "properties": {
                        "Oq9afK3f": type_schema
                    }
                }
            }
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_to_graph_data() {
        let schema = make_schema();

        let input = json!({
            "nodes": {
                "ESVhRs9k": {
                    "columns": ["id", "JboctBKk", "po86zGND"],
                    "rows": [["n1", "Lighthouse", "Bell Rock"]]
                },
                "dudFcexv": {
                    "columns": ["id", "B1ixXXAx", "h2GIMoa9"],
                    "rows": [
                        ["n2", "Alice", 10],
                        ["n3", "Bob", 5]
                    ]
                }
            },
            "edges": {
                "Oq9afK3f": {
                    "columns": ["from", "to", "QVL3enQS"],
                    "rows": [
                        ["n2", "n1", "Daily"],
                        ["n3", "n1", "Weekly"]
                    ]
                }
            }
        });

        let graph_data = json_to_graph_data(input, &schema).unwrap();

        // Three nodes, two edges
        assert_eq!(graph_data.nodes.len(), 3);
        assert_eq!(graph_data.edges.len(), 2);

        // All node IDs must be unique
        let id_set: std::collections::HashSet<_> =
            graph_data.nodes.iter().map(|n| n.node_data_id).collect();
        assert_eq!(id_set.len(), 3);

        // Verify node types and properties (order: schema order → ESVhRs9k first, then dudFcexv)
        let actual_nodes: Vec<(String, serde_json::Value)> = graph_data
            .nodes
            .iter()
            .map(|n| (n.key.to_string(), serde_json::to_value(&n.properties).unwrap()))
            .collect();
        assert_eq!(actual_nodes[0], ("ESVhRs9k".to_string(), json!({"JboctBKk": "Lighthouse", "po86zGND": "Bell Rock"})));
        assert_eq!(actual_nodes[1], ("dudFcexv".to_string(), json!({"B1ixXXAx": "Alice", "h2GIMoa9": 10})));
        assert_eq!(actual_nodes[2], ("dudFcexv".to_string(), json!({"B1ixXXAx": "Bob",   "h2GIMoa9": 5})));

        // Edges must reference the correct node UUIDs
        let id_to_idx: HashMap<NodeDataIdDto, usize> = graph_data
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (n.node_data_id, i))
            .collect();

        let actual_edges: Vec<(String, usize, usize, serde_json::Value)> = graph_data
            .edges
            .iter()
            .map(|e| {
                (
                    e.key.to_string(),
                    id_to_idx[&e.from_node_data_id],
                    id_to_idx[&e.to_node_data_id],
                    serde_json::to_value(&e.properties).unwrap(),
                )
            })
            .collect();

        assert_eq!(
            actual_edges,
            vec![
                ("Oq9afK3f".to_string(), 1, 0, json!({"QVL3enQS": "Daily"})),
                ("Oq9afK3f".to_string(), 2, 0, json!({"QVL3enQS": "Weekly"})),
            ]
        );
    }

    #[test]
    fn test_json_to_graph_data_unknown_node_id_error() {
        let schema = make_schema();

        let input = json!({
            "nodes": {
                "ESVhRs9k": {
                    "columns": ["id", "JboctBKk", "po86zGND"],
                    "rows": [["n1", "Lighthouse", "Bell Rock"]]
                },
                "dudFcexv": {
                    "columns": ["id", "B1ixXXAx", "h2GIMoa9"],
                    "rows": []
                }
            },
            "edges": {
                "Oq9afK3f": {
                    "columns": ["from", "to", "QVL3enQS"],
                    "rows": [["n99", "n1", "Daily"]]
                }
            }
        });

        let err = json_to_graph_data(input, &schema).unwrap_err();
        assert!(err.contains("n99"), "error should mention the bad ID: {}", err);
        assert!(err.contains("Oq9afK3f"), "error should mention the edge type: {}", err);
    }
}
