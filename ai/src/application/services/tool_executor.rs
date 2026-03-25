use crate::infrastructure::clients::{EmbeddingClient, KnowledgeClient, MetadataClient};
use bric_a_brac_protos::common::{
    GraphSchemaProto, InsertEdgeDataProto, InsertNodeDataProto, PropertyValueProto,
    UpdateEdgeDataProto, UpdateNodeDataProto,
};
use serde_json::Value;
use std::collections::HashMap;

const ENTITY_RESOLUTION_LIMIT: i32 = 5;
const SIMILARITY_THRESHOLD: f32 = 0.3;

#[derive(Clone)]
#[allow(clippy::struct_field_names)]
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

const WRITE_TOOLS: &[&str] = &[
    "create_schema",
    "create_edge_schema",
    "create_node",
    "create_edge",
    "create_nodes",
    "create_edges",
    "update_node",
    "update_edge",
    "delete_node",
    "delete_edge",
];

fn is_write_role(role: &str) -> bool {
    matches!(role, "Owner" | "Admin" | "Editor")
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

    #[tracing::instrument(
        level = "debug",
        name = "tool.execute",
        skip(self, arguments, schema),
        fields(%tool_name, %graph_id, %session_id)
    )]
    pub async fn execute(
        &self,
        tool_name: &str,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
        user_role: &str,
    ) -> ToolResult {
        if WRITE_TOOLS.contains(&tool_name) && !is_write_role(user_role) {
            return ToolResult {
                content: format!(
                    "Permission denied: role '{user_role}' cannot use tool '{tool_name}'. \
                     Write access requires Owner, Admin, or Editor role."
                ),
                schema_changed: false,
                is_done: false,
            };
        }

        let result = match tool_name {
            "search_nodes" => self.exec_search_nodes(arguments, graph_id).await,
            "get_node" => self.exec_get_node(arguments, graph_id).await,
            "get_neighbors" => self.exec_get_neighbors(arguments, graph_id).await,
            "find_paths" => self.exec_find_paths(arguments, graph_id).await,
            "read_document" => self.exec_read_document(arguments).await,
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
            "create_nodes" => {
                self.exec_create_nodes(arguments, graph_id, session_id, schema)
                    .await
            }
            "create_edges" => {
                self.exec_create_edges(arguments, graph_id, session_id, schema)
                    .await
            }
            "update_node" => self.exec_update_node(arguments, graph_id).await,
            "update_edge" => self.exec_update_edge(arguments, graph_id).await,
            "delete_node" => self.exec_delete_node(arguments, graph_id).await,
            "delete_edge" => self.exec_delete_edge(arguments, graph_id).await,
            "done" => return self.exec_done(arguments),
            _ => Err(format!("Unknown tool: {tool_name}")),
        };

        ToolResult {
            content: match result {
                Ok(content) => content,
                Err(err) => {
                    tracing::warn!(tool_name, error = %err, "Tool execution failed");
                    format!("Error: {err}")
                }
            },
            schema_changed: false,
            is_done: false,
        }
    }

    async fn exec_search_nodes(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let query = get_str(&args, "query")?;
        let node_key = args
            .get("node_key")
            .and_then(|v| v.as_str())
            .map(String::from);
        let limit = i32::try_from(
            args.get("limit")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(10),
        )
        .unwrap_or(10);

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

    async fn exec_get_neighbors(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")?;
        let edge_key = args
            .get("edge_key")
            .and_then(|v| v.as_str())
            .map(String::from);
        let depth = i32::try_from(
            args.get("depth")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(1),
        )
        .unwrap_or(1);

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

    async fn exec_find_paths(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let from_id = get_str(&args, "from_node_data_id")?;
        let to_id = get_str(&args, "to_node_data_id")?;
        let max_depth = i32::try_from(
            args.get("max_depth")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(5),
        )
        .unwrap_or(5);

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

    async fn exec_read_document(&self, arguments: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let document_id = get_str(&args, "document_id")?;

        let doc = self
            .metadata_client
            .get_session_document(document_id)
            .await
            .map_err(|e| format!("Failed to get document: {e}"))?;

        let result = serde_json::json!({
            "document_id": doc.document_id,
            "filename": doc.filename,
            "content": doc.content,
        });

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_create_schema(&self, arguments: &str, graph_id: &str) -> ToolResult {
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

    async fn exec_create_edge_schema(&self, arguments: &str, graph_id: &str) -> ToolResult {
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
        let force = args
            .get("force")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        // Validate schema exists
        validate_node_key(schema, node_key)?;

        // Generate embedding from properties
        let props_text = properties_to_text(&properties);
        let embedding = self
            .embedding_client
            .embed_one(props_text)
            .await
            .map_err(|e| format!("Embedding failed: {e}"))?;

        // Entity resolution: search for similar existing nodes (unless forced)
        if !force {
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

            if !close_matches.is_empty() {
                let mut candidates = Vec::new();
                for candidate in &close_matches {
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

                    candidates.push(serde_json::json!({
                        "node_data_id": candidate.node_data_id,
                        "key": candidate.key,
                        "properties": proto_properties_to_json(&candidate.properties),
                        "distance": candidate.distance,
                        "neighbors": neighbor_summary,
                    }));
                }

                let result = serde_json::json!({
                    "created": false,
                    "reason": "Similar nodes already exist. Use update_node to merge information \
                        into an existing node, or call create_node again with force=true to \
                        create anyway.",
                    "similar_nodes": candidates,
                });

                return serde_json::to_string_pretty(&result)
                    .map_err(|e| format!("Serialization failed: {e}"));
            }
        }

        // No close matches (or force=true) — create the node
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

        let result = serde_json::json!({
            "created": true,
            "node_data_id": node.node_data_id,
            "key": node.key,
            "properties": proto_properties_to_json(&node.properties),
        });

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

    /// Batch create up to 50 nodes. Embeddings are generated in a single API call.
    /// Entity resolution still runs per-node (N calls to knowledge service).
    // NOTE: This generates O(N) calls to the knowledge service for entity resolution
    // (search_nodes + get_neighbors per node). A future optimisation would be to add
    // a batch entity-resolution endpoint to the knowledge service.
    async fn exec_create_nodes(
        &self,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let nodes_arr = args
            .get("nodes")
            .and_then(|v| v.as_array())
            .ok_or("Missing required field: nodes")?;

        if nodes_arr.len() > 50 {
            return Err("Maximum 50 nodes per batch".to_owned());
        }
        if nodes_arr.is_empty() {
            return Err("nodes array is empty".to_owned());
        }

        // Parse all entries and validate schemas upfront
        let mut entries: Vec<(&str, HashMap<String, Value>, bool)> =
            Vec::with_capacity(nodes_arr.len());
        for (i, item) in nodes_arr.iter().enumerate() {
            let node_key = item
                .get("node_key")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("nodes[{i}]: missing node_key"))?;
            validate_node_key(schema, node_key)?;
            let properties = get_properties(item)?;
            let force = item
                .get("force")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            entries.push((node_key, properties, force));
        }

        // Batch embed all nodes in a single API call
        let texts: Vec<String> = entries
            .iter()
            .map(|(_, props, _)| properties_to_text(props))
            .collect();
        let embeddings = self
            .embedding_client
            .embed(texts)
            .await
            .map_err(|e| format!("Batch embedding failed: {e}"))?;

        if embeddings.len() != entries.len() {
            return Err(format!(
                "Embedding count mismatch: got {} for {} nodes",
                embeddings.len(),
                entries.len()
            ));
        }

        // Process each node: entity resolution then create
        let mut results: Vec<Value> = Vec::with_capacity(entries.len());
        for (i, ((node_key, properties, force), embedding)) in
            entries.into_iter().zip(embeddings).enumerate()
        {
            // Entity resolution (unless forced)
            if !force {
                let similar_nodes = self
                    .knowledge_client
                    .search_nodes(
                        graph_id,
                        Some(node_key.to_owned()),
                        embedding.clone(),
                        ENTITY_RESOLUTION_LIMIT,
                    )
                    .await
                    .map_err(|e| format!("nodes[{i}]: entity resolution failed: {e}"))?;

                let close_matches: Vec<_> = similar_nodes
                    .into_iter()
                    .filter(|n| n.distance < SIMILARITY_THRESHOLD)
                    .collect();

                if !close_matches.is_empty() {
                    let candidates: Vec<Value> = close_matches
                        .iter()
                        .map(|c| {
                            serde_json::json!({
                                "node_data_id": c.node_data_id,
                                "key": c.key,
                                "properties": proto_properties_to_json(&c.properties),
                                "distance": c.distance,
                            })
                        })
                        .collect();

                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "reason": "Similar node already exists",
                        "similar_nodes": candidates,
                    }));
                    continue;
                }
            }

            // Create the node
            let node_data_id = uuid::Uuid::now_v7().to_string();
            let proto_properties = json_properties_to_proto(&properties);

            match self
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
            {
                Ok(node) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": true,
                        "node_data_id": node.node_data_id,
                        "key": node.key,
                        "properties": proto_properties_to_json(&node.properties),
                    }));
                }
                Err(e) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "error": format!("{e}"),
                    }));
                }
            }
        }

        serde_json::to_string_pretty(&results).map_err(|e| format!("Serialization failed: {e}"))
    }

    /// Batch create up to 50 edges. Each edge is merged if it already exists.
    // NOTE: This generates O(N) insert_edge calls to the knowledge service.
    // A future optimisation would be a batch insert endpoint.
    async fn exec_create_edges(
        &self,
        arguments: &str,
        graph_id: &str,
        session_id: &str,
        schema: &GraphSchemaProto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edges_arr = args
            .get("edges")
            .and_then(|v| v.as_array())
            .ok_or("Missing required field: edges")?;

        if edges_arr.len() > 50 {
            return Err("Maximum 50 edges per batch".to_owned());
        }
        if edges_arr.is_empty() {
            return Err("edges array is empty".to_owned());
        }

        let mut results: Vec<Value> = Vec::with_capacity(edges_arr.len());

        for (i, item) in edges_arr.iter().enumerate() {
            let edge_key = item
                .get("edge_key")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("edges[{i}]: missing edge_key"))?;
            let from_id = item
                .get("from_node_data_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("edges[{i}]: missing from_node_data_id"))?;
            let to_id = item
                .get("to_node_data_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("edges[{i}]: missing to_node_data_id"))?;

            validate_edge_key(schema, edge_key)?;

            let properties = item
                .get("properties")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect::<HashMap<_, _>>()
                })
                .unwrap_or_default();

            let proto_properties = json_properties_to_proto(&properties);

            match self
                .knowledge_client
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
            {
                Ok(()) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": true,
                        "edge_key": edge_key,
                        "from_node_data_id": from_id,
                        "to_node_data_id": to_id,
                    }));
                }
                Err(e) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "error": format!("{e}"),
                    }));
                }
            }
        }

        serde_json::to_string_pretty(&results).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_update_node(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
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

    async fn exec_update_edge(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_data_id = get_str(&args, "edge_data_id")?;
        let properties = get_properties(&args)?;

        let proto_properties = json_properties_to_proto(&properties);

        let edge = self
            .knowledge_client
            .update_edge(
                graph_id,
                UpdateEdgeDataProto {
                    edge_data_id: edge_data_id.to_owned(),
                    properties: proto_properties,
                },
            )
            .await
            .map_err(|e| format!("Failed to update edge: {e}"))?;

        let result = serde_json::json!({
            "edge_data_id": edge.edge_data_id,
            "key": edge.key,
            "from_node_data_id": edge.from_node_data_id,
            "to_node_data_id": edge.to_node_data_id,
            "properties": proto_properties_to_json(&edge.properties),
        });

        serde_json::to_string_pretty(&result).map_err(|e| format!("Serialization failed: {e}"))
    }

    async fn exec_delete_node(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")?;

        self.knowledge_client
            .delete_node(graph_id, node_data_id)
            .await
            .map_err(|e| format!("Failed to delete node: {e}"))?;

        Ok(format!("Deleted node {node_data_id} and all its edges."))
    }

    async fn exec_delete_edge(&self, arguments: &str, graph_id: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_data_id = get_str(&args, "edge_data_id")?;

        self.knowledge_client
            .delete_edge(graph_id, edge_data_id)
            .await
            .map_err(|e| format!("Failed to delete edge: {e}"))?;

        Ok(format!("Deleted edge {edge_data_id}."))
    }

    #[allow(clippy::unused_self)]
    fn exec_done(&self, arguments: &str) -> ToolResult {
        let summary = parse_args(arguments)
            .ok()
            .and_then(|args| {
                args.get("summary")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
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
    // Keys that are tool parameters, not user-facing properties
    const NON_PROPERTY_KEYS: &[&str] = &[
        "node_key",
        "edge_key",
        "node_data_id",
        "source_node_data_id",
        "target_node_data_id",
        "edge_data_id",
        "force",
    ];

    // Try nested "properties" key first (correct schema)
    if let Some(obj) = args.get("properties").and_then(|v| v.as_object()) {
        return Ok(obj
            .iter()
            .filter(|(k, _)| !NON_PROPERTY_KEYS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect());
    }

    // Fallback: LLM sometimes puts properties at top level instead of nesting
    let obj = args
        .as_object()
        .ok_or("Missing required field: properties")?;
    let props: HashMap<String, Value> = obj
        .iter()
        .filter(|(k, _)| !NON_PROPERTY_KEYS.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    if props.is_empty() {
        return Err("Missing required field: properties".to_owned());
    }

    Ok(props)
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

fn proto_properties_to_json(properties: &HashMap<String, PropertyValueProto>) -> serde_json::Value {
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
                    bric_a_brac_protos::common::property_value_proto::Value::StringValue(s.clone()),
                ),
                Value::Number(n) => n
                    .as_f64()
                    .map(bric_a_brac_protos::common::property_value_proto::Value::NumberValue),
                Value::Bool(b) => {
                    Some(bric_a_brac_protos::common::property_value_proto::Value::BoolValue(*b))
                }
                _ => None,
            };
            value.map(|v| (k.clone(), PropertyValueProto { value: Some(v) }))
        })
        .collect()
}
