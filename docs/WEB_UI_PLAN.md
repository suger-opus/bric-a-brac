# Web UI — Architecture & Status

> **Branch**: `agent-complete-rework`
> **Last updated**: 2025-03-19
> **Purpose**: Living reference for the web-ui layer. If context is lost, read this file.

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Key Design Decisions](#2-key-design-decisions)
3. [Frontend Structure](#3-frontend-structure)
4. [Backend Endpoints](#4-backend-endpoints)
5. [Remaining Work](#5-remaining-work)

---

## 1. Architecture Overview

### Backend (3 Rust microservices)

| Service | Transport | Port | Role |
|---------|-----------|------|------|
| **metadata** | Axum HTTP + Tonic gRPC | 50052 | Graph CRUD, users, sessions, access control. HTTP for web-ui, gRPC for inter-service |
| **knowledge** | Tonic gRPC | 50051 | Memgraph graph storage, vector embeddings, graph queries |
| **ai** | Tonic gRPC | 50053 | LLM agent with tool-calling. Streams `AgentEventProto` back |

### Frontend

- **Next.js 16.1.1** + **React 19.2.3** + **TypeScript 5** (React Compiler enabled)
- **TailwindCSS v4** + **shadcn/ui** (Radix primitives)
- **Valibot v1.2.0** for DTO runtime validation
- **Mande v2.0.9** as HTTP client (via `lib/api/client.ts`)
- **react-force-graph-3d** for 3D graph visualization
- Config: `NEXT_PUBLIC_API_URL` env var pointing to metadata HTTP

### Communication Flow

```
Browser (Next.js)
    │
    ├─▶ HTTP REST ──▶ Metadata (Axum)   ── graphs, users, sessions
    │
    └─▶ HTTP SSE  ──▶ Metadata (Axum)   ── chat (POST /graphs/{id}/chat)
                         │
                         └─▶ gRPC stream ──▶ AI Service (LLM + tools)
                                               ├─▶ gRPC ──▶ Metadata
                                               └─▶ gRPC ──▶ Knowledge
```

The web-ui **only talks HTTP** to metadata. It cannot talk gRPC directly.

### Layering Rules

These rules apply across the entire codebase:

- **Handlers** extract request data, call services, return responses. No business logic.
- **Services** take and return **DTOs**. Convert DTO↔Model internally.
- **Repositories** take and return **Models**.
- **gRPC clients** take and return **DTOs**.
- **Proto↔Dto** conversions live in dto files as `From` trait impls.
- **Dto↔Model** conversions live in dto files as `From` trait impls.
- Zero `.expect()` / `.unwrap()` in production code. gRPC client constructors return `anyhow::Result<Self>`.
- Type-safe IDs via `id!()` macro (e.g. `SessionIdDto`, `GraphIdDto`).

---

## 2. Key Design Decisions

### The AI Agent is the only way to mutate graph content

- Users **cannot** directly create node/edge schemas or insert/update/delete data.
- All graph mutations go through the **AI chat interface**: user sends a message → AI agent uses tools to create schemas, insert nodes/edges, etc.
- The only direct user actions are: **create a graph** (empty), **view graph** (3D viz + sidebar), **chat with AI**.
- `GET /graphs/{id}/schema` and `GET /graphs/{id}/data` remain as read-only endpoints.

### Simplified scope (for now)

- **No bookmarks/cheers** — hardcoded to `[]` in dashboard cards.
- **No graph metadata updates** — no `PUT /graphs/{id}`, no visibility toggle, no rename.
- **No access management** — users can only see their own graphs.
- **No search** — hardcoded to `[]` in search card.
- **No graph deletion** — disabled.
- **Core focus**: Dashboard (list + create graphs) → Graph page (3D viz + AI chat).

### No command palette, menubar, or drawer

All removed. The graph page is: 3D visualization + sidebar with two tabs (Chat, Schema).

### Auth

Hardcoded `user_id` header (`"019cfc3c-20c4-7aa2-a098-a547f9f13213"`) set in `lib/api/client.ts`.
`AuthenticatedUser` extractor in metadata reads this header. No real auth yet.

---

## 3. Frontend Structure

### API Layer (`lib/api/`)

**`client.ts`** — Typed HTTP client wrapping Mande.
- `get<T>(path, schema)` and `post<T>(path, body, schema)` with Valibot runtime validation.
- Single mande instance with hardcoded `user_id` header.

**Services** (plain objects, no classes):

| Service | Methods |
|---------|---------|
| `graph-service.ts` | `getAllMetadata()`, `getOneMetadata(id)`, `getSchema(id)`, `getData(id)`, `createGraph(body)` |
| `user-service.ts` | `getCurrent()` |
| `session-service.ts` | `create(graphId)`, `get(sessionId)`, `close(sessionId)`, `getMessages(sessionId)` |
| `chat-service.ts` | `streamChat(graphId, sessionId, content, onEvent, onDone?, onError?)` → `AbortController` |

**DTOs** (Valibot schemas):

| File | Exports |
|------|---------|
| `graph-dto.ts` | `GraphMetadataDto`, `CreateGraphDto` |
| `graph-schema-dto.ts` | `GraphSchemaDto` (contains `NodeSchemaDto[]` + `EdgeSchemaDto[]`) |
| `graph-data-dto.ts` | `GraphDataDto` (contains `NodeDataDto[]` + `EdgeDataDto[]`) |
| `node-schema-dto.ts` | `NodeSchemaDto` — has `description: string` (not properties) |
| `edge-schema-dto.ts` | `EdgeSchemaDto` — has `description: string` (not properties) |
| `node-data-dto.ts` | `NodeDataDto` |
| `edge-data-dto.ts` | `EdgeDataDto` |
| `property-data-dto.ts` | `PropertyValueDto`, `PropertiesDataDto` — `Record<string, string \| number \| boolean>` |
| `user-dto.ts` | `UserDto` |
| `session-dto.ts` | `SessionDto`, `SessionMessageDto` |

**Chat service** (`chat-service.ts`):
- Uses raw `fetch()` with manual SSE parsing (not EventSource — needs POST body).
- `ChatEvent` discriminated union: `text`, `tool_call`, `tool_result`, `done`, `error`.
- Returns `AbortController` for cancellation.

### Graph Page (`app/graph/[graph_id]/page.tsx`)

```
page.tsx
└── GraphProvider (context)
    ├── Graph (3D force graph — react-force-graph-3d)
    └── GraphSidebar
        ├── Header (metadata: name, owner, dates, privacy, counts)
        ├── Tabs
        │   ├── "Chat" (default) → ChatPanel
        │   └── "Schema" → Collapsible NodeSchemaItem[] + EdgeSchemaItem[]
        └── Footer (exit to home)
```

**`graph-context.tsx`** — Provides:
- `graphId`, `metadata`, `schema`, `data`, `processedData`
- `isLoading`, `isLoaded`, `error`
- `focusNode`, `focusEdge` + setters
- `refetch()` — increments `fetchTrigger` counter to re-fetch all data

**`chat-panel.tsx`** — Full chat UI:
- Lazy session creation (creates on first message send)
- Streaming text display with blinking cursor
- Tool call/result visualization
- Auto-scroll
- Textarea with Enter to send, Shift+Enter for newline
- Calls `refetch()` on "done" event to update the 3D graph

**`graph.tsx`** — 3D force graph:
- Uses schema key as SpriteText node labels
- Schema colors applied to nodes and edges

### Dashboard (`app/page.tsx`)

| Component | Status |
|-----------|--------|
| `accesses-card.tsx` | ✅ Shows user's graphs |
| `new-graph-content.tsx` | ✅ Creates graphs |
| `search-card.tsx` | 🟡 Stub (`[]`) |
| `bookmarks-card.tsx` | 🟡 Stub (`[]`) |
| `cheers-card.tsx` | 🟡 Stub (`[]`) |
| `settings-card.tsx` | 🟡 Partial |

---

## 4. Backend Endpoints

### Metadata HTTP Routes

```
GET  /docs                              → Scalar API docs
GET  /docs/openapi.json                 → OpenAPI spec

POST /users                             → create user
GET  /users/me                          → get current user

GET  /graphs                            → list user's graphs
POST /graphs                            → create graph
GET  /graphs/{graph_id}                 → get graph metadata
GET  /graphs/{graph_id}/schema          → get graph schema
GET  /graphs/{graph_id}/data            → get graph data

POST /accesses/graphs/{graph_id}        → create access

POST /sessions                          → create session (body: { graph_id })
GET  /sessions/{session_id}             → get session
POST /sessions/{session_id}/close       → close session
GET  /sessions/{session_id}/messages    → get messages

POST /graphs/{graph_id}/chat            → SSE chat bridge (body: { session_id, content })
```

### Metadata Backend Architecture (session + chat)

**Handler → Service → Client** chain:

| Layer | File | Responsibility |
|-------|------|----------------|
| `session_handler.rs` | 4 endpoints | Extracts `Path<SessionIdDto>`, calls `session_service`, returns DTOs |
| `chat_handler.rs` | 1 endpoint | Takes `ChatRequestDto`, calls `ai_service.chat()`, maps stream to SSE |
| `session_service.rs` | 5 methods | Takes/returns DTOs, converts DTO↔Model for repository |
| `ai_service.rs` | 1 method (`chat`) | Wraps `AiClient`, returns `impl Stream<Item = AgentEventDto>` |
| `ai_client.rs` | gRPC client | `send_message()` with `with_retry`, returns `Streaming<AgentEventProto>` |

**DTOs:**

| DTO | File |
|-----|------|
| `SessionIdDto` | `session_dto.rs` — via `id!()` macro |
| `CreateSessionDto` | `session_dto.rs` — `{ graph_id: GraphIdDto }` |
| `SessionDto` | `session_dto.rs` — full session with `From<SessionModel>` + `From<SessionDto> for SessionProto` |
| `SessionMessageDto` | `session_dto.rs` — with `From<SessionMessageModel>` + `From<SessionMessageDto> for SessionMessageProto` |
| `ChatRequestDto` | `ai_dto.rs` — `{ session_id, content }` |
| `AgentEventDto` | `ai_dto.rs` — enum (Text/ToolCall/ToolResult/Done/Error) with `#[derive(Serialize)]` + `From<Option<Event>>` |

### SSE Event Format

```
event: text
data: {"content":"I'll create a Person node type..."}

event: tool_call
data: {"tool_call_id":"abc","name":"create_node_schema","arguments":"{...}"}

event: tool_result
data: {"tool_call_id":"abc","content":"Created node schema 'Person'"}

event: done
data: {"summary":"Created 2 node types and 15 nodes"}

event: error
data: {"message":"Failed to create node schema"}
```

---

## 5. Remaining Work

### Phase 4 — Live Graph Updates

The wiring exists (`refetch()` called on "done" event in `chat-panel.tsx`) but has not been end-to-end tested.

- [ ] Verify that `fetchTrigger` increment correctly re-fetches schema + data
- [ ] Verify 3D graph visualization updates after AI creates schemas/nodes
- [ ] Consider incremental updates from `tool_result` events (optimistic updates)

### Phase 5 — Polish

- [ ] Toast notifications for AI actions (shadcn Sonner)
- [ ] Error handling — toast on API failures, graceful fallbacks
- [ ] Loading states — skeleton loaders for chat messages, dashboard cards
- [ ] End-to-end test: create graph → chat → AI builds graph → 3D renders
