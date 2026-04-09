# bric-a-brac-protos

Shared crate containing **Protocol Buffer definitions** and generated gRPC code for all
inter-service communication. Also provides gRPC utilities: authentication interceptors,
retry helpers, and server construction.

---

## What's Inside

### Proto Files

Located in `protos/`, compiled by `build.rs` via `tonic-prost-build`:

| File | Purpose |
|------|---------|
| [`common.proto`](protos/common.proto) | Shared types: roles, schemas, graph data, sessions, messages, documents |
| [`ai.proto`](protos/ai.proto) | AI agent streaming service (`SendMessage` → `stream AgentEventProto`) |
| [`knowledge.proto`](protos/knowledge.proto) | Knowledge graph CRUD + vector search (13 RPCs) |
| [`metadata.proto`](protos/metadata.proto) | Metadata control plane: sessions, messages, schemas (7 RPCs) |

### Generated Modules

```rust
use bric_a_brac_protos::common::*;    // Shared types
use bric_a_brac_protos::ai::*;        // AI service client/server
use bric_a_brac_protos::knowledge::*;  // Knowledge service client/server
use bric_a_brac_protos::metadata::*;   // Metadata service client/server
```

### Utilities

| Module | Export | Description |
|--------|--------|-------------|
| `auth` | `AuthChannel`, `ServiceAuthInterceptor` | Bearer token auth for gRPC channels |
| `client` | `with_retry` | gRPC call retry with exponential backoff (3 attempts, retries on `Unavailable`/`DeadlineExceeded`) |
| `server` | `build_grpc_server` | Construct a tonic gRPC server with auth layer |
| `error` | `GrpcRequestError` | Typed gRPC error wrapper |

---

## gRPC Services

### AI Service (`ai.proto`)

```
Ai.SendMessage(session_id, content, document_id?) → stream AgentEventProto
```

Stream events: `text`, `tool_call`, `tool_result`, `done`, `error`, `progress`.

### Knowledge Service (`knowledge.proto`)

| RPC | Request | Response |
|-----|---------|----------|
| `LoadGraph` | `graph_id` | `GraphDataProto` (all nodes + edges) |
| `InitializeSchema` | `graph_id`, `node_keys[]` | `Empty` |
| `CreateNode` | `graph_id`, `CreateNodeDataProto` | `NodeDataProto` |
| `UpdateNode` | `graph_id`, `UpdateNodeDataProto` | `NodeDataProto` |
| `DeleteNode` | `graph_id`, `node_data_id` | `Empty` |
| `CreateEdge` | `graph_id`, `CreateEdgeDataProto` | `EdgeDataProto` |
| `UpdateEdge` | `graph_id`, `UpdateEdgeDataProto` | `EdgeDataProto` |
| `DeleteEdge` | `graph_id`, `edge_data_id` | `Empty` |
| `SearchNodes` | `graph_id`, `node_key?`, `query_embedding[]`, `limit` | `NodeSearchProto[]` |
| `GetNode` | `graph_id`, `node_data_id` | `NodeDataProto` |
| `GetNeighbors` | `graph_id`, `node_data_id`, `edge_key?`, `depth` | `GraphDataProto` |
| `FindPaths` | `graph_id`, `from_id`, `to_id`, `max_depth` | `GraphDataProto[]` |
| `DeleteGraph` | `graph_id`, `node_keys[]` | `Empty` |

### Metadata Service (`metadata.proto`)

| RPC | Request | Response |
|-----|---------|----------|
| `GetSession` | `session_id` | `SessionProto` |
| `GetSessionMessages` | `session_id` | `SessionMessageProto[]` |
| `GetSessionDocument` | `document_id` | `SessionDocumentProto` |
| `AppendSessionMessages` | `session_id`, `messages[]` | `Empty` |
| `CreateNodeSchema` | `graph_id`, `label`, `description` | `NodeSchemaProto` |
| `CreateEdgeSchema` | `graph_id`, `label`, `description` | `EdgeSchemaProto` |
| `GetSchema` | `graph_id` | `GraphSchemaProto` |

---

## Shared Types (`common.proto`)

### Enums

- `RoleProto` — Owner, Admin, Editor, Viewer, None
- `SessionStatusProto` — Active, Completed, Error
- `SessionMessageRoleProto` — System, User, Assistant, Tool

### Key Messages

- `NodeSchemaProto` / `EdgeSchemaProto` — schema definitions with 8-char alphanumeric keys
- `NodeDataProto` — node with `node_data_id`, `key` (schema), `properties` map
- `EdgeDataProto` — edge with endpoints, key, properties
- `NodeSearchProto` — search result with cosine `distance`
- `SessionProto` — session with status, timestamps, user role
- `SessionMessageProto` — message with position, role, content, tool calls, document ref
- `PropertyValueProto` — `oneof` string | number | bool

---

## Build

Protos are compiled automatically by `cargo build` via `build.rs`. The build script runs
two passes:

1. **Types only** — compile `common.proto` without client/server generation
2. **Full** — compile all protos with client + server + transport code

To regenerate after editing `.proto` files:

```bash
cargo build -p bric-a-brac-protos
```

### Dependencies

- `tonic` — gRPC framework
- `prost` — Protobuf serialization
- `tonic-prost-build` — Build-time proto compilation
- `tower` / `tower-http` — Middleware (tracing, auth)
- `secrecy` — Bearer token handling
