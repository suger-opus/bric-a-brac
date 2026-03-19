use crate::infrastructure::clients::{EmbeddingClient, KnowledgeClient, MetadataClient};
use bric_a_brac_protos::common::{
    GraphSchemaProto, InsertEdgeDataProto, InsertNodeDataProto, PropertyValueProto,
    UpdateNodeDataProto,
};
use serde_json::Value;
use std::collections::HashMap;

const ENTITY_RESOLUTION_LIMIT: i32 = 5;
const SIMILARITY_THRESHOLD: f32 = 0.3;

#[derive(Clone)]
pub struct ToolExecutor {
    knowledge_client: KnowledgeClient,
    metadata_client: MetadataClient,
    embedding_client: EmbeddingClient,
}

pub struct ToolResult {
    pub content: String,
    /// Whether the executor created/modified a schema (triggers schema refresh)
    pub schema_changed: bool,
    /// Whether the "done" tool was called
    pub is_done: bool,
}

impl ToolExecutor {
    pub const fn new(
        knowledge_client: KnowledgeClient,
        metadata_client: MetadataClient,
        embedding_client: EmbeddingClient,
    ) -> Self {
        Self {
            knowledge_client,
            metadata_client,
            embedding_client,
        }
    }

    pub async fn execute(
        &self,
        tool_name: &str,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
    ) -> ToolResult {
        let result = match tool_name {
            "search_nodes" => self.exec_search_nodes(arguments, graph_id).await,
            "get_node" => self.exec_get_node(arguments, graph_id).await,
            "get_neighbors" => self.exec_get_neighbors(arguments, graph_id).await,
            "find_paths" => self.exec_find_paths(arguments, graph_id).await,
            "create_schema" => {
                return self.exec_create_schema(arguments, graph_id).await;
            }
            "create_edge_schema" => {
                return self.exec_create_edge_schema(arguments, graph_id).await;
            }
            "create_node" => {
                self.exec_create_node(arguments, graph_id, session_id, schema)
                    .await
            }
            "create_edge" => {
                self.exec_create_edge(arguments, graph_id, session_id, schema)
                    .await
            }
            "update_node" => self.exec_update_node(arguments, graph_id).await,
            "done" => return self.exec_done(arguments),
            _ => Err(format!("Unknown tool: {tool_name}")),
        };

        ToolResult {
            content: match result {
                Ok(content) => content,
                Err(err) => format!("Error: {err}"),
            },
            schema_changed: false,
            is_done: false,
        }
    }

    async fn exec_search_nodes(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let query = get_str(&args, "query")?;
        let node_key = args.get("node_key").and_then(|v| v.as_str()).map(String::from);
        let limit = args
            .get("limit")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(10) as i32;

        let embedding = self
            .embedding_client
            .embed_one(query.to_owned())
            .await
            .map_err(|e| format!("Embedding failed: {e}"))?;

        let nodes = self
            .knowledge_client
            .search_nodes(graph_id, node_key, embedding, limit)
            .await
            .map_err(|e| format!("Search failed: {e}"))?;

        if nodes.is_empty() {
            return Ok("No matching nodes found.".to_owned());
        }

        let result: Vec<Value> = nodes
            .into_iter()
            .map(|n| {
                serde_json::json!({
                    "node_data_id": n.node_data_id,
                    "key": n.key,
                    "properties": proto_properties_to_json(&n.properties),
                    "distance": n.distance,
                })
            })
            .collect();

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_get_node(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")?;

        let node = self
            .knowledge_client
            .get_node(graph_id, node_data_id)
            .await
            .map_err(|e| format!("Get node failed: {e}"))?;

        let result = serde_json::json!({
            "node_data_id": node.node_data_id,
            "key": node.key,
            "properties": proto_properties_to_json(&node.properties),
        });

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_get_neighbors(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")?;
        let edge_key = args.get("edge_key").and_then(|v| v.as_str()).map(String::from);
        let depth = args.get("depth").and_then(serde_json::Value::as_i64).unwrap_or(1) as i32;

        let subgraph = self
            .knowledge_client
            .get_neighbors(graph_id, node_data_id, edge_key, depth)
            .await
            .map_err(|e| format!("Get neighbors failed: {e}"))?;

        let result = serde_json::json!({
            "nodes": subgraph.nodes.iter().map(|n| serde_json::json!({
                "node_data_id": n.node_data_id,
                "key": n.key,
                "properties": proto_properties_to_json(&n.properties),
            })).collect::<Vec<_>>(),
            "edges": subgraph.edges.iter().map(|e| serde_json::json!({
                "edge_data_id": e.edge_data_id,
                "key": e.key,
                "from_node_data_id": e.from_node_data_id,
                "to_node_data_id": e.to_node_data_id,
                "properties": proto_properties_to_json(&e.properties),
            })).collect::<Vec<_>>(),
        });

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_find_paths(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let from_id = get_str(&args, "from_node_data_id")?;
        let to_id = get_str(&args, "to_node_data_id")?;
        let max_depth = args
            .get("max_depth")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(5) as i32;

        let paths = self
            .knowledge_client
            .find_paths(graph_id, from_id, to_id, max_depth)
            .await
            .map_err(|e| format!("Find paths failed: {e}"))?;

        if paths.is_empty() {
            return Ok("No paths found between the specified nodes.".to_owned());
        }

        let result: Vec<Value> = paths
            .iter()
            .map(|p| {
                serde_json::json!({
                    "nodes": p.nodes.iter().map(|n| serde_json::json!({
                        "node_data_id": n.node_data_id,
                        "key": n.key,
                        "properties": proto_properties_to_json(&n.properties),
                    })).collect::<Vec<_>>(),
                    "edges": p.edges.iter().map(|e| serde_json::json!({
                        "edge_data_id": e.edge_data_id,
                        "key": e.key,
                        "from_node_data_id": e.from_node_data_id,
                        "to_node_data_id": e.to_node_data_id,
                        "properties": proto_properties_to_json(&e.properties),
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_create_schema(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> ToolResult {
        let result: Result<String, String> = async {
            let args: Value = parse_args(arguments)?;
            let label = get_str(&args, "label")?;
            let description = get_str(&args, "description")?;

            let schema = self
                .metadata_client
                .create_node_schema(graph_id, label, description)
                .await
                .map_err(|e| format!("Failed to create node schema: {e}"))?;

            Ok(format!(
                "Created node schema '{}' with key '{}'. Use this key when creating nodes of this type.",
                schema.label, schema.key
            ))
        }
        .await;

        let schema_changed = result.is_ok();
        ToolResult {
            content: match result {
                Ok(content) => content,
                Err(err) => format!("Error: {err}"),
            },
            schema_changed,
            is_done: false,
        }
    }

    async fn exec_create_edge_schema(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> ToolResult {
        let result: Result<String, String> = async {
            let args: Value = parse_args(arguments)?;
            let label = get_str(&args, "label")?;
            let description = get_str(&args, "description")?;

            let schema = self
                .metadata_client
                .create_edge_schema(graph_id, label, description)
                .await
                .map_err(|e| format!("Failed to create edge schema: {e}"))?;

            Ok(format!(
                "Created edge schema '{}' with key '{}'. Use this key when creating edges of this type.",
                schema.label, schema.key
            ))
        }
        .await;

        let schema_changed = result.is_ok();
        ToolResult {
            content: match result {
                Ok(content) => content,
                Err(err) => format!("Error: {err}"),
            },
            schema_changed,
            is_done: false,
        }
    }

    async fn exec_create_node(
        &self,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_key = get_str(&args, "node_key")?;
        let properties = get_properties(&args)?;

        // Validate schema exists
        validate_node_key(schema, node_key)?;

        // Generate embedding from properties
        let props_text = properties_to_text(&properties);
        let embedding = self
            .embedding_client
            .embed_one(props_text)
            .await
            .map_err(|e| format!("Embedding failed: {e}"))?;

        // Entity resolution: search for similar existing nodes
        let similar_nodes = self
            .knowledge_client
            .search_nodes(
                graph_id,
                Some(node_key.to_owned()),
                embedding.clone(),
                ENTITY_RESOLUTION_LIMIT,
            )
            .await
            .map_err(|e| format!("Entity resolution search failed: {e}"))?;

        let close_matches: Vec<_> = similar_nodes
            .into_iter()
            .filter(|n| n.distance < SIMILARITY_THRESHOLD)
            .collect();

        // Create the node
        let node_data_id = uuid::Uuid::now_v7().to_string();
        let proto_properties = json_properties_to_proto(&properties);

        let node = self
            .knowledge_client
            .insert_node(
                graph_id,
                InsertNodeDataProto {
                    node_data_id: node_data_id.clone(),
                    key: node_key.to_owned(),
                    properties: proto_properties,
                    embedding,
                    session_id: Some(session_id.to_owned()),
                },
            )
            .await
            .map_err(|e| format!("Failed to create node: {e}"))?;

        let mut result = serde_json::json!({
            "node_data_id": node.node_data_id,
            "key": node.key,
            "properties": proto_properties_to_json(&node.properties),
        });

        // If similar nodes found, add warnings for LLM to decide
        if !close_matches.is_empty() {
            let mut warnings = Vec::new();
            for candidate in &close_matches {
                // Fetch neighbor context for each candidate
                let neighbors = self
                    .knowledge_client
                    .get_neighbors(graph_id, &candidate.node_data_id, None, 1)
                    .await
                    .ok();

                let neighbor_summary = neighbors.map(|s| {
                    serde_json::json!({
                        "node_count": s.nodes.len(),
                        "edge_count": s.edges.len(),
                    })
                });

                warnings.push(serde_json::json!({
                    "similar_node_data_id": candidate.node_data_id,
                    "key": candidate.key,
                    "properties": proto_properties_to_json(&candidate.properties),
                    "distance": candidate.distance,
                    "neighbors": neighbor_summary,
                }));
            }

            result["WARNING"] = serde_json::json!(
                "Similar nodes already exist. Review them and consider using update_node to merge \
                 information into an existing node instead of keeping duplicates."
            );
            result["similar_nodes"] = serde_json::json!(warnings);
        }

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_create_edge(
        &self,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_key = get_str(&args, "edge_key")?;
        let from_id = get_str(&args, "from_node_data_id")?;
        let to_id = get_str(&args, "to_node_data_id")?;
        let properties = args
            .get("properties")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        // Validate edge schema exists
        validate_edge_key(schema, edge_key)?;

        let proto_properties = json_properties_to_proto(&properties);

        self.knowledge_client
            .insert_edge(
                graph_id,
                InsertEdgeDataProto {
                    from_node_data_id: from_id.to_owned(),
                    to_node_data_id: to_id.to_owned(),
                    key: edge_key.to_owned(),
                    properties: proto_properties,
                    session_id: Some(session_id.to_owned()),
                },
            )
            .await
            .map_err(|e| format!("Failed to create edge: {e}"))?;

        Ok(format!(
            "Created edge '{edge_key}' from {from_id} to {to_id}."
        ))
    }

    async fn exec_update_node(
        &self,
        arguments: &str,
        graph_id: &str,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")?;
        let properties = get_properties(&args)?;

        // Re-embed from updated properties
        let props_text = properties_to_text(&properties);
        let embedding = self
            .embedding_client
            .embed_one(props_text)
            .await
            .map_err(|e| format!("Embedding failed: {e}"))?;

        let proto_properties = json_properties_to_proto(&properties);

        let node = self
            .knowledge_client
            .update_node(
                graph_id,
                UpdateNodeDataProto {
                    node_data_id: node_data_id.to_owned(),
                    properties: proto_properties,
                    embedding,
                },
            )
            .await
            .map_err(|e| format!("Failed to update node: {e}"))?;

        let result = serde_json::json!({
            "node_data_id": node.node_data_id,
            "key": node.key,
            "properties": proto_properties_to_json(&node.properties),
        });

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    fn exec_done(&self, arguments: &str) -> ToolResult {
        let summary = parse_args(arguments)
            .ok()
            .and_then(|args| args.get("summary").and_then(|v| v.as_str()).map(String::from))
            .unwrap_or_else(|| "Task completed.".to_owned());

        ToolResult {
            content: summary,
            schema_changed: false,
            is_done: true,
        }
    }
}

// --- Helpers ---

fn parse_args(arguments: &str) -> Result<Value, String> {
    serde_json::from_str(arguments).map_err(|e| format!("Invalid tool arguments: {e}"))
}

fn get_str<'a>(args: &'a Value, field: &str) -> Result<&'a str, String> {
    args.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required field: {field}"))
}

fn get_properties(args: &Value) -> Result<HashMap<String, Value>, String> {
    let obj = args
        .get("properties")
        .and_then(|v| v.as_object())
        .ok_or("Missing required field: properties")?;

    Ok(obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

fn validate_node_key(schema: &GraphSchemaProto, node_key: &str) -> Result<(), String> {
    if schema.nodes.iter().any(|n| n.key == node_key) {
        return Ok(());
    }
    let valid: Vec<String> = schema
        .nodes
        .iter()
        .map(|n| format!("{} ({})", n.key, n.label))
        .collect();
    Err(format!(
        "Unknown node schema key '{node_key}'. Valid schemas: {}",
        valid.join(", ")
    ))
}

fn validate_edge_key(schema: &GraphSchemaProto, edge_key: &str) -> Result<(), String> {
    if schema.edges.iter().any(|e| e.key == edge_key) {
        return Ok(());
    }
    let valid: Vec<String> = schema
        .edges
        .iter()
        .map(|e| format!("{} ({})", e.key, e.label))
        .collect();
    Err(format!(
        "Unknown edge schema key '{edge_key}'. Valid schemas: {}",
        valid.join(", ")
    ))
}

fn properties_to_text(properties: &HashMap<String, Value>) -> String {
    properties
        .iter()
        .map(|(k, v)| {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            format!("{k}: {val}")
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn proto_properties_to_json(
    properties: &HashMap<String, PropertyValueProto>,
) -> serde_json::Value {
    let map: serde_json::Map<String, Value> = properties
        .iter()
        .map(|(k, v)| {
            let val = match &v.value {
                Some(bric_a_brac_protos::common::property_value_proto::Value::StringValue(s)) => {
                    Value::String(s.clone())
                }
                Some(bric_a_brac_protos::common::property_value_proto::Value::NumberValue(n)) => {
                    serde_json::json!(*n)
                }
                Some(bric_a_brac_protos::common::property_value_proto::Value::BoolValue(b)) => {
                    Value::Bool(*b)
                }
                None => Value::Null,
            };
            (k.clone(), val)
        })
        .collect();
    Value::Object(map)
}

fn json_properties_to_proto(
    properties: &HashMap<String, Value>,
) -> HashMap<String, PropertyValueProto> {
    properties
        .iter()
        .filter_map(|(k, v)| {
            let value = match v {
                Value::String(s) => Some(
                    bric_a_brac_protos::common::property_value_proto::Value::StringValue(
                        s.clone(),
                    ),
                ),
                Value::Number(n) => n.as_f64().map(
                    bric_a_brac_protos::common::property_value_proto::Value::NumberValue,
                ),
                Value::Bool(b) => Some(
                    bric_a_brac_protos::common::property_value_proto::Value::BoolValue(*b),
                ),
                _ => None,
            };
            value.map(|v| (k.clone(), PropertyValueProto { value: Some(v) }))
        })
        .collect()
}
