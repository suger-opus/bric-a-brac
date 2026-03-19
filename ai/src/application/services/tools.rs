use crate::infrastructure::clients::ToolDefinition;
use serde_json::json;

fn tool(name: &str, description: &str, parameters: serde_json::Value) -> ToolDefinition {
    ToolDefinition {
        type_: "function".to_owned(),
        function: crate::infrastructure::clients::FunctionDefinition {
            name: name.to_owned(),
            description: description.to_owned(),
            parameters,
        },
    }
}

pub fn read_tools() -> Vec<ToolDefinition> {
    vec![
        search_nodes_tool(),
        get_node_tool(),
        get_neighbors_tool(),
        find_paths_tool(),
    ]
}

pub fn write_tools() -> Vec<ToolDefinition> {
    vec![
        create_schema_tool(),
        create_edge_schema_tool(),
        create_node_tool(),
        create_edge_tool(),
        update_node_tool(),
    ]
}

pub fn session_tools() -> Vec<ToolDefinition> {
    vec![done_tool()]
}

fn search_nodes_tool() -> ToolDefinition {
    tool(
        "search_nodes",
        "Search for nodes by semantic similarity. Returns the most relevant nodes matching the query text.",
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query text to find semantically similar nodes"
                },
                "node_key": {
                    "type": "string",
                    "description": "Optional node schema key to filter search to a specific node type"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results to return (default: 10)"
                }
            },
            "required": ["query"],
            "additionalProperties": false
        }),
    )
}

fn get_node_tool() -> ToolDefinition {
    tool(
        "get_node",
        "Retrieve a specific node by its ID. Returns all properties of the node.",
        json!({
            "type": "object",
            "properties": {
                "node_data_id": {
                    "type": "string",
                    "description": "The unique ID of the node to retrieve"
                }
            },
            "required": ["node_data_id"],
            "additionalProperties": false
        }),
    )
}

fn get_neighbors_tool() -> ToolDefinition {
    tool(
        "get_neighbors",
        "Get the neighboring nodes and edges connected to a specific node. Useful for exploring the graph structure around a node.",
        json!({
            "type": "object",
            "properties": {
                "node_data_id": {
                    "type": "string",
                    "description": "The ID of the node to get neighbors for"
                },
                "edge_key": {
                    "type": "string",
                    "description": "Optional edge schema key to filter by relationship type"
                },
                "depth": {
                    "type": "integer",
                    "description": "How many hops to traverse (default: 1)"
                }
            },
            "required": ["node_data_id"],
            "additionalProperties": false
        }),
    )
}

fn find_paths_tool() -> ToolDefinition {
    tool(
        "find_paths",
        "Find all paths between two nodes in the graph. Useful for discovering how two entities are connected.",
        json!({
            "type": "object",
            "properties": {
                "from_node_data_id": {
                    "type": "string",
                    "description": "The ID of the starting node"
                },
                "to_node_data_id": {
                    "type": "string",
                    "description": "The ID of the target node"
                },
                "max_depth": {
                    "type": "integer",
                    "description": "Maximum path length in hops (default: 5)"
                }
            },
            "required": ["from_node_data_id", "to_node_data_id"],
            "additionalProperties": false
        }),
    )
}

fn create_schema_tool() -> ToolDefinition {
    tool(
        "create_schema",
        "Create a new node schema (type) in the graph. Use this when you need to store a new kind of entity that doesn't match any existing schema.",
        json!({
            "type": "object",
            "properties": {
                "label": {
                    "type": "string",
                    "description": "Human-readable name for the node schema (e.g. 'Person', 'Company', 'Event')"
                },
                "description": {
                    "type": "string",
                    "description": "A brief description of what this node schema represents"
                }
            },
            "required": ["label", "description"],
            "additionalProperties": false
        }),
    )
}

fn create_edge_schema_tool() -> ToolDefinition {
    tool(
        "create_edge_schema",
        "Create a new edge schema (relationship type) in the graph. Use this when you need a new kind of relationship between nodes.",
        json!({
            "type": "object",
            "properties": {
                "label": {
                    "type": "string",
                    "description": "Human-readable name for the edge schema (e.g. 'WorksAt', 'Knows', 'LocatedIn')"
                },
                "description": {
                    "type": "string",
                    "description": "A brief description of what this relationship represents"
                }
            },
            "required": ["label", "description"],
            "additionalProperties": false
        }),
    )
}

fn create_node_tool() -> ToolDefinition {
    tool(
        "create_node",
        "Create a new node in the graph. The system will automatically check for similar existing nodes and warn you about potential duplicates. You can then decide to update an existing node instead.",
        json!({
            "type": "object",
            "properties": {
                "node_key": {
                    "type": "string",
                    "description": "The schema key identifying the node type (from the schema list)"
                },
                "properties": {
                    "type": "object",
                    "description": "Free-form properties for the node. Keys are property names, values are strings, numbers, or booleans.",
                    "additionalProperties": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"}
                        ]
                    }
                }
            },
            "required": ["node_key", "properties"],
            "additionalProperties": false
        }),
    )
}

fn create_edge_tool() -> ToolDefinition {
    tool(
        "create_edge",
        "Create a new edge (relationship) between two nodes in the graph.",
        json!({
            "type": "object",
            "properties": {
                "edge_key": {
                    "type": "string",
                    "description": "The schema key identifying the edge type (from the schema list)"
                },
                "from_node_data_id": {
                    "type": "string",
                    "description": "The ID of the source node"
                },
                "to_node_data_id": {
                    "type": "string",
                    "description": "The ID of the target node"
                },
                "properties": {
                    "type": "object",
                    "description": "Optional free-form properties for the edge.",
                    "additionalProperties": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"}
                        ]
                    }
                }
            },
            "required": ["edge_key", "from_node_data_id", "to_node_data_id"],
            "additionalProperties": false
        }),
    )
}

fn update_node_tool() -> ToolDefinition {
    tool(
        "update_node",
        "Update the properties of an existing node. Replaces all properties with the new set. The embedding will be automatically updated.",
        json!({
            "type": "object",
            "properties": {
                "node_data_id": {
                    "type": "string",
                    "description": "The ID of the node to update"
                },
                "properties": {
                    "type": "object",
                    "description": "The complete set of properties for the node. Keys are property names, values are strings, numbers, or booleans.",
                    "additionalProperties": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "number"},
                            {"type": "boolean"}
                        ]
                    }
                }
            },
            "required": ["node_data_id", "properties"],
            "additionalProperties": false
        }),
    )
}

fn done_tool() -> ToolDefinition {
    tool(
        "done",
        "Signal that you have completed the current task. Call this when you have finished processing the user's request and have no more actions to take.",
        json!({
            "type": "object",
            "properties": {
                "summary": {
                    "type": "string",
                    "description": "A brief summary of what was accomplished"
                }
            },
            "required": ["summary"],
            "additionalProperties": false
        }),
    )
}
