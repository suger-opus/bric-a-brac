# Web UI — Analysis & Implementation Plan

> **Date**: 2025-03-19  
> **Branch**: `agent-complete-rework`  
> **Purpose**: Complete reference for bringing the web-ui in sync with the reworked backend. If context is lost, read this file.

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Key Design Decisions](#2-key-design-decisions)
3. [Current Web UI State — What Works](#3-current-web-ui-state--what-works)
4. [What's Dead — Must Remove](#4-whats-dead--must-remove)
5. [What's Mismatched — Must Update](#5-whats-mismatched--must-update)
6. [Missing Backend Endpoints](#6-missing-backend-endpoints)
7. [Step-by-Step Implementation Plan](#7-step-by-step-implementation-plan)

---

## 1. Architecture Overview

### Backend (3 Rust microservices)

| Service | Transport | Port | Role |
|---------|-----------|------|------|
| **metadata** | Axum HTTP + Tonic gRPC | 50052 | Graph CRUD, users, sessions, access control. HTTP for web-ui, gRPC for inter-service |
| **knowledge** | Tonic gRPC | 50051 | Neo4j graph storage, vector embeddings, graph queries |
| **ai** | Tonic gRPC | 50053 | LLM agent with tool-calling. Streams `AgentEventProto` back |

### Frontend

- **Next.js 16.1.1** + **React 19.2.3** + **TypeScript**
- **TailwindCSS v4** + **shadcn/ui** (Radix primitives)
- **Valibot** for DTO runtime validation
- **Mande** as HTTP client (via `proxy.ts`)
- **react-force-graph-3d** for 3D graph visualization
- Config: `NEXT_PUBLIC_API_URL` env var pointing to metadata HTTP

### Communication Flow

```
Browser (Next.js)
    │
    ▼  HTTP (REST + SSE)
Metadata Service (Axum)
    │
    ├──▶ gRPC ──▶ Knowledge Service (Neo4j + pgvector)
    │
    └──▶ gRPC ──▶ AI Service (LLM + tool-calling)
                      │
                      └──▶ gRPC ──▶ Metadata (create schemas, insert data)
                      └──▶ gRPC ──▶ Knowledge (store embeddings, query graph)
```

The web-ui **only talks HTTP** to metadata. It cannot talk gRPC directly.

---

## 2. Key Design Decisions

### The AI Agent is the only way to mutate graph content

- Users **cannot** directly create node/edge schemas or insert/update/delete data.
- All graph mutations go through the **AI chat interface**: user sends a message → AI agent uses tools to create schemas, insert nodes/edges, etc.
- The only direct user actions are: **create a graph** (empty), **view graph** (3D viz + sidebar), **chat with AI**.
- `GET /graphs/{id}/schema` and `GET /graphs/{id}/data` remain as read-only endpoints.

### Simplified scope (for now)

- **No bookmarks/cheers** — hardcode to `[]` in dashboard cards, no backend endpoints needed.
- **No graph metadata updates** — no `PUT /graphs/{id}`, no visibility toggle, no rename.
- **No access management** — users can only see their own graphs. No sharing, no role management.
- **No search** — hardcode to `[]` in search card.
- **No graph deletion** — disabled for now.
- **Core focus**: Dashboard (list + create graphs) → Graph page (3D viz + AI chat).

### Auth

- Currently: hardcoded `user_id` header in `proxy.ts` (`"019cfc3c-20c4-7aa2-a098-a547f9f13213"`).
- `AuthenticatedUser` extractor in metadata reads this header. No real auth yet.
- Keep this approach for now.

---

## 3. Current Web UI State — What Works

### Dashboard (`app/page.tsx`)

| Component | Status | Notes |
|-----------|--------|-------|
| `accesses-card.tsx` | ✅ Works | Fetches `graphService.getAllMetadata()` — shows user's graphs |
| `new-graph-content.tsx` | ✅ Works | Creates graph via `graphService.createGraph()` |
| `search-card.tsx` | 🟡 Stub | Hardcoded `results = []` — keep as-is |
| `bookmarks-card.tsx` | 🟡 Stub | Hardcoded `results = []` — keep as-is |
| `cheers-card.tsx` | 🟡 Stub | Hardcoded `results = []` — keep as-is |
| `settings-card.tsx` | 🟡 Partial | Shows user info, Log Out / Delete do nothing — keep as-is |

### Graph Page (`app/graph/[graph_id]/page.tsx`)

| Component | Status | Notes |
|-----------|--------|-------|
| `graph.tsx` | ✅ Works | 3D force graph rendering |
| `graph-context.tsx` | ✅ Works | Fetches metadata + schema + data, processes for viz |
| `graph-sidebar.tsx` | ✅ Works | Shows metadata + schema items in sidebar |
| `graph-command.tsx` | 🟡 Partial | Command palette — most actions disabled |
| `graph-menu.tsx` | 🟡 Partial | Menubar — same disabled state |
| `graph-dialog.tsx` | ✅ Works | Loading dialog during fetch |
| `graph-drawer.tsx` | ❌ Broken | Maps actions to dead components |
| `node-data-item.tsx` | ✅ Works | Shows clicked node data (read-only) |
| `edge-data-item.tsx` | ✅ Works | Shows clicked edge data (read-only) |

### API Layer

| `GraphService` Method | Status | Backend Route |
|----------------------|--------|---------------|
| `getAllMetadata()` | ✅ | `GET /graphs` |
| `getOneMetadata(id)` | ✅ | `GET /graphs/{id}` |
| `getSchema(id)` | ❌ Broken | `GET /graphs/{id}/schema` — route exists but DTO mismatch (see §5) |
| `getData(id)` | ✅ | `GET /graphs/{id}/data` |
| `createGraph(body)` | ✅ | `POST /graphs` |
| `generateSchema(...)` | ❌ Dead | `POST /graphs/{id}/schema/generate` — endpoint deleted |
| `generateData(...)` | ❌ Dead | `POST /graphs/{id}/data/generate` — endpoint never existed |
| `createSchema(...)` | ❌ Dead | `POST /graphs/{id}/schema` — no POST handler |
| `createData(...)` | ❌ Dead | `POST /graphs/{id}/data` — no POST handler |

---

## 4. What's Dead — Must Remove

### Files to Delete

| File | Why |
|------|-----|
| `lib/api/dtos/property-schema-dto.ts` | `PropertyType`, `PropertySchemaDto`, `CreatePropertySchemaDto` — backend has no property schemas |
| `hooks/use-graph-schema-form.ts` | Calls dead endpoints: `generateSchema`, `generateData`, `createSchema`, `createData` |
| `hooks/use-element-schema-form.ts` | Manual schema creation forms — users can't create schemas directly |
| `hooks/use-element-data-form.ts` | Manual data insertion forms — users can't insert data directly |
| `components/graph/contents/generate-content.tsx` | Old BUILD_WITH_AI pipeline (file upload → generate → submit) |
| `components/graph/contents/new-node-schema-content.tsx` | Manual node type creation — dead |
| `components/graph/contents/new-edge-schema-content.tsx` | Manual edge type creation — dead |
| `components/graph/contents/new-node-data-content.tsx` | Manual node insertion — dead |
| `components/graph/contents/new-edge-data-content.tsx` | Manual edge insertion — dead |
| `components/graph/contents/new-element-schema-content.tsx` | Shared schema creation form — dead |
| `components/graph/contents/new-element-data-content.tsx` | Shared data insertion form — dead |
| `components/graph/forms/property-form.tsx` | Property definition form — dead concept |
| `components/graph/badges/property-type-badge.tsx` | PropertyType badge — dead concept |
| `components/graph/items/draft-element-schema.tsx` | Schema preview during generation — dead |
| `components/graph/items/draft-element-data.tsx` | Data preview during generation — dead |

### Dead Exports to Remove from `types/index.ts`

- `PropertyType`, `PropertySchema`, `PropertyValue`, `CreatePropertySchema`
- `CreateNodeSchema`, `CreateEdgeSchema`
- `CreateGraphSchema`, `CreateGraphData`

### Dead Exports to Remove from `dtos/index.ts`

- `PropertyType`, `PropertySchemaDto`, `CreatePropertySchemaDto`, `CreatePropertySchemaMetadataDto`
- `CreateNodeSchemaDto`, `CreateEdgeSchemaDto`
- `CreateGraphSchemaDto`, `CreateGraphDataDto`

### Dead `GraphService` Interface Members

- `generateSchema()`, `generateData()`, `createSchema()`, `createData()`

### Dead Actions from `Action` Enum

Remove or keep-disabled:
- `NEW_NODE_TYPE`, `NEW_EDGE_TYPE` — users can't manually create schemas
- `INSERT_NODE`, `INSERT_EDGE` — users can't manually insert data
- `MANAGE_NODE_TYPES`, `MANAGE_EDGE_TYPES`, `MANAGE_NODES`, `MANAGE_EDGES` — no management UI
- `METADATA`, `ACCESSES`, `VISIBILITY`, `DELETE_GRAPH`, `ANALYTICS` — disabled for now

Keep:
- `BUILD_WITH_AI` → becomes the **chat interface**
- `ASK_AI` → could be merged with BUILD_WITH_AI or kept as read-only mode
- `FIND_NODE`, `FIND_PATH` → could be client-side search or AI-powered (later)

---

## 5. What's Mismatched — Must Update

### Schema DTOs

**Frontend `NodeSchemaDto`** (current):
```ts
v.object({
  node_schema_id: v.string(),
  graph_id: v.string(),
  label: v.string(),
  key: v.string(),
  color: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
  properties: v.array(PropertySchemaDto) // ❌ WRONG
})
```

**Backend `NodeSchemaDto`** (actual):
```rust
pub struct NodeSchemaDto {
    pub node_schema_id: NodeSchemaIdDto,
    pub graph_id: GraphIdDto,
    pub label: LabelDto,
    pub key: KeyDto,
    pub color: ColorDto,
    pub description: String,  // ✅ THIS
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Fix**: Replace `properties: v.array(PropertySchemaDto)` with `description: v.string()` in both `NodeSchemaDto` and `EdgeSchemaDto`.

### Data DTOs

**Frontend `PropertiesDataDto`**:
```ts
export const PropertyValueDto = v.union([v.string(), v.number(), v.boolean()]);
export const PropertiesDataDto = v.record(v.string(), PropertyValueDto);
```

**Backend `PropertiesDataDto`**: `HashMap<String, PropertyValueProto>` where `PropertyValueProto` is `oneof { string, i64, f64, bool }`.

**The frontend DTO for data is actually correct** — `Record<string, string | number | boolean>` matches. ✅

### Components That Reference Dead Property Schema

| Component | What to Fix |
|-----------|-------------|
| `element-schema-item.tsx` | Shows properties table with `PropertyTypeBadge` → show `description` string instead |
| `graph-sidebar.tsx` | Renders `NodeSchemaItem` / `EdgeSchemaItem` — these delegate to `element-schema-item.tsx` |
| `node-schema-item.tsx` | Passes `properties` to `ElementSchemaItem` — needs update for `description` |
| `edge-schema-item.tsx` | Same |
| `graph-context.tsx` | `displayedNodeProperties` / `displayedEdgeProperties` system was for typed properties — needs rethinking |
| `graph.tsx` | Uses `displayedNodeProperties` for SpriteText labels — now properties are free-form keys |

---

## 6. Missing Backend Endpoints

### Required for Chat (Critical Path)

| Route | Method | Purpose | Implementation |
|-------|--------|---------|----------------|
| `POST /sessions` | Create | Create a new chat session for a graph | Wrap `session_service.create_session()` |
| `GET /sessions/{session_id}` | Read | Get session metadata | Wrap `session_service.get_session()` |
| `POST /sessions/{session_id}/close` | Update | Close a session | Wrap `session_service.close_session()` |
| `GET /sessions/{session_id}/messages` | Read | Get message history | Wrap `session_service.get_messages()` |
| `POST /graphs/{graph_id}/chat` | Create+Stream | **SSE bridge to AI agent** | New: opens gRPC stream to AI, returns SSE to browser |

The SSE bridge is the most important new endpoint. It:
1. Receives `{ session_id, content }` from the browser
2. Opens a gRPC `SendMessage` stream to the AI service
3. Receives `AgentEventProto` messages from the stream
4. Forwards them as SSE events to the browser

### SSE Event Types (from `AgentEventProto`)

```
event: text
data: {"content": "I'll create a Person node type..."}

event: tool_call
data: {"tool_call_id": "abc", "name": "create_node_schema", "arguments": "{...}"}

event: tool_result
data: {"tool_call_id": "abc", "content": "Created node schema 'Person'"}

event: done
data: {"summary": "Created 2 node types and 15 nodes"}

event: error
data: {"message": "Failed to create node schema"}
```

### Not Needed (Deferred)

- Graph update/delete → deferred
- Access CRUD → deferred (users see only own graphs)
- Bookmark/cheer endpoints → deferred (hardcoded `[]`)
- Search → deferred (hardcoded `[]`)

---

## 7. Step-by-Step Implementation Plan

### Phase 1 — Frontend: DTO Alignment & Dead Code Removal

**Goal**: Make the frontend compile cleanly against the real backend responses.

1. Delete `lib/api/dtos/property-schema-dto.ts`
2. Update `node-schema-dto.ts`: replace `properties: v.array(PropertySchemaDto)` → `description: v.string()`
3. Update `edge-schema-dto.ts`: same change
4. Delete `CreateNodeSchemaDto` and `CreateEdgeSchemaDto` from schema DTOs (or repurpose — no HTTP endpoint)
5. Delete `CreateGraphSchemaDto` from `graph-schema-dto.ts`
6. Delete `CreateGraphDataDto` from `graph-data-dto.ts`
7. Clean `dtos/index.ts` — remove all dead exports
8. Clean `types/index.ts` — remove dead type exports, update `NodeSchema`/`EdgeSchema` types
9. Delete dead `GraphService` methods: `generateSchema`, `generateData`, `createSchema`, `createData`
10. Delete all dead files listed in §4 (15 files)
11. Simplify `Action` enum — keep only `BUILD_WITH_AI`, `ASK_AI`, `FIND_NODE`, `FIND_PATH` (rest removed or disabled)
12. Update `graph-command.tsx` and `graph-menu.tsx` — remove dead action items
13. Update `graph-drawer.tsx` — remove dead action-to-component mappings
14. Update `element-schema-item.tsx` — show `description` instead of properties table
15. Update `node-schema-item.tsx` / `edge-schema-item.tsx` — pass `description` instead of `properties`
16. Update `graph-context.tsx` — simplify `displayedNodeProperties`/`displayedEdgeProperties` to work with free-form property keys
17. Update `graph.tsx` — adapt SpriteText to use free-form property keys

### Phase 2 — Backend: Session HTTP Handlers + SSE Chat Bridge

**Goal**: Give the web-ui a way to create sessions and chat with the AI agent.

18. Add session HTTP handlers in metadata:
    - `POST /sessions` → body: `{ graph_id }` → returns session metadata
    - `GET /sessions/{session_id}` → returns session metadata
    - `POST /sessions/{session_id}/close` → closes session
    - `GET /sessions/{session_id}/messages` → returns message history
19. Add SSE chat endpoint:
    - `POST /graphs/{graph_id}/chat` → body: `{ session_id, content }`
    - Opens gRPC `SendMessage` stream to AI service
    - Returns SSE stream of `AgentEventProto` events
20. Register all new routes in `router.rs`
21. Add AI gRPC client to metadata's `ApiState` (it currently only has knowledge client for gRPC)

### Phase 3 — Frontend: Chat Interface

**Goal**: Build the chat UI that drives the AI agent.

22. Create `lib/api/services/session-service.ts` — `createSession()`, `getSession()`, `closeSession()`, `getMessages()`
23. Create `lib/api/services/chat-service.ts` — `sendMessage()` using `EventSource`/`fetch` with SSE streaming
24. Create `contexts/chat-context.tsx` — manages session state, messages, streaming state, SSE connection
25. Build `components/graph/contents/chat-content.tsx` — the chat panel:
    - Message list (user messages + AI responses)
    - Streaming text display
    - Tool call indicators (show what the AI is doing)
    - Input box with send button
    - Session status (active/closed)
26. Wire `BUILD_WITH_AI` action → opens chat panel in `graph-drawer.tsx`
27. Wire `ASK_AI` action → same chat panel (possibly different system prompt or mode)

### Phase 4 — Frontend: Live Graph Updates

**Goal**: When the AI creates schemas/data, the 3D visualization updates in real-time.

28. After `done` SSE event, refetch schema + data in `GraphContext`
29. Optionally: update graph incrementally from tool_result events (optimistic updates)
30. Add toast notifications (shadcn `Sonner` or `Toast`) for AI actions: "Created node type 'Person'", "Inserted 15 nodes", etc.

### Phase 5 — Polish

31. Error handling — toast on API failures, graceful fallbacks
32. Loading states — skeleton loaders for chat messages, dashboard cards
33. Fix `settings-card.tsx` — at minimum make it display correctly
34. End-to-end test: create graph → open chat → tell AI to build a graph → see it render in 3D

---

## Appendix: File Inventory

### Files That Stay (No Changes Needed)

```
app/layout.tsx
app/page.tsx
app/globals.css
app/graph/[graph_id]/page.tsx (minor: remove dead imports if any)
app/graph/[graph_id]/layout.tsx
components/ui/* (all shadcn components)
components/dashboard/cards/accesses-card.tsx
components/dashboard/cards/bookmarks-card.tsx (keep stub)
components/dashboard/cards/cheers-card.tsx (keep stub)
components/dashboard/cards/search-card.tsx (keep stub)
components/dashboard/cards/settings-card.tsx (keep stub)
components/dashboard/contents/new-graph-content.tsx
components/dashboard/tables/data-table.tsx
components/dashboard/tables/graph-cols.tsx
components/graph/graph.tsx (needs update §Phase 1)
components/graph/graph-dialog.tsx
lib/api/proxy.ts
lib/api/provider.ts
lib/config.ts
lib/utils.ts
lib/api/dtos/access-dto.ts
lib/api/dtos/graph-dto.ts
lib/api/dtos/node-data-dto.ts
lib/api/dtos/edge-data-dto.ts
lib/api/dtos/property-data-dto.ts
lib/api/dtos/user-dto.ts
lib/api/dtos/utils-dto.ts
lib/api/services/user-service.ts
contexts/graph-context.tsx (needs update §Phase 1)
types/defaults.ts
```

### Files to Delete (15 files)

```
lib/api/dtos/property-schema-dto.ts
hooks/use-graph-schema-form.ts
hooks/use-element-schema-form.ts
hooks/use-element-data-form.ts
components/graph/contents/generate-content.tsx
components/graph/contents/new-node-schema-content.tsx
components/graph/contents/new-edge-schema-content.tsx
components/graph/contents/new-node-data-content.tsx
components/graph/contents/new-edge-data-content.tsx
components/graph/contents/new-element-schema-content.tsx
components/graph/contents/new-element-data-content.tsx
components/graph/forms/property-form.tsx
components/graph/badges/property-type-badge.tsx
components/graph/items/draft-element-schema.tsx
components/graph/items/draft-element-data.tsx
```

### Files to Create (5+ files)

```
lib/api/services/session-service.ts
lib/api/services/chat-service.ts
contexts/chat-context.tsx
components/graph/contents/chat-content.tsx
lib/api/dtos/session-dto.ts (if needed)
```

### Files to Modify (10+ files)

```
lib/api/dtos/node-schema-dto.ts (property → description)
lib/api/dtos/edge-schema-dto.ts (property → description)
lib/api/dtos/graph-schema-dto.ts (remove CreateGraphSchemaDto)
lib/api/dtos/graph-data-dto.ts (remove CreateGraphDataDto)
lib/api/dtos/index.ts (remove dead exports)
lib/api/services/graph-service.ts (remove 4 dead methods)
types/index.ts (remove dead types)
lib/actions.ts (simplify action list)
components/graph/graph-drawer.tsx (rewire actions)
components/graph/graph-command.tsx (simplify actions)
components/graph/graph-menu.tsx (simplify actions)
components/graph/graph-sidebar.tsx (description instead of properties)
components/graph/items/element-schema-item.tsx (description instead of properties table)
components/graph/items/node-schema-item.tsx (adapt)
components/graph/items/edge-schema-item.tsx (adapt)
contexts/graph-context.tsx (simplify displayed properties)
components/graph/graph.tsx (adapt labels)
```

### Backend Files to Create/Modify

```
metadata/src/presentation/http/session_handler.rs (new)
metadata/src/presentation/http/chat_handler.rs (new)
metadata/src/presentation/router.rs (add routes)
metadata/src/presentation/http/mod.rs (add modules)
metadata/src/presentation/state.rs (add AI gRPC client)
metadata/src/infrastructure/config/ (add AI service config)
```
