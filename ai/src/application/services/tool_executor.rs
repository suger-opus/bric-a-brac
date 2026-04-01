use crate::infrastructure::{EmbeddingClient, KnowledgeClient, MetadataClient};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, DescriptionDto, EdgeDataIdDto, GraphIdDto,
    GraphSchemaDto, KeyDto, LabelDto, NodeDataIdDto, PropertiesDataDto, RoleDto,
    SessionDocumentIdDto, SessionIdDto, UpdateEdgeDataDto, UpdateNodeDataDto,
};
use serde_json::Value;

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

const fn is_write_role(role: RoleDto) -> bool {
    matches!(role, RoleDto::Owner | RoleDto::Admin | RoleDto::Editor)
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
        graph_id: GraphIdDto,
        session_id: SessionIdDto,
        schema: &GraphSchemaDto,
        user_role: RoleDto,
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
            "create_schema" => self.exec_create_schema(arguments, graph_id).await,
            "create_edge_schema" => self.exec_create_edge_schema(arguments, graph_id).await,
            "create_node" => self.exec_create_node(arguments, graph_id, schema).await,
            "create_edge" => self.exec_create_edge(arguments, graph_id, schema).await,
            "create_nodes" => self.exec_create_nodes(arguments, graph_id, schema).await,
            "create_edges" => self.exec_create_edges(arguments, graph_id, schema).await,
            "update_node" => self.exec_update_node(arguments, graph_id).await,
            "update_edge" => self.exec_update_edge(arguments, graph_id).await,
            "delete_node" => self.exec_delete_node(arguments, graph_id).await,
            "delete_edge" => self.exec_delete_edge(arguments, graph_id).await,
            "done" => Ok(self.exec_done(arguments)),
            _ => Err(format!("Unknown tool: {tool_name}")),
        };

        let schema_changed =
            matches!(tool_name, "create_schema" | "create_edge_schema") && result.is_ok();
        let is_done = tool_name == "done" && result.is_ok();

        ToolResult {
            content: match result {
                Ok(content) => content,
                Err(err) => {
                    tracing::warn!(tool_name, error = %err, "Tool execution failed");
                    format!("Error: {err}")
                }
            },
            schema_changed,
            is_done,
        }
    }

    // TODO: tracing
    async fn exec_search_nodes(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let query = get_str(&args, "query")?;
        let node_key = args
            .get("node_key")
            .and_then(|v| v.as_str().map(|s| KeyDto::from(String::from(s))));
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
            .map_err(|err| format!("Embedding failed: {err}"))?;

        let nodes = self
            .knowledge_client
            .search_nodes(graph_id, node_key, embedding, limit)
            .await
            .map_err(|err| format!("Search failed: {err}"))?;

        if nodes.is_empty() {
            return Ok("No matching nodes found.".to_owned());
        }

        serde_json::to_string(&nodes).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_get_node(&self, arguments: &str, graph_id: GraphIdDto) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;

        let node = self
            .knowledge_client
            .get_node(graph_id, node_data_id)
            .await
            .map_err(|err| format!("Get node failed: {err}"))?;

        serde_json::to_string(&node).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_get_neighbors(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let edge_key = args
            .get("edge_key")
            .and_then(|v| v.as_str().map(|s| KeyDto::from(String::from(s))));
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
            .map_err(|err| format!("Get neighbors failed: {err}"))?;

        serde_json::to_string(&subgraph).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_find_paths(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let from_id = get_str(&args, "from_node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let to_id = get_str(&args, "to_node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
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
            .map_err(|err| format!("Find paths failed: {err}"))?;

        if paths.is_empty() {
            return Ok("No paths found between the specified nodes.".to_owned());
        }

        serde_json::to_string(&paths).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_read_document(&self, arguments: &str) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let document_id = get_str(&args, "document_id")
            .and_then(|s| SessionDocumentIdDto::try_from(s).map_err(|e| e.to_string()))?;

        let doc = self
            .metadata_client
            .get_session_document(document_id)
            .await
            .map_err(|err| format!("Failed to get document: {err}"))?;

        serde_json::to_string(&doc).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_create_schema(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let label = get_str(&args, "label").map(LabelDto::from)?;
        let description = get_str(&args, "description").map(DescriptionDto::from)?;

        let schema = self
            .metadata_client
            .create_node_schema(graph_id, label, description)
            .await
            .map_err(|err| format!("Failed to create node schema: {err}"))?;

        Ok(format!(
                "Created node schema '{}' with key '{}'. Use this key when creating nodes of this type.",
                schema.label, schema.key
            ))
    }

    async fn exec_create_edge_schema(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let label = get_str(&args, "label").map(LabelDto::from)?;
        let description = get_str(&args, "description").map(DescriptionDto::from)?;

        let schema = self
            .metadata_client
            .create_edge_schema(graph_id, label, description)
            .await
            .map_err(|err| format!("Failed to create edge schema: {err}"))?;

        Ok(format!(
                "Created edge schema '{}' with key '{}'. Use this key when creating edges of this type.",
                schema.label, schema.key
            ))
    }

    async fn exec_create_node(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
        schema: &GraphSchemaDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        // let node_key = get_str(&args, "node_key")?;
        let node_key = get_str(&args, "node_key").map(|s| KeyDto::from(String::from(s)))?;
        let properties = get_properties(&args)?;
        let force = args
            .get("force")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        // Validate schema exists
        validate_node_key(schema, &node_key)?;

        // Generate embedding from properties
        let props_text = properties_to_text(&properties);
        let embedding = self
            .embedding_client
            .embed_one(props_text)
            .await
            .map_err(|err| format!("Embedding failed: {err}"))?;

        // Entity resolution: search for similar existing nodes (unless forced)
        if !force {
            let similar_nodes = self
                .knowledge_client
                .search_nodes(
                    graph_id,
                    Some(node_key.clone()),
                    embedding.clone(),
                    ENTITY_RESOLUTION_LIMIT,
                )
                .await
                .map_err(|err| format!("Entity resolution search failed: {err}"))?;

            let close_matches: Vec<_> = similar_nodes
                .into_iter()
                .filter(|n| n.distance < SIMILARITY_THRESHOLD)
                .collect();

            if !close_matches.is_empty() {
                let mut candidates = Vec::new();
                for candidate in &close_matches {
                    let neighbors = self
                        .knowledge_client
                        .get_neighbors(graph_id, candidate.node_data_id, None, 1)
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
                        "properties": candidate.properties,
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

                return serde_json::to_string(&result)
                    .map_err(|err| format!("Serialization failed: {err}"));
            }
        }

        // No close matches (or force=true) — create the node
        let node = self
            .knowledge_client
            .create_node(
                graph_id,
                CreateNodeDataDto {
                    node_data_id: NodeDataIdDto::new(),
                    key: node_key,
                    properties,
                    embedding,
                },
            )
            .await
            .map_err(|err| format!("Failed to create node: {err}"))?;

        serde_json::to_value(&node)
            .map(|mut value| {
                if let Some(obj) = value.as_object_mut() {
                    obj.insert("created".to_owned(), Value::Bool(true));
                }
                value
            })
            .and_then(|value| serde_json::to_string(&value))
            .map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_create_edge(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
        schema: &GraphSchemaDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_key = get_str(&args, "edge_key").map(|s| KeyDto::from(String::from(s)))?;
        let from_id = get_str(&args, "from_node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let to_id = get_str(&args, "to_node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let properties = get_properties(&args)?;

        // Validate edge schema exists
        validate_edge_key(schema, &edge_key)?;

        let edge = self
            .knowledge_client
            .create_edge(
                graph_id,
                CreateEdgeDataDto {
                    from_node_data_id: from_id,
                    to_node_data_id: to_id,
                    key: edge_key,
                    properties,
                },
            )
            .await
            .map_err(|err| format!("Failed to create edge: {err}"))?;

        serde_json::to_value(&edge)
            .map(|mut value| {
                if let Some(obj) = value.as_object_mut() {
                    obj.insert("created".to_owned(), serde_json::json!(true));
                }
                value
            })
            .and_then(|value| serde_json::to_string(&value))
            .map_err(|err| format!("Serialization failed: {err}"))
    }

    /// Batch create up to 50 nodes. Embeddings are generated in a single API call.
    /// Entity resolution still runs per-node (N calls to knowledge service).
    // NOTE: This generates O(N) calls to the knowledge service for entity resolution
    // (search_nodes + get_neighbors per node). A future optimisation would be to add
    // a batch entity-resolution endpoint to the knowledge service.
    async fn exec_create_nodes(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
        schema: &GraphSchemaDto,
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
        let entries: Vec<(KeyDto, PropertiesDataDto, bool)> = nodes_arr
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let node_key = item
                    .get("node_key")
                    .and_then(|v| v.as_str())
                    .map(|s| KeyDto::from(s.to_owned()))
                    .ok_or_else(|| format!("nodes[{i}]: missing node_key"))?;
                validate_node_key(schema, &node_key)?;
                let properties = get_properties(item)?;
                let force = item
                    .get("force")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false);
                Ok((node_key, properties, force))
            })
            .collect::<Result<_, String>>()?;

        // Batch embed all nodes in a single API call
        let texts: Vec<String> = entries
            .iter()
            .map(|(_, props, _)| properties_to_text(props))
            .collect();
        let embeddings = self
            .embedding_client
            .embed(texts)
            .await
            .map_err(|err| format!("Batch embedding failed: {err}"))?;

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
                        Some(node_key.clone()),
                        embedding.clone(),
                        ENTITY_RESOLUTION_LIMIT,
                    )
                    .await
                    .map_err(|err| format!("nodes[{i}]: entity resolution failed: {err}"))?;

                let close_matches: Vec<_> = similar_nodes
                    .into_iter()
                    .filter(|n| n.distance < SIMILARITY_THRESHOLD)
                    .collect();

                if !close_matches.is_empty() {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "reason": "Similar node already exists",
                        "similar_nodes": close_matches,
                    }));
                    continue;
                }
            }

            // Create the node
            match self
                .knowledge_client
                .create_node(
                    graph_id,
                    CreateNodeDataDto {
                        node_data_id: NodeDataIdDto::new(),
                        key: node_key,
                        properties,
                        embedding,
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
                        "properties": node.properties,
                    }));
                }
                Err(err) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "error": format!("{err}"),
                    }));
                }
            }
        }

        serde_json::to_string(&results).map_err(|err| format!("Serialization failed: {err}"))
    }

    /// Batch create up to 50 edges. Each edge is merged if it already exists.
    // NOTE: This generates O(N) create_edge calls to the knowledge service.
    // A future optimisation would be a batch insert endpoint.
    async fn exec_create_edges(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
        schema: &GraphSchemaDto,
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
                .map(|s| KeyDto::from(s.to_owned()))
                .ok_or_else(|| format!("edges[{i}]: missing edge_key"))?;
            let from_id = item
                .get("from_node_data_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("edges[{i}]: missing from_node_data_id"))
                .and_then(|s| {
                    NodeDataIdDto::try_from(s)
                        .map_err(|e| format!("edges[{i}]: invalid from_node_data_id: {e}"))
                })?;
            let to_id = item
                .get("to_node_data_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| format!("edges[{i}]: missing to_node_data_id"))
                .and_then(|s| {
                    NodeDataIdDto::try_from(s)
                        .map_err(|e| format!("edges[{i}]: invalid to_node_data_id: {e}"))
                })?;

            validate_edge_key(schema, &edge_key)?;

            let properties = get_properties(item)?;
            match self
                .knowledge_client
                .create_edge(
                    graph_id,
                    CreateEdgeDataDto {
                        from_node_data_id: from_id,
                        to_node_data_id: to_id,
                        key: edge_key,
                        properties,
                    },
                )
                .await
            {
                Ok(edge) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": true,
                        "edge_key": edge.key,
                        "from_node_data_id": edge.from_node_data_id,
                        "to_node_data_id": edge.to_node_data_id,
                    }));
                }
                Err(err) => {
                    results.push(serde_json::json!({
                        "index": i,
                        "created": false,
                        "error": format!("{err}"),
                    }));
                }
            }
        }

        serde_json::to_string(&results).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_update_node(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let properties = get_properties(&args)?;

        // Re-embed from updated properties
        let props_text = properties_to_text(&properties);
        let embedding = self
            .embedding_client
            .embed_one(props_text)
            .await
            .map_err(|err| format!("Embedding failed: {err}"))?;

        let node = self
            .knowledge_client
            .update_node(
                graph_id,
                UpdateNodeDataDto {
                    node_data_id,
                    properties,
                    embedding,
                },
            )
            .await
            .map_err(|err| format!("Failed to update node: {err}"))?;

        serde_json::to_string(&node).map_err(|err| format!("Serialization failed: {err}"))
    }

    // TODO: doesn't update embeddings ?
    async fn exec_update_edge(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_data_id = get_str(&args, "edge_data_id")
            .and_then(|s| EdgeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;
        let properties = get_properties(&args)?;

        let edge = self
            .knowledge_client
            .update_edge(
                graph_id,
                UpdateEdgeDataDto {
                    edge_data_id,
                    properties,
                },
            )
            .await
            .map_err(|err| format!("Failed to update edge: {err}"))?;

        serde_json::to_string_pretty(&edge).map_err(|err| format!("Serialization failed: {err}"))
    }

    async fn exec_delete_node(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let node_data_id = get_str(&args, "node_data_id")
            .and_then(|s| NodeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;

        self.knowledge_client
            .delete_node(graph_id, node_data_id)
            .await
            .map_err(|err| format!("Failed to delete node: {err}"))?;

        Ok(format!("Deleted node {node_data_id} and all its edges."))
    }

    async fn exec_delete_edge(
        &self,
        arguments: &str,
        graph_id: GraphIdDto,
    ) -> Result<String, String> {
        let args: Value = parse_args(arguments)?;
        let edge_data_id = get_str(&args, "edge_data_id")
            .and_then(|s| EdgeDataIdDto::try_from(s).map_err(|e| e.to_string()))?;

        self.knowledge_client
            .delete_edge(graph_id, edge_data_id)
            .await
            .map_err(|err| format!("Failed to delete edge: {err}"))?;

        Ok(format!("Deleted edge {edge_data_id}."))
    }

    #[allow(clippy::unused_self)]
    fn exec_done(&self, arguments: &str) -> String {
        parse_args(arguments)
            .ok()
            .and_then(|args| {
                args.get("summary")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
            .unwrap_or_else(|| "Task completed.".to_owned())
    }
}

// --- Helpers ---

fn parse_args(arguments: &str) -> Result<Value, String> {
    serde_json::from_str(arguments).map_err(|err| format!("Invalid tool arguments: {err}"))
}

fn get_str<'a>(args: &'a Value, field: &str) -> Result<&'a str, String> {
    args.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Missing required field: {field}"))
}

fn get_properties(args: &Value) -> Result<PropertiesDataDto, String> {
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
    let props = if let Some(obj) = args.get("properties").and_then(|v| v.as_object()) {
        obj.iter()
            .filter(|(k, _)| !NON_PROPERTY_KEYS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<serde_json::Map<String, Value>>()
    } else {
        // Fallback: LLM sometimes puts properties at top level instead of nesting
        let filtered = args
            .as_object()
            .ok_or("Missing required field: properties")?
            .iter()
            .filter(|(k, _)| !NON_PROPERTY_KEYS.contains(&k.as_str()))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<serde_json::Map<String, Value>>();

        if filtered.is_empty() {
            return Err("Missing required field: properties".to_owned());
        }
        filtered
    };

    serde_json::from_value(Value::Object(props)).map_err(|err| format!("Invalid properties: {err}"))
}

fn validate_node_key(schema: &GraphSchemaDto, node_key: &KeyDto) -> Result<(), String> {
    if schema
        .nodes
        .iter()
        .any(|n| n.key.as_str() == node_key.as_str())
    {
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

fn validate_edge_key(schema: &GraphSchemaDto, edge_key: &KeyDto) -> Result<(), String> {
    if schema
        .edges
        .iter()
        .any(|err| err.key.as_str() == edge_key.as_str())
    {
        return Ok(());
    }
    let valid: Vec<String> = schema
        .edges
        .iter()
        .map(|err| format!("{} ({})", err.key, err.label))
        .collect();
    Err(format!(
        "Unknown edge schema key '{edge_key}'. Valid schemas: {}",
        valid.join(", ")
    ))
}

fn properties_to_text(properties: &PropertiesDataDto) -> String {
    properties
        .values
        .iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("; ")
}
