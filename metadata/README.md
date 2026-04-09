# Metadata Service

The **control plane microservice** and the only service exposed to the frontend. It owns
users, graphs, schemas, sessions, documents, and access control in PostgreSQL — and serves
as the HTTP API gateway + SSE chat bridge.

The web UI talks exclusively to this service over HTTP. Internally, it communicates with
the knowledge and AI services over gRPC.

---

## Architecture

```
src/
├── main.rs                              # Entry point: load config, start HTTP + gRPC
├── lib.rs                               # Wire up layers, run HTTP and gRPC concurrently
├── application/
│   ├── services/
│   │   ├── user_service.rs              # User CRUD
│   │   ├── graph_service.rs             # Graph lifecycle, schema CRUD, data proxy
│   │   ├── session_service.rs           # Session lifecycle, messages, documents
│   │   ├── chat_service.rs              # SSE bridge to AI service
│   │   └── access_service.rs            # RBAC: grant/check access
│   ├── dtos/                            # Data transfer objects
│   └── error.rs                         # AppError (Forbidden, NotFound, Conflict, etc.)
├── domain/
│   └── models/                          # Domain models, RoleModel with level-based hierarchy
├── infrastructure/
│   ├── database.rs                      # PostgreSQL connection pool (SQLx)
│   ├── repositories/
│   │   ├── user_repository.rs           # User queries
│   │   ├── graph_repository.rs          # Graph + schema queries
│   │   ├── session_repository.rs        # Session + message + document queries
│   │   └── access_repository.rs         # RBAC queries
│   ├── clients/
│   │   ├── knowledge_client.rs          # gRPC client to knowledge service
│   │   └── ai_client.rs                 # gRPC client to AI service
│   └── config/                          # Environment-based configuration
└── presentation/
    ├── http/
    │   ├── router.rs                    # Axum route definitions
    │   ├── state.rs                     # Shared application state
    │   ├── openapi.rs                   # OpenAPI/Scalar documentation
    │   ├── handlers/
    │   │   ├── user_handler.rs          # POST /users, GET /users/me
    │   │   ├── graph_handler.rs         # CRUD + schema + data
    │   │   ├── session_handler.rs       # Sessions + messages
    │   │   ├── chat_handler.rs          # SSE streaming chat (multipart upload)
    │   │   └── access_handler.rs        # Grant access
    │   └── extractors/                  # AuthenticatedUser extractor
    ├── grpc.rs                          # gRPC Metadata trait (internal APIs)
    └── tracing.rs                       # Tracing setup
```

### Layers

| Layer | Responsibility |
|-------|---------------|
| **Presentation (HTTP)** | Axum router, handlers, auth extraction, SSE streaming, OpenAPI docs at `/docs` |
| **Presentation (gRPC)** | Internal API for the AI service: session/message/document/schema operations |
| **Application** | Business logic, RBAC enforcement, DTO↔Model conversions |
| **Domain** | Models: users, graphs, schemas, sessions, messages, documents, roles |
| **Infrastructure** | SQLx repositories, gRPC clients, config |

---

## HTTP API

All routes require authentication via `AuthenticatedUser` extractor (currently reads
`user_id` from a header — OAuth2 planned).

### Users

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/users` | Create a user |
| `GET` | `/users/me` | Get the authenticated user |

### Graphs

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/graphs` | List all graphs the user has access to |
| `POST` | `/graphs` | Create a graph (assigns Owner role) |
| `GET` | `/graphs/{graph_id}` | Get graph metadata (Viewer+) |
| `DELETE` | `/graphs/{graph_id}` | Delete graph + cascade (Owner only) |
| `GET` | `/graphs/{graph_id}/schema` | Get node and edge schemas |
| `GET` | `/graphs/{graph_id}/data` | Get full graph data (proxied from knowledge) |

### Sessions

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/graphs/{graph_id}/sessions` | Create a session (one active per graph) |
| `GET` | `/graphs/{graph_id}/sessions` | List user's sessions for a graph |
| `POST` | `/sessions/{session_id}/close` | Close a session (set status to Completed) |
| `GET` | `/sessions/{session_id}/messages` | Get session message history |

### Chat

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/graphs/{graph_id}/chat` | SSE streaming chat (multipart: text + optional file, 50MB limit) |

The chat endpoint bridges HTTP to gRPC: it calls `ai_service.send_message()` over gRPC
and maps `AgentEventProto` stream events to SSE events (`text`, `tool_call`, `tool_result`,
`done`, `error`).

### Access Control

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/graphs/{graph_id}/accesses` | Grant a user access to a graph (Admin+ only) |

### API Documentation

OpenAPI docs are served at `/docs` via Scalar UI.

---

## gRPC API (Internal)

Defined in [`metadata.proto`](../crates/bric-a-brac-protos/protos/metadata.proto).
Called by the AI service during agent execution:

| RPC | Description |
|-----|-------------|
| `GetSession` | Load session with user role |
| `GetSessionMessages` | Load conversation history |
| `GetSessionDocument` | Load document content by ID |
| `AppendSessionMessages` | Persist new messages (user, assistant, tool) |
| `CreateNodeSchema` | Create a node schema (generates key, notifies knowledge) |
| `CreateEdgeSchema` | Create an edge schema (generates key) |
| `GetSchema` | Load all node + edge schemas for a graph |

---

## Database

**PostgreSQL 18** with SQLx (compile-time query checking via offline mode).

### Tables

| Table | Purpose |
|-------|---------|
| `users` | User accounts (email, username) |
| `graphs` | Graph metadata (name, description, is_public) |
| `accesses` | RBAC: (user_id, graph_id, role) with role hierarchy |
| `nodes_schemas` | Node type definitions (label, 8-char key, color, description) |
| `edges_schemas` | Edge type definitions (label, 8-char key, color, description) |
| `sessions` | Conversation sessions (active/completed/error) |
| `session_messages` | Message history (role, content, tool_calls, position) |
| `session_documents` | Uploaded documents (filename, content_hash, extracted text) |

### Role Hierarchy

| Role | Level | Permissions |
|------|-------|-------------|
| **Owner** | 4 | Full control, delete graph |
| **Admin** | 3 | Manage access, schemas |
| **Editor** | 2 | Read + write data |
| **Viewer** | 1 | Read only |
| **None** | 0 | No access |

Migrations are in `migrations/` and run automatically on startup (unless `METADATA_DB_SKIP_MIGRATION=true`).

---

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `METADATA_SERVER_HOST` | Bind address | `0.0.0.0` |
| `METADATA_HTTP_SERVER_PORT` | HTTP listen port | `8080` |
| `METADATA_GRPC_SERVER_PORT` | gRPC listen port | `50052` |
| `METADATA_DB_URL` | PostgreSQL connection string | **Yes** |
| `METADATA_DB_MAX_CONNECTIONS` | Connection pool size | `20` |
| `METADATA_DB_SKIP_MIGRATION` | Skip auto-migration on startup | `false` |
| `SQLX_OFFLINE` | Use offline query data (no live DB needed at compile time) | `true` |
| `KNOWLEDGE_GRPC_SERVER_URL` | Knowledge service URL | `http://localhost:50051` |
| `AI_GRPC_SERVER_URL` | AI service URL | `http://localhost:50053` |
| `INTERNAL_SERVICES_AUTH_TOKEN` | Shared gRPC auth token | **Yes** |
| `RUST_LOG` | Tracing filter | `info` |

For local development, configure in `mise.local.toml`:

```toml
[env]
DATABASE_URL = "postgresql://bricabrac:bricabrac@localhost:5432/bricabrac"
KNOWLEDGE_GRPC_SERVER_URL = "http://localhost:50051"
AI_GRPC_SERVER_URL = "http://localhost:50053"
INTERNAL_SERVICES_AUTH_TOKEN = "dev-token"
```

> `DATABASE_URL` is used by SQLx CLI for migration management. The service reads `METADATA_DB_URL`.

---

## Running

### Start the database

```bash
docker compose --profile metadata-db up -d
# PostgreSQL on :5432
```

### Run locally

```bash
cargo run   # HTTP on :8080, gRPC on :50052
```

### With Docker

```bash
docker compose --profile metadata up -d
```

The Dockerfile uses a multi-stage build: `rust:1.93` builder with `protobuf-compiler`,
then `debian:bookworm-slim` runtime.