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

## Step 3: Metadata Service — Sessions + Schema Management + gRPC Server ✅

**Status:** COMPLETED

**Goal:** Add session management, schema CRUD (for AI agent to create schemas), a gRPC server
for AI→metadata communication, and the hook that initializes vector indexes in knowledge.

**Key decisions (from brainstorming):**
- Edited the existing single migration file (not a new migration) — user drops docker volumes to reset
- No HTTP endpoints for session or schema CRUD — **all AI→metadata communication uses gRPC** via a new `metadata.proto`
- No `user_role` on sessions (access/role already handled by the `accesses` table)
- One active session per graph — repository refuses creation if one already exists
- Consistent "schema" naming (not "type")

### 3.1 — Database migration (sessions)

**File:** `metadata/migrations/20260130112820_setup.up.sql` (edited in-place)

Added two tables to the existing migration:

```sql
CREATE TABLE sessions (
    session_id          UUID PRIMARY KEY                NOT NULL,
    graph_id            UUID NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    user_id             UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    status              VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT session_status_check CHECK (status IN ('active', 'completed', 'error'))
);

CREATE INDEX idx_sessions_graph_id ON sessions(graph_id);

CREATE TABLE session_messages (
    message_id          UUID PRIMARY KEY                NOT NULL,
    session_id          UUID NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    position            INTEGER NOT NULL,
    role                VARCHAR(20) NOT NULL,
    content             TEXT NOT NULL DEFAULT '',
    tool_calls          JSONB,
    tool_call_id        VARCHAR,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT message_role_check CHECK (role IN ('system', 'user', 'assistant', 'tool')),
    CONSTRAINT unique_session_position UNIQUE (session_id, position)
);
```

**File:** `metadata/migrations/20260130112820_setup.down.sql` — added corresponding drops.

### 3.2 — New proto: `metadata.proto`

**File:** `crates/bric-a-brac-protos/protos/metadata.proto` (new)

The AI service communicates with metadata **exclusively via gRPC** (not HTTP). This proto
defines the contract:

```protobuf
service Metadata {
    rpc CreateSession(CreateSessionRequest) returns (SessionProto);
    rpc GetSession(GetSessionRequest) returns (SessionProto);
    rpc CloseSession(CloseSessionRequest) returns (SessionProto);
    rpc GetSessionMessages(GetSessionMessagesRequest) returns (GetSessionMessagesResponse);
    rpc AppendSessionMessages(AppendSessionMessagesRequest) returns (AppendSessionMessagesResponse);
    rpc CreateNodeSchema(CreateNodeSchemaRequest) returns (common.NodeSchemaProto);
    rpc CreateEdgeSchema(CreateEdgeSchemaRequest) returns (common.EdgeSchemaProto);
    rpc GetSchema(GetSchemaRequest) returns (common.GraphSchemaProto);
}
```

8 RPCs: 5 session management + 3 schema management. Imports `common.proto` for schema protos.

**File:** `crates/bric-a-brac-protos/build.rs` — updated to compile `metadata.proto` in the
second pass (alongside common, ai, knowledge).

**File:** `crates/bric-a-brac-protos/src/lib.rs` — added `pub mod metadata`.

### 3.3 — Domain models

**File:** `metadata/src/domain/models/session_model.rs` (new)
- `SessionIdModel` (via `id!` macro), `SessionStatusModel` (enum: Active/Completed/Error with Display/FromStr/sqlx::Type)
- `SessionModel` (session_id, graph_id, user_id, status, created_at, updated_at)
- `CreateSessionModel` (session_id, graph_id, user_id)

**File:** `metadata/src/domain/models/session_message_model.rs` (new)
- `SessionMessageIdModel`, `SessionMessageRoleModel` (enum: System/User/Assistant/Tool)
- `SessionMessageModel` (message_id, session_id, position, role, content, tool_calls JSONB, tool_call_id, created_at)
- `CreateSessionMessageModel`

**File:** `metadata/src/domain/models/node_schema_model.rs` — added `CreateNodeSchemaModel`
**File:** `metadata/src/domain/models/edge_schema_model.rs` — added `CreateEdgeSchemaModel`
**File:** `metadata/src/domain/models/mod.rs` — updated exports

### 3.4 — Repository

**File:** `metadata/src/infrastructure/repositories/session_repository.rs` (new)

Methods:
- `has_active_session(connection, graph_id) → bool` — checks for active session on graph
- `create_session(connection, CreateSessionModel) → SessionModel`
- `get_session(connection, session_id) → SessionModel`
- `close_session(connection, session_id, status) → SessionModel`
- `get_messages(connection, session_id) → Vec<SessionMessageModel>` (ordered by position ASC)
- `get_max_position(connection, session_id) → i32`
- `append_messages(connection, Vec<CreateSessionMessageModel>) → Vec<SessionMessageModel>`

Row types: `SessionRow`, `SessionMessageRow` with `From` impls.

**File:** `metadata/src/infrastructure/repositories/graph_repository.rs` — added
`create_node_schema(connection, CreateNodeSchemaModel) → NodeSchemaModel` and
`create_edge_schema(connection, CreateEdgeSchemaModel) → EdgeSchemaModel`

### 3.5 — Application service

**File:** `metadata/src/application/services/session_service.rs` (new)

Methods:
- `create_session(CreateSessionModel) → SessionModel` — **checks `has_active_session` first**, refuses if active exists (returns `DatabaseError::UnexpectedState`)
- `get_session(session_id) → SessionModel`
- `close_session(session_id, status) → SessionModel`
- `get_messages(session_id) → Vec<SessionMessageModel>`
- `append_messages(session_id, Vec<CreateSessionMessageModel>)` — gets max_position, renumbers messages starting from max_pos+1

**File:** `metadata/src/application/services/graph_service.rs` — added:
- `create_node_schema(graph_id, label, description) → NodeSchemaModel` — generates key (8-char alphanumeric) and color (random hex), inserts in DB, calls `knowledge_client.initialize_schema()`
- `create_edge_schema(graph_id, label, description) → EdgeSchemaModel` — generates key/color, inserts in DB
- Helper functions: `generate_key()` → `^[a-zA-Z][a-zA-Z0-9]{7}$`, `generate_color()` → `#RRGGBB`

### 3.6 — DTOs (Proto ↔ Model conversions)

**File:** `metadata/src/application/dtos/session_dto.rs` (new)

- `From<SessionModel> for SessionProto`
- `From<SessionMessageModel> for SessionMessageProto`
- `create_session_from_proto(graph_id, user_id) → CreateSessionModel`
- `create_messages_from_proto(session_id, start_position, messages) → Vec<CreateSessionMessageModel>`

Uses `bric_a_brac_dtos::utils::ProtoTimestampExt` for timestamp conversion (required making
`utils` module public in the dtos crate).

### 3.7 — gRPC server

**File:** `metadata/src/presentation/grpc/metadata_service.rs` (new)

`MetadataGrpcService` struct with `session_service` and `graph_service` fields.
Implements all 8 Metadata RPCs. Error handling: parse errors → `Status::invalid_argument`,
service errors → `Status::internal`.

**File:** `metadata/src/lib.rs` — now runs **both HTTP and gRPC servers in parallel** via
`tokio::select!`. Creates `MetadataGrpcService` from state's services. Uses
`build_grpc_server(MetadataServer::new(grpc_service), grpc_addr)`.

**File:** `metadata/src/presentation/state.rs` — `ApiState` now includes `SessionService`
alongside existing services.

**File:** `metadata/src/infrastructure/config/metadata_server_config.rs` — added
`metadata_grpc_server_port: u16` field with env var `METADATA_GRPC_SERVER_PORT` and
`grpc_url()` method.

**File:** `metadata/mise.local.toml` — added `METADATA_GRPC_SERVER_PORT="50052"`.

### 3.8 — InitializeSchema hook

**File:** `metadata/src/infrastructure/clients/knowledge_client.rs` — added
`initialize_schema(graph_id, node_keys)` and `try_initialize_schema()` (with connection retry).
Called when `create_node_schema()` creates a node schema.

### Checklist

- [x] Migration — `sessions` + `session_messages` tables (edited existing migration)
- [x] `metadata.proto` — 8 RPCs for AI→metadata gRPC communication
- [x] Domain models — `SessionModel`, `SessionMessageModel`, `CreateNodeSchemaModel`, `CreateEdgeSchemaModel`
- [x] `session_repository.rs` — all CRUD methods + active session guard
- [x] `graph_repository.rs` — `create_node_schema`, `create_edge_schema`
- [x] `session_service.rs` — business logic with one-active-session-per-graph enforcement
- [x] `graph_service.rs` — schema CRUD with key/color generation + knowledge init hook
- [x] DTOs — Proto ↔ Model conversions for sessions
- [x] gRPC server — `MetadataGrpcService` implementing all 8 RPCs
- [x] gRPC server wired alongside HTTP in `lib.rs` via `tokio::select!`
- [x] `KnowledgeClient::initialize_schema()` — called on node schema creation
- [x] Config + env vars updated (`METADATA_GRPC_SERVER_PORT`)
- [x] `bric_a_brac_dtos::utils` made public for `ProtoTimestampExt`
- [x] `cargo sqlx prepare` regenerated (17 cached queries)
- [x] `cargo build` passes (zero warnings)
- [x] All tests pass

**Extra changes:**
- `crates/bric-a-brac-dtos/src/lib.rs` — `pub mod utils` (was private)
- `metadata/Cargo.toml` — added `prost-types = { workspace = true }`

---

## Step 4: AI Service — OpenRouterClient Upgrades ✅

**Status:** COMPLETED

**Goal:** Add SSE streaming and tool calling support to `OpenRouterClient`. Add `EmbeddingClient`.
Add `KnowledgeServerConfig`.

### 4.1 — SSE streaming support

**File:** `ai/src/infrastructure/clients/openrouter_client.rs`

Complete rewrite. Kept original `chat()` method (non-streaming, structured JSON output) and
added `chat_stream()`:

```rust
pub async fn chat_stream(
    &self,
    messages: Vec<Message>,
    tools: Option<Vec<ToolDefinition>>,
) -> Result<StreamChatResult, OpenRouterClientError>
```

The method:
1. Sends request with `"stream": true` and optional `tools` array
2. Reads the SSE response via `response.bytes_stream()` (requires reqwest `"stream"` feature)
3. Parses `data:` SSE lines, accumulates content parts and tool call fragments
4. Returns a complete `StreamChatResult { content: Option<String>, tool_calls: Vec<ToolCall> }`

Uses an internal `ToolCallBuilder` to accumulate streamed tool call deltas (id, name,
arguments arrive in fragments across multiple SSE events).

**Key types added:**

```rust
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

impl Message {
    pub fn system(content) -> Self
    pub fn user(content) -> Self
    pub fn assistant(content, tool_calls) -> Self
    pub fn tool(tool_call_id, content) -> Self
}

pub struct ToolDefinition { pub type_: String, pub function: FunctionDefinition }
pub struct FunctionDefinition { pub name: String, pub description: String, pub parameters: Value }
pub struct ToolCall { pub id: String, pub type_: String, pub function: FunctionCall }
pub struct FunctionCall { pub name: String, pub arguments: String }
pub struct ChatResult { pub raw_content: String, pub value: Value }
pub struct StreamChatResult { pub content: Option<String>, pub tool_calls: Vec<ToolCall> }
```

### 4.2 — EmbeddingClient

**File:** `ai/src/infrastructure/clients/embedding_client.rs` (new)

```rust
pub struct EmbeddingClient { api_key, embedding_model, client }

impl EmbeddingClient {
    pub fn new(config: &OpenRouterConfig) -> Self
    pub async fn embed(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>>
    pub async fn embed_one(&self, text: String) -> Result<Vec<f32>>
}
```

Calls `POST https://openrouter.ai/api/v1/embeddings` with configurable model. Shares the
same API key as `OpenRouterClient`. Uses the same error type (`OpenRouterClientError`).

### 4.3 — Config updates

**File:** `ai/src/infrastructure/config/openrouter_config.rs`
- Added `openrouter_embedding_model: String` (env: `OPENROUTER_EMBEDDING_MODEL`, default: `"openai/text-embedding-3-small"`)
- Added `embedding_model()` getter

**File:** `ai/src/infrastructure/config/knowledge_server_config.rs` (new)
- `KnowledgeServerConfig` with `knowledge_grpc_server_url: Uri` (env: `KNOWLEDGE_GRPC_SERVER_URL`)
- For AI → Knowledge gRPC client connection

**File:** `ai/src/infrastructure/config/mod.rs`
- Config now has 4 sub-configs: `AiServerConfig`, `MetadataServerConfig`, `KnowledgeServerConfig`, `OpenRouterConfig`

### 4.4 — Client module

**File:** `ai/src/infrastructure/clients/mod.rs` — exports:
- `OpenRouterClient`, `Message`, `ToolDefinition`, `ToolCall`, `FunctionDefinition`, `FunctionCall`, `ChatResult`, `StreamChatResult`
- `EmbeddingClient`

### 4.5 — Dependencies

**File:** `ai/Cargo.toml`:
- Added `futures-util = { workspace = true }` (for `StreamExt` on bytes stream)
- Changed reqwest features to `["json", "stream"]` (stream feature needed for `bytes_stream()`)

### Checklist

- [x] `chat_stream()` method with SSE parsing and tool call accumulation
- [x] `Message` type with `system()`, `user()`, `assistant()`, `tool()` constructors
- [x] `ToolDefinition`, `ToolCall`, `FunctionDefinition`, `FunctionCall`, `StreamChatResult` types
- [x] `EmbeddingClient` with `embed()` (batch) and `embed_one()` methods
- [x] Config: embedding model (`OPENROUTER_EMBEDDING_MODEL`), knowledge server URL (`KNOWLEDGE_GRPC_SERVER_URL`)
- [x] `KnowledgeServerConfig` added to AI config
- [x] `cargo build -p ai` passes (zero warnings)

**Note:** Provider routing (OpenAI/Anthropic preference to avoid Bedrock) was not implemented
in this step — will be added when needed during agent loop implementation (Step 5).

---

## Step 5: AI Service — Agent Loop ✅

**Status:** COMPLETED

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

A gRPC client wrapping the metadata service. Implements `GrpcClient` trait from
`bric-a-brac-protos`. Uses `metadata.proto` (created in Step 3):

- `create_session(graph_id, user_id) → Session`
- `get_session(session_id) → Session`
- `close_session(session_id, status)`
- `get_messages(session_id) → Vec<SessionMessage>`
- `append_messages(session_id, Vec<NewSessionMessage>)`
- `get_schema(graph_id) → GraphSchemaProto` — all schemas for a graph
- `create_node_schema(graph_id, label, description) → NodeSchemaProto`
- `create_edge_schema(graph_id, label, description) → EdgeSchemaProto`

### 5.2 — Tool definitions

**File:** `ai/src/application/services/tools.rs` (new)

Defines the tool schemas as `ToolDefinition` structs:

```rust
pub fn read_tools() -> Vec<ToolDefinition> {
    vec![search_nodes_tool(), get_node_tool(), get_neighbors_tool(), find_paths_tool()]
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
```

Each tool definition includes name, description, and JSON Schema for parameters.

**New tools:**
- `create_schema` — params: `name` (string), `description` (string) → creates a node schema
- `create_edge_schema` — params: `name` (string), `description` (string) → creates an edge schema

### 5.3 — System prompt builder

**File:** `ai/src/application/services/prompt.rs` (new)

```rust
pub fn build_system_prompt(types: &GraphSchemaDto, user_role: &str) -> String {
    // Builds the dynamic system prompt as described in AI_AGENT_DESIGN.md
    // - Identity section
    // - Schemas section: human-readable listing of node schemas + edge schemas
    //   (name, key, description — no properties)
    // - Capabilities section (based on role)
    // - Behavioral rules (type reuse, entity resolution awareness, normalization)
}
```

Types are injected fresh each message (not once per session). Format:

```
Node schemas:
- Person (key: ESVhRs9k): "Any human individual"
- Company (key: dudFcexv): "Organizations, corporations"

Edge schemas:
- WorksAt (key: xR4kLm2p): "Employment or affiliation"
```

### 5.4 — Tool executor

**File:** `ai/src/application/services/tool_executor.rs` (new)

```rust
pub struct ToolExecutor {
    knowledge_client: KnowledgeClient,
    metadata_client: MetadataClient,  // gRPC client (uses metadata.proto)
    embedding_client: EmbeddingClient,
}

impl ToolExecutor {
    pub async fn execute(
        &self,
        tool_call: &ToolCall,
        graph_id: &str,
        session_id: &str,
        user_role: &str,
        schemas: &GraphSchemaProto,
    ) -> ToolResult {
        match tool_call.name.as_str() {
            "search_nodes" => { /* parse args, embed query, call knowledge */ }
            "get_node" => { /* parse args, call knowledge */ }
            "get_neighbors" => { /* parse args, call knowledge */ }
            "find_paths" => { /* parse args, call knowledge */ }
            "create_schema" => {
                if user_role != "write" { return permission_denied(); }
                /* call metadata_client.create_node_schema(graph_id, label, description) */
                /* metadata creates schema + calls knowledge.initialize_schema for vector index */
                /* return schema with key to LLM */
            }
            "create_edge_schema" => {
                if user_role != "write" { return permission_denied(); }
                /* call metadata_client.create_edge_schema(graph_id, label, description) */
                /* return schema with key to LLM */
            }
            "create_node" => {
                if user_role != "write" { return permission_denied(); }
                self.validate_schema_exists(schemas, &node_key)?;
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
                self.validate_schema_exists(schemas, &edge_key)?;
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

**Schema validation (replaces old property validation):** The tool executor only checks that the
`node_key` / `edge_key` exists in the schemas loaded from metadata (via `get_schema` gRPC call).
No property validation — properties are free-form.

On failure, returns a descriptive error as the `ToolResult` (not a gRPC error):

    "Unknown node schema 'xInvalid'. Valid schemas: ESVhRs9k (Person), dudFcexv (Company)."

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
    metadata_client: MetadataClient,  // gRPC client (uses metadata.proto)
    tool_executor: ToolExecutor,
}

impl AgentService {
    pub fn send_message(
        &self,
        session_id: String,
        user_message: String,
    ) -> impl Stream<Item = AgentEvent> {
        // 1. Load session from metadata via gRPC (get graph_id, user_role from accesses)
        // 2. Load messages from metadata via gRPC
        // 3. Load schemas from metadata via gRPC (get_schema) → build system prompt
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
        //       - If create_schema/create_edge_schema was called → refresh schemas from metadata
        //       - Check iteration count (max 50)
        //    d. Go to (a)
        // 7. Persist all new messages to metadata via gRPC
        // 8. If done, update session status via gRPC
    }
}
```

**Important:** When the AI creates a schema during the loop (via `create_schema` or
`create_edge_schema`), the schemas must be refreshed from metadata before the next
iteration so the system prompt and schema validation reflect the new schema.

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

- [x] `knowledge_client.rs` — gRPC client for knowledge service (7 RPCs: insert/update node, insert edge, search nodes, get node, get neighbors, find paths)
- [x] `metadata_client.rs` — gRPC client for metadata service (8 RPCs: session CRUD, message append/get, schema CRUD)
- [x] `tools.rs` — 10 tool definitions: 4 read (search_nodes, get_node, get_neighbors, find_paths) + 5 write (create_schema, create_edge_schema, create_node, create_edge, update_node) + 1 session (done)
- [x] `prompt.rs` — system prompt builder with dynamic schema listing and behavioral rules
- [x] `tool_executor.rs` — dispatches tool calls, entity resolution in `create_node` (similarity search + neighbor context), schema-only validation, auto-embedding on create/update
- [x] `agent_service.rs` — core agent loop with streaming via `mpsc` channel, schema refresh on create, max 50 iterations, message persistence
- [x] Module exports updated (`AgentService`, `ToolExecutor`)
- [x] `cargo build -p ai` passes

**Implementation notes:**
- `ToolExecutor` returns `ToolResult { content, schema_changed, is_done }` — simple struct, not gRPC errors
- Entity resolution uses `SIMILARITY_THRESHOLD = 0.3` and `ENTITY_RESOLUTION_LIMIT = 5`
- Agent loop uses `tokio::spawn` + `mpsc::Sender<AgentEventProto>` instead of returning a `Stream` trait (simpler wiring with tonic)
- All tools available to all users (no role-based filtering yet — deferred to Step 7 testing)
- `KNOWLEDGE_GRPC_SERVER_URL` added to `ai/mise.local.toml`

---

## Step 6: AI Service — SendMessage Handler + Wiring ✅

**Status:** COMPLETED

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

- [x] `send_message` RPC handler implemented — bridges `mpsc::channel<AgentEventProto>` to `ReceiverStream` for tonic
- [x] `AiService` struct updated with `AgentService` field
- [x] `lib.rs` wiring updated — creates OpenRouterClient, EmbeddingClient, KnowledgeClient, MetadataClient → ToolExecutor → AgentService → AiService
- [x] `cargo build -p ai` passes (zero errors)
- [ ] End-to-end test: create session → send message → receive streaming events (deferred to Step 7)

**Implementation notes:**
- `AiService.send_message()` creates two channels: one for raw `AgentEventProto` events from the agent, one forwarding task wrapping them as `Ok(event)` for the gRPC stream
- `AgentService.send_message()` takes a `mpsc::Sender<AgentEventProto>` and spawns the loop in a `tokio::spawn` task
- All 4 clients wired in `lib.rs`: `KnowledgeClient::new(config.knowledge_server().clone())`, `MetadataClient::new(config.metadata_server().clone())`

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
| Document ingestion | AI creates schemas on first document ingestion |
| Schema creation | AI creates schemas only when needed, reuses existing ones |
| Entity resolution | AI receives duplicate warnings on similar nodes, merges correctly |
| Question answering | Agent calls search_nodes, reasons, responds in text |
| Read-only user | Write tools return permission denied, agent communicates it |
| Unknown type | Agent reports error, gets valid types list, self-corrects |
| Empty graph | Agent handles no search results gracefully, creates types from scratch |
| 50 tool limit | Agent stops after 50 calls, streams error |

### Checklist

- [ ] All services start and connect
- [ ] AI creates schemas on first document ingestion
- [ ] Vector indexes created when node schemas are created
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
- [ ] Schema list displayed in sidebar/panel

---

## Dependencies Between Steps

```
Step 1 (Protos) ✅ ───┬──→ Step 2 (Knowledge) ✅
                      ├──→ Step 2b (Schema Simplification) ✅
                      ├──→ Step 3 (Metadata) ✅
                      └──→ Step 4 (AI: clients) ✅

Step 2  ──┐
Step 2b ──┤
Step 3  ──┼──→ Step 5 (AI: agent loop) ✅ ──→ Step 6 (AI: handler) ✅ ──→ Step 7 (Testing)
Step 4  ──┘

Step 7 ──→ Step 8 (Web UI)
```

Steps 2, 3, and 4 were done after Step 1 (2 was done individually, 3 and 4 in parallel).
Step 2b was done between 2 and 3.
Steps 5 and 6 were implemented together (all now complete).

---

## Files Created (New)

| File | Service | Step |
|---|---|---|
| `metadata/src/domain/models/session_model.rs` | metadata | 3 |
| `metadata/src/domain/models/session_message_model.rs` | metadata | 3 |
| `metadata/src/infrastructure/repositories/session_repository.rs` | metadata | 3 |
| `metadata/src/application/services/session_service.rs` | metadata | 3 |
| `metadata/src/application/dtos/session_dto.rs` | metadata | 3 |
| `metadata/src/presentation/grpc/mod.rs` | metadata | 3 |
| `metadata/src/presentation/grpc/metadata_service.rs` | metadata | 3 |
| `crates/bric-a-brac-protos/protos/metadata.proto` | protos | 3 |
| `ai/src/infrastructure/clients/embedding_client.rs` | ai | 4 |
| `ai/src/infrastructure/config/knowledge_server_config.rs` | ai | 4 |
| `ai/src/infrastructure/clients/knowledge_client.rs` | ai | 5 |
| `ai/src/infrastructure/clients/metadata_client.rs` | ai | 5 |
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
| `metadata/src/application/services/graph_service.rs` | Schema CRUD methods (`create_node_schema`, `create_edge_schema`), key/color generation, `initialize_schema` hook | 3 |
| `metadata/src/infrastructure/clients/knowledge_client.rs` | `initialize_schema()`, `try_initialize_schema()` methods | 3 |
| `metadata/src/domain/models/mod.rs` | New model exports (sessions + Create*SchemaModel) | 3 |
| `metadata/src/domain/models/node_schema_model.rs` | Added `CreateNodeSchemaModel` | 3 |
| `metadata/src/domain/models/edge_schema_model.rs` | Added `CreateEdgeSchemaModel` | 3 |
| `metadata/src/infrastructure/repositories/mod.rs` | Added `SessionRepository` export | 3 |
| `metadata/src/infrastructure/repositories/graph_repository.rs` | Added `create_node_schema`, `create_edge_schema` | 3 |
| `metadata/src/application/services/mod.rs` | Added `SessionService` export | 3 |
| `metadata/src/application/dtos/mod.rs` | Added `session_dto` module | 3 |
| `metadata/src/presentation/mod.rs` | Added `grpc` module | 3 |
| `metadata/src/presentation/state.rs` | Added `SessionService` + `SessionRepository` to `ApiState` | 3 |
| `metadata/src/lib.rs` | gRPC server running alongside HTTP via `tokio::select!` | 3 |
| `metadata/src/infrastructure/config/metadata_server_config.rs` | Added `grpc_url()` + `METADATA_GRPC_SERVER_PORT` | 3 |
| `metadata/mise.local.toml` | Added `METADATA_GRPC_SERVER_PORT="50052"` | 3 |
| `metadata/Cargo.toml` | Added `prost-types = { workspace = true }` | 3 |
| `metadata/migrations/20260130112820_setup.up.sql` | Added sessions + session_messages tables | 3 |
| `metadata/migrations/20260130112820_setup.down.sql` | Added drops for session tables | 3 |
| `crates/bric-a-brac-protos/build.rs` | Added `metadata.proto` to compilation | 3 |
| `crates/bric-a-brac-protos/src/lib.rs` | Added `pub mod metadata` | 3 |
| `crates/bric-a-brac-dtos/src/lib.rs` | Changed `mod utils` to `pub mod utils` | 3 |
| `ai/src/infrastructure/clients/openrouter_client.rs` | Complete rewrite: SSE streaming + tool calling + `Message` constructors | 4 |
| `ai/src/infrastructure/clients/mod.rs` | New client exports (`EmbeddingClient`, `Message`, `ToolDefinition`, `ToolCall`, etc.) | 4 |
| `ai/src/infrastructure/config/mod.rs` | Added `KnowledgeServerConfig` | 4 |
| `ai/src/infrastructure/config/openrouter_config.rs` | Embedding model config (`OPENROUTER_EMBEDDING_MODEL`) | 4 |
| `ai/Cargo.toml` | Added `futures-util`, reqwest `"stream"` feature | 4 |
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
