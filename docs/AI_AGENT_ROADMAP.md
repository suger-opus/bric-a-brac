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
    rpc LoadGraph(LoadGraphRequest) returns (common.GraphDataProto);
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
    rpc SendMessage(SendMessageRequest) returns (stream AgentEventProto);
}
// NOTE: GenerateSchema has been removed — the AI creates types via agent tools.
// Proto cleanup (removing GenerateSchema RPC, GenerateSchemaRequest, CreateGraphSchemaProto,
// all property schema protos) will be done as part of Step 1b (see below).

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

## Step 2: Knowledge Service — New Handlers ✅

**Status:** COMPLETED

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

- [x] `mutate_repository.rs` — `insert_node`, `update_node`, `insert_edge`, `initialize_schema`
- [x] `query_repository.rs` — `get_node`, `search_nodes`, `get_neighbors`, `find_paths`
- [x] Domain models merged into existing files (no separate `agent_models.rs`) + DTO conversions follow Proto→Dto→Model pattern
- [x] `mutate_service.rs` — 4 new methods
- [x] `query_service.rs` — 4 new methods
- [x] `knowledge_service.rs` — 8 new RPC handlers (replaced all `todo!()` stubs)
- [x] All existing tests still pass
- [x] `cargo build -p knowledge -p ai -p metadata` passes (zero warnings)

**Extra changes:**
- `extend_element.rs` — `collect_properties()` now filters `embedding` and `session_id` in addition to existing system fields
- `edge_data_model.rs` — added `TryFrom<UnboundedRelation>` for path parsing in `find_paths`
- `extend_element.rs` — added `ExtendElement` impl for `neo4rs::UnboundedRelation`
- `app_error.rs` — added `AppError::NotFound`, `AppError::InvalidInput`, `DatabaseError::InvalidDepth`
- `tonic_error.rs` — proper gRPC status code mapping (`NotFound` → `NOT_FOUND`, `InvalidInput`/`Conversion` → `INVALID_ARGUMENT`)
- `initialize_schema` uses `graph.run()` (auto-commit) instead of transactions for DDL; index creation errors are logged but don't fail (idempotent)
- Proto→Dto→Model conversion pattern enforced: validation happens at the Dto layer (in `bric-a-brac-dtos` crate), model conversions are infallible `From` impls
- `InsertNodeDataDto`, `InsertEdgeDataDto`, `UpdateNodeDataDto` created in `bric-a-brac-dtos` crate with `#[validate(nested)]` on `KeyDto`
- `validator` dependency added to knowledge crate for key validation in gRPC handlers

---

## Step 2b: Schema Simplification — Remove Property Schemas + GenerateSchema ✅

**Status:** COMPLETED

**Goal:** Simplify schemas to lightweight types (key + name + description + color). Remove
property schemas, GenerateSchema RPC, SchemaService, and all schema creation endpoints.
Only the read endpoint (`GET /graphs/:id/schema`) is kept.

### What was done

**Proto cleanup:**
- `common.proto` — removed 7 messages: `PropertySchemaProto`, `CreateGraphSchemaProto`,
  `CreateNodeSchemaProto`, `CreateEdgeSchemaProto`, `CreatePropertySchemaProto`,
  `PropertyTypeProto` enum, `PropertyMetadataProto`. Changed `NodeSchemaProto` and
  `EdgeSchemaProto`: replaced `repeated PropertySchemaProto properties` with `string description`.
- `ai.proto` — removed `GenerateSchema` RPC, `GenerateSchemaRequest`, `FileTypeProto` enum.
  Service now has only `rpc SendMessage`.

**DTO cleanup:**
- Deleted `property_schema_dto.rs` and `openapi.rs` (structured output doc for GenerateSchema).
- Simplified `node_schema_dto.rs`, `edge_schema_dto.rs` — removed properties, removed
  `CreateNodeSchemaDto`/`CreateEdgeSchemaDto`, added `description: String`.
- Simplified `graph_schema_dto.rs` — removed `CreateGraphSchemaDto`.
- Updated `mod.rs` and `lib.rs` — removed all property/create schema exports.

**Migration:** Edited the existing single migration (no new migration file):
- `setup.up.sql` — removed `property_type` enum, removed `properties_schemas` table,
  added `description TEXT NOT NULL DEFAULT ''` to `nodes_schemas` and `edges_schemas`.
- `setup.down.sql` — removed corresponding drops.

**Metadata service:**
- Deleted: `property_schema_model.rs`, `property_schema_dto.rs` (app layer),
  `ai_client.rs`, `ai_server_config.rs`.
- Domain models: removed `CreateNodeSchemaModel`, `CreateEdgeSchemaModel`,
  `CreateGraphSchemaModel`, property references. Added `description` field.
- Repository: rewrote `get_schema()` as two simple SELECTs (no JOIN to properties_schemas).
  Removed `create_nodes_schemas()`, `create_edges_schemas()`, `create_properties()`,
  `PropertySchemaRow`, `SchemaRow`, and all `TryFrom` impls. Added `description` to row structs.
- Service: removed `create_schema()`, `generate_schema()`, `AiClient` dependency.
- App DTOs: removed all Create*→Model conversions, property mappings. Added description mapping.
- Handlers: removed `create_schema` and `generate_schema` handlers.
- Router: removed POST `/schema` and POST `/schema/generate` routes.
- OpenAPI: removed corresponding path entries.
- State: removed `AiClient` construction.
- Config: removed `AiServerConfig` and `ai_server` field.
- Extractors: removed `MultipartFileUpload` (only used by generate_schema).

**AI service:**
- Deleted `schema_service.rs`.
- Emptied `application/services/mod.rs`.
- `ai_service.rs` — removed `SchemaService` field, `generate_schema` handler. `AiService` is
  now a unit struct with only `SendMessage` stub.
- `lib.rs` — removed `SchemaService` and `OpenRouterClient` wiring (will be re-added in Step 6).

### Checklist

- [x] Proto cleanup: removed property schema messages, GenerateSchema RPC, FileTypeProto
- [x] DTO cleanup: deleted `property_schema_dto.rs` + `openapi.rs`, simplified node/edge/graph schema DTOs
- [x] Migration edited in-place (single migration, dropped property_type enum + properties_schemas table, added description)
- [x] Metadata code: removed property schema models, repos, services, handlers, routes, AiClient, AiServerConfig, MultipartFileUpload
- [x] AI service: deleted `SchemaService`, removed `GenerateSchema` handler, simplified wiring
- [x] Web UI: not changed (deferred — user will do it separately)
- [x] `cargo build` passes (zero warnings)
- [x] `cargo test` passes (13 tests)
- [x] `cargo sqlx prepare` regenerated

---

## Step 3: Metadata Service — Sessions + Type Management

**Goal:** Add session management, simplified type CRUD (for AI agent to create types), and
the hook that initializes vector indexes when types are created.

### 3.1 — Database migration (sessions)

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

### 3.6 — Type CRUD endpoints

The AI agent creates types mid-conversation. These endpoints let it (through the AI service's
`MetadataClient`) create and list types per graph.

**File:** `metadata/src/application/services/graph_service.rs` (or new `type_service.rs`)

Methods:
- `create_node_type(graph_id, name, description, color) → NodeSchemaDto`
  - Generates key (8-char alphanumeric)
  - Inserts into `node_schemas`
  - Calls `knowledge_client.initialize_schema(graph_id, [key])` to create vector index
  - Returns the created type with key
- `create_edge_type(graph_id, name, description, color) → EdgeSchemaDto`
  - Generates key
  - Inserts into `edge_schemas`
  - Returns the created type with key
- `get_types(graph_id) → GraphSchemaDto`
  - Returns all node types + edge types for a graph (lightweight: key, name, description, color)

**File:** `metadata/src/presentation/` — new type routes

| Route | Handler |
|---|---|
| `POST /graphs/:graph_id/node-types` | Create node type (body: `{ name, description, color }`) |
| `POST /graphs/:graph_id/edge-types` | Create edge type (body: `{ name, description, color }`) |
| `GET /graphs/:graph_id/types` | List all types for a graph |

### 3.7 — InitializeSchema hook

**File:** `metadata/src/infrastructure/clients/knowledge_client.rs`

Add the `initialize_schema` method. Called in two places:
1. When `create_schema()` creates node types from the web UI
2. When `create_node_type()` creates a single type (from AI agent tool)

```rust
pub async fn initialize_schema(&self, graph_id: &str, node_keys: &[String]) -> Result<()> {
    // calls knowledge gRPC: InitializeSchema { graph_id, node_keys }
}
```

### 3.8 — DTOs

**File:** `metadata/src/application/dtos/` — new session DTOs

- `SessionDto`, `CreateSessionDto`, `SessionMessageDto`, `CreateSessionMessageDto`

Type DTOs already exist (`NodeSchemaDto`, `EdgeSchemaDto`, `GraphSchemaDto`) — they will be
simplified in Step 2b to remove property schema fields and add `description`.

### Checklist

- [ ] Migration — `sessions` + `session_messages` tables
- [ ] Domain models — `SessionModel`, `SessionMessageModel`
- [ ] `session_repository.rs` with all CRUD methods
- [ ] `session_service.rs` with business logic
- [ ] HTTP handlers for session CRUD + message management
- [ ] DTOs for session data
- [ ] Type CRUD: `create_node_type`, `create_edge_type`, `get_types`
- [ ] Type HTTP handlers: `POST /node-types`, `POST /edge-types`, `GET /types`
- [ ] `KnowledgeClient::initialize_schema()` — called on node type creation
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
- `get_types(graph_id) → GraphSchemaDto` — lightweight types (key, name, description, color)
- `create_node_type(graph_id, name, description) → NodeSchemaDto`
- `create_edge_type(graph_id, name, description) → EdgeSchemaDto`
- `update_session_status(session_id, status)`

### 5.2 — Tool definitions

**File:** `ai/src/application/services/tools.rs` (new)

Defines the tool schemas as `ToolDefinition` structs:

```rust
pub fn read_tools() -> Vec<ToolDefinition> {
    vec![search_nodes_tool(), get_node_tool(), get_neighbors_tool(), find_paths_tool()]
}

pub fn write_tools() -> Vec<ToolDefinition> {
    vec![
        create_type_tool(),
        create_edge_type_tool(),
        create_node_tool(),
        create_edge_tool(),
        update_node_tool(),
    ]
}

pub fn session_tools() -> Vec<ToolDefinition> {
    vec![done_tool()]
}
```

Each tool definition includes name, description, and JSON Schema for parameters.

**New tools:**
- `create_type` — params: `name` (string), `description` (string) → creates a node type
- `create_edge_type` — params: `name` (string), `description` (string) → creates an edge type

### 5.3 — System prompt builder

**File:** `ai/src/application/services/prompt.rs` (new)

```rust
pub fn build_system_prompt(types: &GraphSchemaDto, user_role: &str) -> String {
    // Builds the dynamic system prompt as described in AI_AGENT_DESIGN.md
    // - Identity section
    // - Types section: human-readable listing of node types + edge types
    //   (name, key, description — no properties)
    // - Capabilities section (based on role)
    // - Behavioral rules (type reuse, entity resolution awareness, normalization)
}
```

Types are injected fresh each message (not once per session). Format:

```
Node types:
- Person (key: ESVhRs9k): "Any human individual"
- Company (key: dudFcexv): "Organizations, corporations"

Edge types:
- WorksAt (key: xR4kLm2p): "Employment or affiliation"
```

### 5.4 — Tool executor

**File:** `ai/src/application/services/tool_executor.rs` (new)

```rust
pub struct ToolExecutor {
    knowledge_client: KnowledgeClient,
    metadata_client: MetadataClient,
    embedding_client: EmbeddingClient,
}

impl ToolExecutor {
    pub async fn execute(
        &self,
        tool_call: &ToolCall,
        graph_id: &str,
        session_id: &str,
        user_role: &str,
        types: &GraphSchemaDto,
    ) -> ToolResult {
        match tool_call.name.as_str() {
            "search_nodes" => { /* parse args, embed query, call knowledge */ }
            "get_node" => { /* parse args, call knowledge */ }
            "get_neighbors" => { /* parse args, call knowledge */ }
            "find_paths" => { /* parse args, call knowledge */ }
            "create_type" => {
                if user_role != "write" { return permission_denied(); }
                /* call metadata_client.create_node_type(graph_id, name, description) */
                /* metadata creates type + calls knowledge.initialize_schema for vector index */
                /* return type with key to LLM */
            }
            "create_edge_type" => {
                if user_role != "write" { return permission_denied(); }
                /* call metadata_client.create_edge_type(graph_id, name, description) */
                /* return type with key to LLM */
            }
            "create_node" => {
                if user_role != "write" { return permission_denied(); }
                self.validate_type_exists(types, &node_type)?;
                /* 1. serialize props → embed */
                /* 2. search_nodes with embedding (entity resolution) */
                /* 3. if similar nodes found: */
                /*    - get_neighbors for each candidate */
                /*    - create the node anyway */
                /*    - return node + similarity warnings to LLM */
                /*    - LLM decides: update_node to merge, or keep both */
                /* 4. if no similar nodes: create directly */
            }
            "create_edge" => {
                if user_role != "write" { return permission_denied(); }
                self.validate_type_exists(types, &edge_type)?;
                /* parse args, call knowledge.insert_edge */
            }
            "update_node" => {
                if user_role != "write" { return permission_denied(); }
                /* parse args, re-embed from updated props, call knowledge.update_node */
            }
            "done" => { /* return done signal */ }
            _ => { /* unknown tool error */ }
        }
    }
}
```

**Type validation (replaces old schema validation):** The tool executor only checks that the
`node_type` / `edge_type` key exists in the `types` (loaded from metadata). No property
validation — properties are free-form.

On failure, returns a descriptive error as the `ToolResult` (not a gRPC error):

    "Unknown node type 'xInvalid'. Valid types: ESVhRs9k (Person), dudFcexv (Company)."

**Entity resolution (built into `create_node`):** Automatic similarity search + neighbor
context is **structural**, not prompt-dependent. The executor embeds, searches, fetches
neighbors for candidates, creates the node, and returns everything to the LLM. The LLM
then decides merge or keep. See AI_AGENT_DESIGN.md for the full flow.

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
        // 3. Load types from metadata (get_types) → build system prompt
        // 4. Build tool list based on user_role
        //    (write: read_tools + write_tools + session_tools)
        //    (read: read_tools + session_tools)
        // 5. Append user message to history
        // 6. LOOP:
        //    a. Call openrouter_client.chat_stream(messages, tools)
        //    b. Forward text deltas as AgentText events
        //    c. On complete response:
        //       - If no tool calls → yield AgentDone, break
        //       - If "done" tool called → yield AgentDone, break
        //       - If tool calls → for each: yield AgentToolCall, execute, yield AgentToolResult
        //       - Append assistant message + tool results to history
        //       - If create_type/create_edge_type was called → refresh types from metadata
        //       - Check iteration count (max 50)
        //    d. Go to (a)
        // 7. Persist all new messages to metadata
        // 8. If done, update session status
    }
}
```

**Important:** When the AI creates a type during the loop (via `create_type` or
`create_edge_type`), the `types` object must be refreshed from metadata before the next
iteration so the system prompt and type validation reflect the new type.

The method returns a `tokio_stream::Stream` or `futures::Stream` that the gRPC handler
wraps into a tonic `Streaming` response.

### 5.6 — Update service module

**File:** `ai/src/application/services/mod.rs`

```rust
mod agent_service;
mod prompt;
mod tool_executor;
mod tools;

pub use agent_service::AgentService;
```

No `SchemaService` — schema generation has been removed.

### Checklist

- [ ] `knowledge_client.rs` — gRPC client for knowledge service
- [ ] `metadata_client.rs` — HTTP client for metadata service (types + sessions)
- [ ] `tools.rs` — tool definitions including `create_type` and `create_edge_type`
- [ ] `prompt.rs` — system prompt builder (types, not property schemas)
- [ ] `tool_executor.rs` — dispatches tool calls, entity resolution in `create_node`, type-only validation
- [ ] `agent_service.rs` — core agent loop with streaming, type refresh on create
- [ ] Module exports updated (no `SchemaService`)
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

    let tool_executor = ToolExecutor::new(knowledge_client, metadata_client.clone(), embedding_client);
    let agent_service = AgentService::new(openrouter_client, metadata_client, tool_executor);

    let ai_service = AiService::new(agent_service);
    // ...
}
```

No `SchemaService` — schema generation has been removed (Step 2b).

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
2. Create a graph via metadata API
3. Create a session via `POST /graphs/:id/sessions`
4. Send a message via `SendMessage` gRPC with a document
5. Observe streaming events (text tokens, tool calls for type creation, more tool calls for entity extraction, done)
6. Verify types created in metadata (`GET /graphs/:id/types`)
7. Verify vector indexes created in Memgraph
8. Verify nodes + edges created in Memgraph with `session_id` and `embedding`
9. Send a question message — verify the agent searches the graph and responds
10. Send more data — verify entity resolution detects duplicates

### 7.2 — Test scenarios

| Scenario | What to verify |
|---|---|
| Document ingestion | AI creates types, then nodes + edges. Embeddings stored, session_id set |
| Type creation | AI creates types only when needed, reuses existing ones |
| Entity resolution | AI receives duplicate warnings on similar nodes, merges correctly |
| Question answering | Agent calls search_nodes, reasons, responds in text |
| Read-only user | Write tools return permission denied, agent communicates it |
| Unknown type | Agent reports error, gets valid types list, self-corrects |
| Empty graph | Agent handles no search results gracefully, creates types from scratch |
| 50 tool limit | Agent stops after 50 calls, streams error |

### Checklist

- [ ] All services start and connect
- [ ] AI creates types on first document ingestion
- [ ] Vector indexes created when node types are created
- [ ] Document ingestion produces correct graph data
- [ ] Entity resolution flags duplicates on second ingestion
- [ ] Search finds nodes by semantic similarity
- [ ] Question answering works
- [ ] Read-only enforcement works

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
- [ ] Type list displayed in sidebar/panel

---

## Dependencies Between Steps

```
Step 1 (Protos) ✅ ───┬──→ Step 2 (Knowledge) ✅
                      ├──→ Step 2b (Schema Simplification) ✅
                      ├──→ Step 3 (Metadata)
                      └──→ Step 4 (AI: clients)

Step 2  ──┐
Step 2b ──┤
Step 3  ──┼──→ Step 5 (AI: agent loop) ──→ Step 6 (AI: handler) ──→ Step 7 (Testing)
Step 4  ──┘

Step 7 ──→ Step 8 (Web UI)
```

Steps 2, 3, and 4 can be done **in parallel** after Step 1.
Step 2b is COMPLETED (prerequisite for Step 3 — types are now simplified).
Step 5 needs all four.

---

## Files Created (New)

| File | Service | Step |
|---|---|---|
| `metadata/migrations/YYYYMMDDHHMMSS_sessions.up.sql` | metadata | 3 |
| `metadata/migrations/YYYYMMDDHHMMSS_sessions.down.sql` | metadata | 3 |
| `metadata/src/domain/models/session_model.rs` | metadata | 3 |
| `metadata/src/domain/models/session_message_model.rs` | metadata | 3 |
| `metadata/src/infrastructure/repositories/session_repository.rs` | metadata | 3 |
| `metadata/src/application/services/session_service.rs` | metadata | 3 |
| `metadata/src/application/dtos/session_dto.rs` | metadata | 3 |
| `ai/src/infrastructure/clients/embedding_client.rs` | ai | 4 |
| `ai/src/infrastructure/clients/knowledge_client.rs` | ai | 5 |
| `ai/src/infrastructure/clients/metadata_client.rs` | ai | 5 |
| `ai/src/infrastructure/config/knowledge_server_config.rs` | ai | 4 |
| `ai/src/application/services/agent_service.rs` | ai | 5 |
| `ai/src/application/services/tools.rs` | ai | 5 |
| `ai/src/application/services/prompt.rs` | ai | 5 |
| `ai/src/application/services/tool_executor.rs` | ai | 5 |

## Files Modified

| File | Change | Step |
|---|---|---|
| `crates/bric-a-brac-protos/protos/common.proto` | New message types; remove property schema protos, add description to type protos | 1, 2b |
| `crates/bric-a-brac-protos/protos/ai.proto` | `SendMessage` RPC; remove `GenerateSchema` RPC | 1, 2b |
| `crates/bric-a-brac-protos/protos/knowledge.proto` | 8 new RPCs + messages | 1 |
| `crates/bric-a-brac-dtos/src/dtos/node_schema_dto.rs` | Remove properties + Create type, add description | 2b |
| `crates/bric-a-brac-dtos/src/dtos/edge_schema_dto.rs` | Remove properties + Create type, add description | 2b |
| `crates/bric-a-brac-dtos/src/dtos/graph_schema_dto.rs` | Remove CreateGraphSchemaDto | 2b |
| `crates/bric-a-brac-dtos/src/dtos/mod.rs` | Remove property schema exports | 2b |
| `crates/bric-a-brac-dtos/src/lib.rs` | Remove openapi module | 2b |
| `metadata/migrations/20260130112820_setup.up.sql` | Remove property_type enum + properties_schemas, add description | 2b |
| `metadata/migrations/20260130112820_setup.down.sql` | Remove property_schemas + property_type drops | 2b |
| `metadata/src/domain/models/mod.rs` | Remove property_schema_model, Create* exports | 2b |
| `metadata/src/domain/models/node_schema_model.rs` | Remove properties + CreateNodeSchemaModel, add description | 2b |
| `metadata/src/domain/models/edge_schema_model.rs` | Remove properties + CreateEdgeSchemaModel, add description | 2b |
| `metadata/src/domain/models/graph_schema_model.rs` | Remove CreateGraphSchemaModel | 2b |
| `metadata/src/infrastructure/repositories/graph_repository.rs` | Rewrite get_schema (simple SELECTs), remove create_*_schemas, remove property rows/TryFrom | 2b |
| `metadata/src/infrastructure/clients/mod.rs` | Remove AiClient export | 2b |
| `metadata/src/infrastructure/config/mod.rs` | Remove AiServerConfig | 2b |
| `metadata/src/application/services/graph_service.rs` | Remove create_schema, generate_schema, AiClient | 2b |
| `metadata/src/application/dtos/mod.rs` | Remove property_schema_dto | 2b |
| `metadata/src/application/dtos/node_schema_dto.rs` | Remove Create conversion + properties, add description | 2b |
| `metadata/src/application/dtos/edge_schema_dto.rs` | Remove Create conversion + properties, add description | 2b |
| `metadata/src/application/dtos/graph_schema_dto.rs` | Remove CreateGraphSchemaDto conversion | 2b |
| `metadata/src/presentation/extractors.rs` | Remove MultipartFileUpload | 2b |
| `knowledge/src/infrastructure/repositories/mutate_repository.rs` | 3 new methods | 2 |
| `knowledge/src/infrastructure/repositories/query_repository.rs` | 4 new methods | 2 |
| `knowledge/src/application/services/mutate_service.rs` | 4 new methods | 2 |
| `knowledge/src/application/services/query_service.rs` | 4 new methods | 2 |
| `knowledge/src/presentation/grpc/knowledge_service.rs` | 8 new RPC handlers | 2 |
| `knowledge/src/domain/models/mod.rs` | New model exports | 2 |
| `metadata/src/application/services/graph_service.rs` | Type CRUD methods, `initialize_schema` hook | 3 |
| `metadata/src/infrastructure/clients/knowledge_client.rs` | `initialize_schema()` method | 3 |
| `metadata/src/domain/models/mod.rs` | New model exports | 3 |
| `ai/src/infrastructure/clients/openrouter_client.rs` | SSE streaming + tool calling + provider routing | 4 |
| `ai/src/infrastructure/clients/mod.rs` | New client exports | 4 |
| `ai/src/infrastructure/config/mod.rs` | Knowledge + metadata server config | 4 |
| `ai/src/infrastructure/config/openrouter_config.rs` | Embedding model config | 4 |
| `ai/src/application/services/mod.rs` | Emptied (SchemaService removed); new exports in Step 5 | 2b, 5 |
| `ai/src/presentation/grpc/ai_service.rs` | Removed GenerateSchema + SchemaService; `SendMessage` in Step 6 | 2b, 6 |
| `ai/src/presentation/errors/` | New error variants | 4 |
| `ai/src/lib.rs` | Removed SchemaService wiring; new wiring in Step 6 | 2b, 6 |

## Files Deleted

| File | Service | Step |
|---|---|---|
| `crates/bric-a-brac-dtos/src/dtos/property_schema_dto.rs` | dtos | 2b |
| `crates/bric-a-brac-dtos/src/openapi.rs` | dtos | 2b |
| `metadata/src/domain/models/property_schema_model.rs` | metadata | 2b |
| `metadata/src/application/dtos/property_schema_dto.rs` | metadata | 2b |
| `metadata/src/infrastructure/clients/ai_client.rs` | metadata | 2b |
| `metadata/src/infrastructure/config/ai_server_config.rs` | metadata | 2b |
| `ai/src/application/services/schema_service.rs` | ai | 2b |
