# AI Agent Roadmap

Precise implementation plan for the AI agent architecture described in `AI_AGENT_DESIGN.md`.
Each step lists exactly what to create, modify, or delete — with file paths and key details.

---

## Step 1: Proto Definitions ✅

**Status:** COMPLETED

**Goal:** Define all new messages and RPCs. Once compiled, all services can start implementing
handlers and clients independently.

### 1.1 — Update `common.proto`

**File:** `crates/bric-a-brac-protos/protos/common.proto`

Add messages for single-node and single-edge operations (used by knowledge service):

```protobuf
message InsertNodeDataProto {
    string node_data_id = 1;
    string key = 2;
    map<string, PropertyValueProto> properties = 3;
    repeated float embedding = 4;
    optional string session_id = 5;
}

message InsertEdgeDataProto {
    string from_node_data_id = 1;
    string to_node_data_id = 2;
    string key = 3;
    map<string, PropertyValueProto> properties = 4;
    optional string session_id = 5;
}

message UpdateNodeDataProto {
    string node_data_id = 1;
    map<string, PropertyValueProto> properties = 2;
    repeated float embedding = 3;
}

message NodeSummaryProto {
    string node_data_id = 1;
    string key = 2;
    map<string, PropertyValueProto> properties = 3;
    float distance = 4;
}

message PathProto {
    repeated NodeDataProto nodes = 1;
    repeated EdgeDataProto edges = 2;
}

message SubgraphProto {
    repeated NodeDataProto nodes = 1;
    repeated EdgeDataProto edges = 2;
}
```

### 1.2 — Update `knowledge.proto`

**File:** `crates/bric-a-brac-protos/protos/knowledge.proto`

Add new RPCs to the `Knowledge` service:

```protobuf
service Knowledge {
    // Existing
    rpc LoadGraph(LoadGraphRequest) returns (common.GraphDataProto);
    rpc InsertGraph(InsertGraphRequest) returns (common.GraphDataProto);

    // New
    rpc InitializeSchema(InitializeSchemaRequest) returns (InitializeSchemaResponse);
    rpc InsertNode(InsertNodeRequest) returns (common.NodeDataProto);
    rpc UpdateNode(UpdateNodeRequest) returns (common.NodeDataProto);
    rpc InsertEdge(InsertEdgeRequest) returns (common.EdgeDataProto);
    rpc SearchNodes(SearchNodesRequest) returns (SearchNodesResponse);
    rpc GetNode(GetNodeRequest) returns (common.NodeDataProto);
    rpc GetNeighbors(GetNeighborsRequest) returns (GetNeighborsResponse);
    rpc FindPaths(FindPathsRequest) returns (FindPathsResponse);
}
```

New request/response messages:

```protobuf
message InitializeSchemaRequest {
    string graph_id = 1;
    repeated string node_keys = 2;  // Memgraph labels to create vector indexes on
}
message InitializeSchemaResponse {}

message InsertNodeRequest {
    string graph_id = 1;
    common.InsertNodeDataProto node = 2;
}

message UpdateNodeRequest {
    string graph_id = 1;
    common.UpdateNodeDataProto node = 2;
}

message InsertEdgeRequest {
    string graph_id = 1;
    common.InsertEdgeDataProto edge = 2;
}

message SearchNodesRequest {
    string graph_id = 1;
    optional string node_key = 2;   // filter by label; omit to search all types
    repeated float query_embedding = 3;
    int32 limit = 4;
}
message SearchNodesResponse {
    repeated common.NodeSummaryProto nodes = 1;
}

message GetNodeRequest {
    string graph_id = 1;
    string node_data_id = 2;
}

message GetNeighborsRequest {
    string graph_id = 1;
    string node_data_id = 2;
    optional string edge_key = 3;
    int32 depth = 4;
}
message GetNeighborsResponse {
    common.SubgraphProto subgraph = 1;
}

message FindPathsRequest {
    string graph_id = 1;
    string from_node_data_id = 2;
    string to_node_data_id = 3;
    int32 max_depth = 4;
}
message FindPathsResponse {
    repeated common.PathProto paths = 1;
}
```

### 1.3 — Update `ai.proto`

**File:** `crates/bric-a-brac-protos/protos/ai.proto`

Add the `SendMessage` streaming RPC and agent event messages:

```protobuf
service Ai {
    // Existing
    rpc GenerateSchema(GenerateSchemaRequest) returns (common.CreateGraphSchemaProto);
    rpc GenerateData(GenerateDataRequest) returns (common.CreateGraphDataProto);

    // New
    rpc SendMessage(SendMessageRequest) returns (stream AgentEventProto);
}

message SendMessageRequest {
    string session_id = 1;
    string content = 2;  // user message text (may include document content)
}

message AgentEventProto {
    oneof event {
        AgentTextProto text = 1;
        AgentToolCallProto tool_call = 2;
        AgentToolResultProto tool_result = 3;
        AgentDoneProto done = 4;
        AgentErrorProto error = 5;
    }
}

message AgentTextProto {
    string content = 1;  // incremental token(s)
}

message AgentToolCallProto {
    string tool_call_id = 1;
    string name = 2;
    string arguments = 3;  // JSON string
}

message AgentToolResultProto {
    string tool_call_id = 1;
    string content = 2;  // JSON string
}

message AgentDoneProto {
    string summary = 1;
}

message AgentErrorProto {
    string message = 1;
}
```

### 1.4 — Update `build.rs`

**File:** `crates/bric-a-brac-protos/build.rs`

No changes needed — it already compiles all three proto files.

### 1.5 — Compile and verify

```bash
cargo build -p bric-a-brac-protos
```

Fix any compilation errors in generated code.

### Checklist

- [x] `common.proto` — 6 new messages: `InsertNodeDataProto`, `InsertEdgeDataProto`, `UpdateNodeDataProto`, `NodeSummaryProto`, `PathProto`, `SubgraphProto`
- [x] `knowledge.proto` — 8 new RPCs with all request/response messages
- [x] `ai.proto` — `SendMessage` streaming RPC with `AgentEventProto` (5 event variants)
- [x] `cargo build -p bric-a-brac-protos` passes
- [x] `cargo build -p knowledge -p ai -p metadata` passes (todo!() stubs added for new RPCs)
- [x] All 6 existing tests still pass
- [x] `tokio-stream` added to workspace + AI crate dependencies (needed for `ReceiverStream`)

**Extra changes (stubs for compilation):**
- `knowledge/src/presentation/grpc/knowledge_service.rs` — `todo!()` stubs for 8 new RPCs
- `ai/src/presentation/grpc/ai_service.rs` — `todo!()` stub for `SendMessage`

---

## Step 2: Knowledge Service — New Handlers

**Goal:** Implement all 8 new RPCs in the knowledge service. These have no dependency on the
agent — they're pure graph operations.

### 2.1 — New repository methods

**File:** `knowledge/src/infrastructure/repositories/mutate_repository.rs`

Add methods:

| Method | Cypher template |
|---|---|
| `insert_node(txn, graph_id, InsertNodeData)` | `CREATE (n:{key}) SET n = $props, n.graph_id = $gid, n.node_data_id = $nid, n.session_id = $sid, n.embedding = $emb RETURN n` |
| `update_node(txn, graph_id, UpdateNodeData)` | `MATCH (n { node_data_id: $nid, graph_id: $gid }) SET n += $props, n.embedding = $emb RETURN n` |
| `insert_edge(txn, graph_id, InsertEdgeData)` | `MATCH (a { node_data_id: $from, graph_id: $gid }), (b { node_data_id: $to, graph_id: $gid }) CREATE (a)-[r:{key} $props]->(b) SET r.edge_data_id = $eid, r.session_id = $sid RETURN r` |

**Important:** The `{key}` (node label / edge type) is **not** a Cypher parameter — it's
interpolated into the query string. It **must** be validated against the schema (alphanumeric,
8 chars, matches `^[a-zA-Z][a-zA-Z0-9]{7}$`) before interpolation. Property values go through
neo4rs `BoltType` parameters.

**File:** `knowledge/src/infrastructure/repositories/query_repository.rs`

Add methods:

| Method | Cypher template |
|---|---|
| `get_node(txn, graph_id, node_data_id)` | `MATCH (n { node_data_id: $nid, graph_id: $gid }) RETURN n` |
| `search_nodes(txn, graph_id, key?, embedding, limit)` | Per label: `CALL vector_search.search({label}, "embedding", $emb, $limit) YIELD node, distance WHERE node.graph_id = $gid RETURN node, distance` — if no key, run for all labels and merge |
| `get_neighbors(txn, graph_id, node_data_id, edge_key?, depth)` | `MATCH path = (n { node_data_id: $nid, graph_id: $gid })-[*1..{depth}]-(m) RETURN path` (filter by edge type if provided) |
| `find_paths(txn, graph_id, from_id, to_id, max_depth)` | `MATCH path = shortestPath((a { node_data_id: $from, graph_id: $gid })-[*..{max_depth}]-(b { node_data_id: $to, graph_id: $gid })) RETURN path` |
| `initialize_schema(txn, graph_id, node_keys)` | For each key: `CREATE VECTOR INDEX ON :{key}(embedding) OPTIONS { dimension: 1536, capacity: 10000, metric: "cos" } IF NOT EXISTS` |

Note: `{depth}` and `{max_depth}` are also interpolated (integers). Validate they are within
bounds (1-10) before interpolation.

### 2.2 — New domain models

**File:** `knowledge/src/domain/models/` — add or update:

- `InsertNodeDataModel` — `node_data_id`, `key`, `properties`, `embedding: Vec<f32>`, `session_id: Option<String>`
- `UpdateNodeDataModel` — `node_data_id`, `properties`, `embedding: Vec<f32>`
- `InsertEdgeDataModel` — `edge_data_id`, `key`, `from_node_data_id`, `to_node_data_id`, `properties`, `session_id: Option<String>`
- `NodeSummaryModel` — `node_data_id`, `key`, `properties`, `distance: f32`
- Proto conversions (`From`/`TryFrom` impls) for all new types

### 2.3 — New application service methods

**File:** `knowledge/src/application/services/mutate_service.rs`

Add: `insert_node()`, `update_node()`, `insert_edge()`, `initialize_schema()`

**File:** `knowledge/src/application/services/query_service.rs`

Add: `get_node()`, `search_nodes()`, `get_neighbors()`, `find_paths()`

### 2.4 — Update gRPC handler

**File:** `knowledge/src/presentation/grpc/knowledge_service.rs`

Implement the 8 new RPC methods on the `Knowledge` trait. Each one:
1. Extracts and validates request fields
2. Calls the appropriate service method
3. Converts domain model to proto and returns

### 2.5 — Test

- Unit tests for each repository method (mock graph or integration test with Memgraph)
- Verify vector index creation works on empty labels
- Verify `search_nodes` without `node_key` merges results from multiple labels

### Checklist

- [ ] `mutate_repository.rs` — `insert_node`, `update_node`, `insert_edge`, `initialize_schema`
- [ ] `query_repository.rs` — `get_node`, `search_nodes`, `get_neighbors`, `find_paths`
- [ ] Domain models + proto conversions
- [ ] `mutate_service.rs` — 4 new methods
- [ ] `query_service.rs` — 4 new methods
- [ ] `knowledge_service.rs` — 8 new RPC handlers
- [ ] Tests passing
- [ ] `cargo build -p knowledge` passes

---

## Step 3: Metadata Service — Sessions + InitializeSchema Hook

**Goal:** Add session management and the schema creation hook that initializes vector indexes.

### 3.1 — Database migration

**File:** `metadata/migrations/YYYYMMDDHHMMSS_sessions.up.sql` (new)

```sql
CREATE TABLE sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    graph_id    UUID NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    user_role   TEXT NOT NULL CHECK (user_role IN ('read', 'write')),
    status      TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'error')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE session_messages (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id    UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    position      INTEGER NOT NULL,
    role          TEXT NOT NULL CHECK (role IN ('system', 'user', 'assistant', 'tool')),
    content       TEXT,
    tool_calls    JSONB,
    tool_call_id  TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_session_messages_session ON session_messages(session_id, position);
CREATE INDEX idx_sessions_graph ON sessions(graph_id);
```

**File:** `metadata/migrations/YYYYMMDDHHMMSS_sessions.down.sql` (new)

```sql
DROP TABLE session_messages;
DROP TABLE sessions;
```

### 3.2 — Domain models

**File:** `metadata/src/domain/models/` — new files:

- `session_model.rs` — `SessionModel`, `CreateSessionModel`, `SessionIdModel`, `SessionStatusModel`
- `session_message_model.rs` — `SessionMessageModel`, `CreateSessionMessageModel`

Update `metadata/src/domain/models/mod.rs` to export them.

### 3.3 — Repository

**File:** `metadata/src/infrastructure/repositories/` — new `session_repository.rs`

Methods:
- `create_session(txn, CreateSessionModel) → SessionModel`
- `get_session(txn, session_id) → SessionModel`
- `update_session_status(txn, session_id, status)`
- `get_messages(txn, session_id) → Vec<SessionMessageModel>` (ordered by position)
- `insert_messages(txn, session_id, Vec<CreateSessionMessageModel>)` — batch insert
- `get_latest_position(txn, session_id) → i32` — for appending new messages

### 3.4 — Application service

**File:** `metadata/src/application/services/` — new `session_service.rs`

Methods:
- `create_session(graph_id, user_id, user_role) → SessionDto`
- `get_session(session_id) → SessionDto`
- `get_messages(session_id) → Vec<SessionMessageDto>`
- `append_messages(session_id, Vec<CreateSessionMessageDto>)`
- `close_session(session_id)`

### 3.5 — HTTP handlers

**File:** `metadata/src/presentation/` — new session routes

| Route | Handler |
|---|---|
| `POST /graphs/:graph_id/sessions` | Create session (body: `{ user_role: "read" \| "write" }`) |
| `GET /sessions/:session_id` | Get session metadata |
| `GET /sessions/:session_id/messages` | Get all messages |
| `POST /sessions/:session_id/messages` | Append messages (used by AI service) |
| `PATCH /sessions/:session_id` | Update status (close) |

### 3.6 — InitializeSchema hook

**File:** `metadata/src/application/services/graph_service.rs`

In `create_schema()`, after the Postgres transaction commits, call:
```rust
self.knowledge_client
    .initialize_schema(graph_id, node_keys)
    .await?;
```

Where `node_keys` is the list of `key` values from the node schemas just created.

This requires adding the `initialize_schema` method to the existing `KnowledgeClient` in
the metadata service (`metadata/src/infrastructure/clients/knowledge_client.rs`).

### 3.7 — DTOs

**File:** `metadata/src/application/dtos/` — new session DTOs

- `SessionDto`, `CreateSessionDto`, `SessionMessageDto`, `CreateSessionMessageDto`

### Checklist

- [ ] Migration — `sessions` + `session_messages` tables
- [ ] Domain models — `SessionModel`, `SessionMessageModel`
- [ ] `session_repository.rs` with all CRUD methods
- [ ] `session_service.rs` with business logic
- [ ] HTTP handlers for session CRUD + message management
- [ ] DTOs for session data
- [ ] `create_schema` now calls `knowledge_client.initialize_schema()`
- [ ] `KnowledgeClient::initialize_schema()` method added
- [ ] Run migration: `sqlx migrate run`
- [ ] Tests passing
- [ ] `cargo build -p metadata` passes

---

## Step 4: AI Service — OpenRouterClient Upgrades

**Goal:** Add SSE streaming and tool calling support to `OpenRouterClient`. Add `EmbeddingClient`.

### 4.1 — SSE streaming support

**File:** `ai/src/infrastructure/clients/openrouter_client.rs`

Add a new method alongside the existing `chat()`:

```rust
pub async fn chat_stream(
    &self,
    messages: Vec<Message>,
    tools: Option<Vec<ToolDefinition>>,
) -> Result<impl Stream<Item = Result<StreamChunk, OpenRouterClientError>>, OpenRouterClientError>
```

`StreamChunk` variants:
- `Delta { content: String }` — incremental text token
- `ToolCallDelta { index: usize, id: String, name: String, arguments: String }` — incremental tool call
- `Done { full_message: AssistantMessage }` — complete parsed message

The method:
1. Sends request with `"stream": true` and optional `tools` array
2. Returns an async stream that reads the SSE response line by line
3. Accumulates deltas into a complete `AssistantMessage` at the end

**Key types to add:**

```rust
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,  // JSON Schema for the tool's parameters
}

pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,  // JSON string
}

pub struct AssistantMessage {
    pub content: Option<String>,
    pub tool_calls: Vec<ToolCall>,
}
```

### 4.2 — Provider routing

**File:** `ai/src/infrastructure/clients/openrouter_client.rs`

Add provider preferences to structured output requests to avoid Amazon Bedrock:

```rust
#[derive(Serialize)]
struct ProviderPreferences {
    order: Vec<String>,
    allow_fallbacks: bool,
}
```

Include in `ChatRequest` when `response_format` or `tools` are present:
```json
{"provider": {"order": ["OpenAI", "Anthropic", "Google"], "allow_fallbacks": true}}
```

### 4.3 — EmbeddingClient

**File:** `ai/src/infrastructure/clients/embedding_client.rs` (new)

```rust
pub struct EmbeddingClient { /* api_key, client */ }

impl EmbeddingClient {
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingClientError> {
        // POST https://openrouter.ai/api/v1/embeddings
        // { "model": "openai/text-embedding-3-small", "input": text }
        // Parse response.data[0].embedding → Vec<f32>
    }
}
```

### 4.4 — Config updates

**File:** `ai/src/infrastructure/config/openrouter_config.rs`

Add embedding model config:
```rust
#[arg(long, env = "OPENROUTER_EMBEDDING_MODEL", default_value = "openai/text-embedding-3-small")]
openrouter_embedding_model: String,
```

**File:** `ai/src/infrastructure/config/mod.rs`

Add `KnowledgeServerConfig` (new):
```rust
#[clap(flatten)]
knowledge_server: KnowledgeServerConfig,
```

For AI → Knowledge gRPC client connection.

### 4.5 — Update client module

**File:** `ai/src/infrastructure/clients/mod.rs`

```rust
mod openrouter_client;
mod embedding_client;

pub use openrouter_client::{...};
pub use embedding_client::EmbeddingClient;
```

### 4.6 — Error types

**File:** `ai/src/presentation/errors/` — add error variants for:
- `EmbeddingClientError` — request/response/deserialization errors
- Streaming errors — SSE parse failures, incomplete streams

### Checklist

- [ ] `chat_stream()` method with SSE parsing
- [ ] `ToolDefinition`, `ToolCall`, `AssistantMessage` types
- [ ] Provider routing in all requests
- [ ] `EmbeddingClient` with `embed()` method
- [ ] Config: embedding model, knowledge server URL
- [ ] Error types
- [ ] Unit test: parse a mock SSE stream into chunks
- [ ] `cargo build -p ai` passes

---

## Step 5: AI Service — Agent Loop

**Goal:** Implement `AgentService` — the core agent loop that receives a user message,
calls the model, executes tools, and streams events.

### 5.1 — gRPC clients

**File:** `ai/src/infrastructure/clients/knowledge_client.rs` (new)

A gRPC client wrapping the knowledge service. Implements `GrpcClient` trait from
`bric-a-brac-protos`. Methods mirror the knowledge RPCs:

- `search_nodes(graph_id, node_key?, embedding, limit) → Vec<NodeSummary>`
- `get_node(graph_id, node_data_id) → NodeData`
- `get_neighbors(graph_id, node_data_id, edge_key?, depth) → Subgraph`
- `find_paths(graph_id, from_id, to_id, max_depth) → Vec<Path>`
- `insert_node(graph_id, InsertNodeData) → NodeData`
- `update_node(graph_id, UpdateNodeData) → NodeData`
- `insert_edge(graph_id, InsertEdgeData) → EdgeData`
- `initialize_schema(graph_id, node_keys)`

**File:** `ai/src/infrastructure/clients/metadata_client.rs` (new)

An HTTP client wrapping the metadata service REST API:

- `get_session(session_id) → Session`
- `get_messages(session_id) → Vec<SessionMessage>`
- `append_messages(session_id, Vec<CreateSessionMessage>)`
- `get_schema(graph_id) → GraphSchema`
- `update_session_status(session_id, status)`

### 5.2 — Tool definitions

**File:** `ai/src/application/services/tools.rs` (new)

Defines the tool schemas as `ToolDefinition` structs:

```rust
pub fn read_tools() -> Vec<ToolDefinition> {
    vec![search_nodes_tool(), get_node_tool(), get_neighbors_tool(), find_paths_tool()]
}

pub fn write_tools() -> Vec<ToolDefinition> {
    vec![create_node_tool(), create_edge_tool(), update_node_tool()]
}

pub fn session_tools() -> Vec<ToolDefinition> {
    vec![done_tool()]
}
```

Each tool definition includes name, description, and JSON Schema for parameters.

### 5.3 — System prompt builder

**File:** `ai/src/application/services/prompt.rs` (new)

```rust
pub fn build_system_prompt(schema: &GraphSchemaDto, user_role: &str) -> String {
    // Builds the dynamic system prompt as described in AI_AGENT_DESIGN.md
    // - Identity section
    // - Schema section (human-readable, from GraphSchemaDto)
    // - Capabilities section (based on role)
    // - Behavioral rules
}
```

### 5.4 — Tool executor

**File:** `ai/src/application/services/tool_executor.rs` (new)

```rust
pub struct ToolExecutor {
    knowledge_client: KnowledgeClient,
    embedding_client: EmbeddingClient,
}

impl ToolExecutor {
    pub async fn execute(
        &self,
        tool_call: &ToolCall,
        graph_id: &str,
        session_id: &str,
        user_role: &str,
    ) -> ToolResult {
        match tool_call.name.as_str() {
            "search_nodes" => { /* parse args, embed query, call knowledge */ }
            "get_node" => { /* parse args, call knowledge */ }
            "get_neighbors" => { /* parse args, call knowledge */ }
            "find_paths" => { /* parse args, call knowledge */ }
            "create_node" => {
                if user_role != "write" { return permission_denied(); }
                /* parse args, embed node text, call knowledge.insert_node */
            }
            "create_edge" => {
                if user_role != "write" { return permission_denied(); }
                /* parse args, call knowledge.insert_edge */
            }
            "update_node" => {
                if user_role != "write" { return permission_denied(); }
                /* parse args, re-embed if needed, call knowledge.update_node */
            }
            "done" => { /* return done signal */ }
            _ => { /* unknown tool error */ }
        }
    }
}
```

### 5.5 — AgentService

**File:** `ai/src/application/services/agent_service.rs` (new)

The core agent loop:

```rust
pub struct AgentService {
    openrouter_client: OpenRouterClient,
    metadata_client: MetadataClient,
    tool_executor: ToolExecutor,
}

impl AgentService {
    pub fn send_message(
        &self,
        session_id: String,
        user_message: String,
    ) -> impl Stream<Item = AgentEvent> {
        // 1. Load session from metadata (get graph_id, user_role)
        // 2. Load messages from metadata
        // 3. Load schema from metadata → build system prompt
        // 4. Build tool list based on user_role
        // 5. Append user message to history
        // 6. LOOP:
        //    a. Call openrouter_client.chat_stream(messages, tools)
        //    b. Forward text deltas as AgentText events
        //    c. On complete response:
        //       - If no tool calls → yield AgentDone, break
        //       - If "done" tool called → yield AgentDone, break
        //       - If tool calls → for each: yield AgentToolCall, execute, yield AgentToolResult
        //       - Append assistant message + tool results to history
        //       - Check iteration count (max 50)
        //    d. Go to (a)
        // 7. Persist all new messages to metadata
        // 8. If done, update session status
    }
}
```

The method returns a `tokio_stream::Stream` or `futures::Stream` that the gRPC handler
wraps into a tonic `Streaming` response.

### 5.6 — Update service module

**File:** `ai/src/application/services/mod.rs`

```rust
mod agent_service;
mod data_service;    // keep for now, deprecated
mod prompt;
mod schema_service;
mod tool_executor;
mod tools;

pub use agent_service::AgentService;
pub use data_service::DataService;
pub use schema_service::SchemaService;
```

### Checklist

- [ ] `knowledge_client.rs` — gRPC client for knowledge service
- [ ] `metadata_client.rs` — HTTP client for metadata service
- [ ] `tools.rs` — tool definitions (JSON Schema for each)
- [ ] `prompt.rs` — system prompt builder
- [ ] `tool_executor.rs` — dispatches tool calls to clients
- [ ] `agent_service.rs` — core agent loop with streaming
- [ ] Module exports updated
- [ ] `cargo build -p ai` passes

---

## Step 6: AI Service — SendMessage Handler + Wiring

**Goal:** Wire the `AgentService` into the gRPC server and expose `SendMessage`.

### 6.1 — Update gRPC handler

**File:** `ai/src/presentation/grpc/ai_service.rs`

Add `SendMessage` implementation:

```rust
type SendMessageStream = Pin<Box<dyn Stream<Item = Result<AgentEventProto, Status>> + Send>>;

async fn send_message(
    &self,
    request: Request<SendMessageRequest>,
) -> Result<Response<Self::SendMessageStream>, Status> {
    let req = request.into_inner();
    let stream = self.agent_service.send_message(req.session_id, req.content);
    // Map AgentEvent → AgentEventProto, wrap errors
    Ok(Response::new(Box::pin(mapped_stream)))
}
```

### 6.2 — Update `AiService` struct

**File:** `ai/src/presentation/grpc/ai_service.rs`

Add `agent_service: AgentService` field. Update constructor.

### 6.3 — Update `lib.rs` wiring

**File:** `ai/src/lib.rs`

Wire up all new clients and services:

```rust
pub async fn run(config: Config) -> anyhow::Result<()> {
    let openrouter_client = OpenRouterClient::new(config.openrouter());
    let embedding_client = EmbeddingClient::new(config.openrouter());
    let knowledge_client = KnowledgeClient::new(config.knowledge_server());
    let metadata_client = MetadataClient::new(config.metadata_server());

    let schema_service = SchemaService::new(openrouter_client.clone());
    let data_service = DataService::new(openrouter_client.clone());
    let tool_executor = ToolExecutor::new(knowledge_client, embedding_client);
    let agent_service = AgentService::new(openrouter_client, metadata_client, tool_executor);

    let ai_service = AiService::new(schema_service, data_service, agent_service);
    // ...
}
```

### Checklist

- [ ] `send_message` RPC handler implemented
- [ ] `AiService` struct updated with `AgentService`
- [ ] `lib.rs` wiring updated
- [ ] `cargo build -p ai` passes
- [ ] End-to-end test: create session → send message → receive streaming events

---

## Step 7: Integration Testing

**Goal:** Verify the full pipeline works end-to-end.

### 7.1 — Manual test flow

1. Start all services (metadata, knowledge, ai)
2. Create a graph + schema via metadata API
3. Verify `InitializeSchema` was called (check Memgraph for vector indexes)
4. Create a session via `POST /graphs/:id/sessions`
5. Send a message via `SendMessage` gRPC with a document
6. Observe streaming events (text tokens, tool calls, tool results, done)
7. Verify nodes + edges created in Memgraph with `session_id` and `embedding`
8. Send a question message — verify the agent searches the graph and responds

### 7.2 — Test scenarios

| Scenario | What to verify |
|---|---|
| Document ingestion | Nodes + edges created, embeddings stored, session_id set |
| Duplicate detection | Send same document twice — second run finds existing nodes via search |
| Question answering | Agent calls search_nodes, reasons, responds in text |
| Read-only user | Write tools return permission denied, agent communicates it |
| Schema mismatch | Agent reports unmapped data in done summary |
| Empty graph | Agent handles no search results gracefully |
| 50 tool limit | Agent stops after 50 calls, streams error |

### Checklist

- [ ] All services start and connect
- [ ] Vector indexes created on schema creation
- [ ] Document ingestion produces correct graph data
- [ ] Search finds nodes by semantic similarity
- [ ] Question answering works
- [ ] Read-only enforcement works
- [ ] Schema mismatch reported in summary

---

## Step 8: Web UI

**Goal:** Replace the "generate data from file" flow with a chat interface.

### 8.1 — Session management

- Create session when user opens a graph's agent tab
- Store session ID in React context
- Close session on tab close or explicit action

### 8.2 — Chat interface

- Message input with send button
- File attachment (reads file as text, includes in message)
- Streaming display: show events as they arrive
  - `AgentText` → append to current assistant message bubble
  - `AgentToolCall` → show collapsible "calling search_nodes..." indicator
  - `AgentToolResult` → show collapsible result under the tool call
  - `AgentDone` → show summary card with node/edge counts
  - `AgentError` → show error banner

### 8.3 — gRPC-Web or REST bridge

The web UI can't call gRPC directly. Options:
- **gRPC-Web** via Envoy or tonic-web (add `tonic-web` layer to the AI server)
- **REST bridge** in metadata service that proxies to AI gRPC

Recommended: add `tonic-web` to the AI server — minimal change, built-in support.

### Checklist

- [ ] Session creation/management
- [ ] Chat UI with message input
- [ ] Streaming event display
- [ ] File attachment support
- [ ] gRPC-Web connectivity
- [ ] Schema generation flow still works

---

## Dependencies Between Steps

```
Step 1 (Protos) ─────┬──→ Step 2 (Knowledge)
                      ├──→ Step 3 (Metadata)
                      └──→ Step 4 (AI: clients)

Step 2 ──┐
Step 3 ──┼──→ Step 5 (AI: agent loop) ──→ Step 6 (AI: handler) ──→ Step 7 (Testing)
Step 4 ──┘

Step 7 ──→ Step 8 (Web UI)
```

Steps 2, 3, and 4 can be done **in parallel** after Step 1.
Step 5 needs all three.

---

## Files Created (New)

| File | Service |
|---|---|
| `metadata/migrations/YYYYMMDDHHMMSS_sessions.up.sql` | metadata |
| `metadata/migrations/YYYYMMDDHHMMSS_sessions.down.sql` | metadata |
| `metadata/src/domain/models/session_model.rs` | metadata |
| `metadata/src/domain/models/session_message_model.rs` | metadata |
| `metadata/src/infrastructure/repositories/session_repository.rs` | metadata |
| `metadata/src/application/services/session_service.rs` | metadata |
| `metadata/src/application/dtos/session_dto.rs` | metadata |
| `ai/src/infrastructure/clients/embedding_client.rs` | ai |
| `ai/src/infrastructure/clients/knowledge_client.rs` | ai |
| `ai/src/infrastructure/clients/metadata_client.rs` | ai |
| `ai/src/infrastructure/config/knowledge_server_config.rs` | ai |
| `ai/src/application/services/agent_service.rs` | ai |
| `ai/src/application/services/tools.rs` | ai |
| `ai/src/application/services/prompt.rs` | ai |
| `ai/src/application/services/tool_executor.rs` | ai |

## Files Modified

| File | Change |
|---|---|
| `crates/bric-a-brac-protos/protos/common.proto` | New message types |
| `crates/bric-a-brac-protos/protos/ai.proto` | `SendMessage` RPC + event messages |
| `crates/bric-a-brac-protos/protos/knowledge.proto` | 8 new RPCs + messages |
| `knowledge/src/infrastructure/repositories/mutate_repository.rs` | 3 new methods |
| `knowledge/src/infrastructure/repositories/query_repository.rs` | 4 new methods |
| `knowledge/src/application/services/mutate_service.rs` | 4 new methods |
| `knowledge/src/application/services/query_service.rs` | 4 new methods |
| `knowledge/src/presentation/grpc/knowledge_service.rs` | 8 new RPC handlers |
| `knowledge/src/domain/models/mod.rs` | New model exports |
| `metadata/src/application/services/graph_service.rs` | `create_schema` hook |
| `metadata/src/infrastructure/clients/knowledge_client.rs` | `initialize_schema()` |
| `metadata/src/domain/models/mod.rs` | New model exports |
| `ai/src/infrastructure/clients/openrouter_client.rs` | SSE streaming + tool calling + provider routing |
| `ai/src/infrastructure/clients/mod.rs` | New client exports |
| `ai/src/infrastructure/config/mod.rs` | Knowledge server config |
| `ai/src/infrastructure/config/openrouter_config.rs` | Embedding model config |
| `ai/src/application/services/mod.rs` | New service exports |
| `ai/src/presentation/grpc/ai_service.rs` | `SendMessage` handler |
| `ai/src/presentation/errors/` | New error variants |
| `ai/src/lib.rs` | Wiring for new clients and services |
