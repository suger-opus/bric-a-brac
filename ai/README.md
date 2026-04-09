# AI Service

The **stateless agent microservice** that orchestrates the LLM tool-calling loop. It
receives a user message (via gRPC from the metadata service), runs an autonomous agent
that reads and writes to the knowledge graph, and streams events back in real time.

No database — all state lives in the metadata service (sessions, messages) and the
knowledge service (graph data). The AI service is a pure compute node.

---

## Architecture

```
src/
├── main.rs                         # Entry point: load config, build clients, start gRPC
├── lib.rs                          # Wire up layers, start gRPC server
├── application/
│   ├── services/
│   │   ├── agent_service.rs        # Top-level orchestrator, spawns agent loops
│   │   ├── agent_loop.rs           # Core loop: LLM call → tool execution → repeat
│   │   ├── tool_service.rs         # Tool executor: dispatch + role guard
│   │   ├── tools/                  # Individual tool implementations
│   │   │   ├── search_nodes.rs     # Vector similarity search
│   │   │   ├── get_node.rs         # Fetch single node with all properties
│   │   │   ├── get_neighbors.rs    # Subgraph traversal (depth-configurable)
│   │   │   ├── find_paths.rs       # Shortest paths between two nodes
│   │   │   ├── read_document.rs    # Load session document by ID
│   │   │   ├── create_node.rs      # Single node with entity resolution
│   │   │   ├── create_nodes.rs     # Batch (up to 50), single embedding call
│   │   │   ├── create_edge.rs      # Single edge (MERGE semantics)
│   │   │   ├── create_edges.rs     # Batch (up to 50)
│   │   │   ├── create_schema.rs    # Node schema creation
│   │   │   ├── create_edge_schema.rs
│   │   │   ├── update_node.rs      # Merge properties + recompute embedding
│   │   │   ├── update_edge.rs      # Merge edge properties
│   │   │   ├── delete_node.rs      # DETACH DELETE
│   │   │   ├── delete_edge.rs      # Delete single relationship
│   │   │   └── done.rs             # Terminate the agent loop
│   │   ├── prompt.rs               # Dynamic system prompt builder
│   │   └── context.rs              # Context window management (500K budget)
│   └── error.rs
├── infrastructure/
│   ├── clients/
│   │   ├── knowledge_client.rs     # gRPC client to knowledge service
│   │   ├── metadata_client.rs      # gRPC client to metadata service
│   │   ├── openrouter_client.rs    # HTTP client for LLM chat completions
│   │   └── embedding_client.rs     # HTTP client for embeddings
│   ├── config/                     # Environment-based configuration
│   └── http_retry.rs              # Retry with backoff for OpenRouter (429/5xx)
└── presentation/
    └── service.rs                  # gRPC Ai trait implementation
```

### Layers

| Layer | Responsibility |
|-------|---------------|
| **Presentation** | gRPC `Ai::SendMessage` implementation. Spawns the agent loop and bridges events to a gRPC response stream |
| **Application** | Agent orchestration: prompt building, LLM calls, tool dispatch, context window trimming, entity resolution, document chunking |
| **Infrastructure** | HTTP/gRPC clients, configuration, retry logic |

---

## Agent Loop

1. Load session + message history + graph schemas from metadata gRPC
2. Build the dynamic system prompt (identity + current schemas + 5-phase workflow + rules)
3. If a document was uploaded, chunk it (~8K chars at paragraph boundaries)
4. For each chunk (or the single message):
   - Build context window (trim to 500K char budget)
   - Call OpenRouter with tools + history → stream tokens as `AgentText` events
   - If tool calls returned → execute them → send `ToolCall`/`ToolResult` events → loop
   - If text-only response → auto-done
   - If `done` tool called → stream summary → end
5. Persist all new messages to metadata via gRPC
6. Send `Done` or `Error` event to close the stream

### Concurrency Controls

- **LLM semaphore** — 20 permits across all sessions. Prevents overwhelming OpenRouter
- **One active session per graph** — enforced by the metadata service
- **Max 200 tool calls** — hard safety limit per session message

### Entity Resolution

Baked into `create_node` (not prompt-dependent):

1. Serialize properties → embed → search for similar existing nodes
2. If similar nodes found (distance < 0.3): block creation, return candidates with
   first-degree neighbors for the LLM to inspect
3. LLM decides: merge (`update_node`) or force-create (`create_node` with `force: true`)

### Role-Based Tool Filtering

The agent receives different tool sets based on the user's role:
- **Owner / Admin / Editor:** read tools + write tools + session tools
- **Viewer:** read tools + session tools only

A defense-in-depth executor guard independently checks the role before running any write tool.

---

## gRPC API

Defined in [`ai.proto`](../crates/bric-a-brac-protos/protos/ai.proto):

```protobuf
service Ai {
  rpc SendMessage(SendMessageRequest) returns (stream AgentEventProto);
}
```

**Request:** `session_id`, `content`, optional `document_id`

**Stream events:** `text` (tokens), `tool_call`, `tool_result`, `done`, `error`, `progress`

---

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `AI_SERVER_HOST` | Bind address | `0.0.0.0` |
| `AI_GRPC_SERVER_PORT` | gRPC listen port | `50053` |
| `OPENROUTER_API_KEY` | OpenRouter API key | **Yes** |
| `OPENROUTER_DEFAULT_MODEL` | Chat model | `openai/gpt-4.1` |
| `OPENROUTER_EMBEDDING_MODEL` | Embedding model | `openai/text-embedding-3-small` |
| `KNOWLEDGE_GRPC_SERVER_URL` | Knowledge service URL | `http://localhost:50051` |
| `METADATA_GRPC_SERVER_URL` | Metadata service URL | `http://localhost:50052` |
| `INTERNAL_SERVICES_AUTH_TOKEN` | Shared gRPC auth token | **Yes** |
| `RUST_LOG` | Tracing filter | `info` |

For local development, configure these in `mise.local.toml`:

```toml
[env]
OPENROUTER_API_KEY = "sk-or-..."
KNOWLEDGE_GRPC_SERVER_URL = "http://localhost:50051"
METADATA_GRPC_SERVER_URL = "http://localhost:50052"
INTERNAL_SERVICES_AUTH_TOKEN = "dev-token"
```

---

## Running

### Locally

```bash
# Prerequisites: knowledge and metadata services must be running
cargo run
```

### With Docker

```bash
docker compose --profile ai up -d
```

The Dockerfile uses a multi-stage build: `rust:1.93` builder with `protobuf-compiler`,
then `debian:bookworm-slim` runtime (~50MB image).