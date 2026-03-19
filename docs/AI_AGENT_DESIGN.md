# AI Agent Design

## Product Vision

The product is a conversational AI memory system. Users give data to the AI — documents,
topics, questions — and the AI stores its understanding in a knowledge graph. New data is
merged with existing data: the AI detects when new information overlaps with what's already
stored, links related entities, and resolves duplicates. The user never loses data context,
regardless of context window limitations.

The graph is the AI's structured long-term memory. Not a database the user manages — a
memory the AI builds and navigates.

**Model:** `openai/gpt-4.1` via OpenRouter.

---

## Architecture: Graph + Embeddings

Knowledge is stored as a **graph** (nodes, edges, typed relationships) with **vector
embeddings** on each node for semantic search. This was evaluated against:

- **Embeddings only** (vector store): no relationships, no structural traversal, no merging.
  Search works but the AI can't answer "what connects X to Y." Becomes Mem.ai with a
  different UI.
- **Triples** (RDF): maximally decomposed but harder to reason about when answering questions.
  No standard triple store with vector search.
- **Hybrid summaries**: natural language summary per node + graph. Duplicates data (the
  summary restates what the edges already express) and drifts over time.

Graph + embeddings is the right fit:
- **Graph** gives structure: entities, relationships, traversal, incremental merging.
- **Embeddings** give semantic search: fuzzy matching ("M. Aurelius" ↔ "Marcus Aurelius")
  and entity resolution.

---

## Lightweight Schemas (no property schemas)

Schemas define **what kinds of entities and relationships exist** in the graph. They are
lightweight — no property definitions, no type enforcement on property values.

### Node schema

| Field | Role |
|---|---|
| `key` | Memgraph label (immutable, 8-char alphanumeric, e.g. `ESVhRs9k`) |
| `name` | Human-readable name (e.g. "Person") — can be renamed |
| `description` | What this schema represents — guides the AI |
| `color` | UI rendering |

### Edge schema

| Field | Role |
|---|---|
| `key` | Memgraph relationship type (immutable) |
| `name` | Human-readable name (e.g. "WorksAt") |
| `description` | What this relationship means — guides the AI |
| `color` | UI rendering |

### What was removed

- **Property schemas** (`property_schemas` table, `PropertySchemaDto`, `PropertySchemaProto`,
  `PropertyTypeDto`, `PropertyMetadataDto`) — all deleted. Properties on nodes and edges
  are free-form key-value pairs. Memgraph is schema-free by design; we no longer fight it.
- **`GenerateSchema` RPC** — deleted. The AI creates schemas on its own as it processes data.
  No upfront schema design step.
- **Property validation** — deleted. The tool executor only validates that schemas exist
  (prevents typos), not that properties conform to a schema.

### Why

The old schema design assumed a user designs the schema upfront, then data conforms to it.
With an AI as the primary data creator, this creates constant friction:
- The AI encounters properties not in the schema → reports a mismatch → user edits schema
  → re-processes. Terrible UX.
- Users can't design property schemas before seeing the data.
- The AI is better at deciding what properties to store than a rigid schema.

Schemas still matter: without them, the AI creates inconsistent types ("Person" vs "person" vs
"Individual"). Schemas give consistency, vector index targets, and UI structure.

### How schemas are created

The AI creates schemas as needed during conversations. No upfront step.

1. AI receives a document or topic
2. AI analyzes what schemas of entities and relationships exist
3. AI checks existing schemas — reuses when possible
4. AI creates new schemas only when no existing schema fits
5. AI proceeds with extraction using those schemas

The **system prompt** guides this behavior:

> *"Before extracting entities, analyze what schemas of entities and relationships are present.
> Reuse existing schemas. Create new schemas only when no existing schema fits. Keep the number
> of schemas manageable — prefer reusing a broader schema over creating a narrow one."*

### Where schemas are stored

In **metadata Postgres** (control plane). Not in Memgraph (data plane).

- Schemas belong to a graph. Graphs are in Postgres. One transaction creates both.
- The web UI lists/edits schemas via metadata HTTP API — no extra hop.
- When the AI creates a schema mid-conversation: AI service → metadata gRPC
  (`CreateNodeSchema`) → knowledge gRPC (`InitializeSchema`). Clean separation.
- AI→metadata communication uses **gRPC** via `metadata.proto` (not HTTP).

### Write tools: `create_schema` and `create_edge_schema`

The AI has tools to create new schemas:

| Tool | Parameters | Returns |
|---|---|---|
| `create_schema` | `name` (string), `description` (string) | Schema with key |
| `create_edge_schema` | `name` (string), `description` (string) | Schema with key |

These call metadata gRPC to create the schema, then metadata calls knowledge gRPC to create
the vector index (for node schemas). The key is generated server-side. The AI uses the
returned key for subsequent `create_node` / `create_edge` calls.

---

## Node Structure

| Field | Role |
|---|---|
| `type` (key) | Memgraph label |
| `node_id` | Unique ID (UUIDv7) |
| `properties` | Free-form structured key-value facts |
| `embedding` | Vector of serialized properties |
| `session_id` | Provenance — which session created/last updated |

No description field. A node is fully described by its properties and its edges.

The embedding is computed from serialized properties:
`"Person: name=Marcus Aurelius, birth_year=121, occupation=Emperor"`

---

## Entity Resolution

**The central challenge.** When the AI processes new data, it must detect when new entities
overlap with existing ones and merge instead of duplicating.

**Built into `create_node` — automatic, not prompt-dependent.**

### Flow

1. AI calls `create_node(node_key, properties)`
2. Tool executor serializes properties → embeds them
3. Executor runs `search_nodes` with that embedding
4. **If similar nodes found:**
   - Fetches their properties + first-degree neighbors (live, not cached)
   - Returns everything to the AI as part of the tool response:
     ```
     "Created node abc123 (Person: Marcus Aurelius).
     Note: found 2 similar existing nodes:
     - node def456 (Person: Marcus Aurelius, Emperor of Rome) — similarity: 0.94
       Neighbors: wrote → Meditations, ruled → Roman Empire
     - node ghi789 (Person: Marcus, the lighthouse keeper) — similarity: 0.67
     If node def456 is the same entity, use update_node to merge them."
     ```
   - The AI decides: same entity → `update_node` to merge, different → keep both
5. **If no similar nodes:** creates directly

### Why automatic

Relying on the LLM to remember to call `search_nodes` before every `create_node` is fragile.
Deep into a long extraction, it may skip the check. By baking the similarity search into
`create_node`, duplicates are flagged even when the AI forgets. Entity resolution becomes a
structural feature, not a prompt behavior.

### Why neighbors matter

Vector similarity alone can't distinguish "John Smith at Acme Corp" from "John Smith at
NASA." Properties might not contain enough context. But neighbors do: the first John Smith
has edges to Acme Corp and Boston; the second to NASA and Houston. The AI reads the full
context (properties + neighbors) and decides.

---

## Graph Normalization (prompt-guided)

No hard property limits. The system prompt teaches the AI to think in graph terms:

- If a node would have many properties, some probably represent separate entities → split
  into multiple nodes with edges between them.
- If an edge needs many properties, it's probably a concept → promote to a node.
  (Employment with salary, start_date, title, benefits → Employment node connected to
  Person and Company.)
- Keep nodes focused on one concept. Same principle as code refactoring: a function doing
  a lot should be split into reusable functions.

---

## Agent Loop

A single LLM tool-calling loop:
1. Receive user message
2. Build system prompt (identity + schemas + rules — see [System Prompt Design](#system-prompt-design))
3. Load conversation history from the database
4. Call the model with history + tools
5. **Stream** the model's response token-by-token to the client as `AgentText` events
6. If the response contains tool calls → execute them server-side → send `ToolCall`/`ToolResult` events → append to history → go to step 4
7. If the response is text-only (no tool calls) → stream as final answer → **done**
8. If the `done` tool is called → stream summary → **done**
9. If 50 tool calls reached → force stop → stream error

The loop runs as an async Tokio task. One active session per graph at a time.

---

## Sessions

A session is a persistent conversation between a user and the agent on a specific graph.

**Owned by the metadata service** (Postgres).

### `sessions` table

```sql
CREATE TABLE sessions (
    session_id  UUID PRIMARY KEY NOT NULL,
    graph_id    UUID NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    status      VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT session_status_check CHECK (status IN ('active', 'completed', 'error'))
);
```

Note: no `user_role` on sessions — the user's role is already tracked in the `accesses` table.

### `session_messages` table

```sql
CREATE TABLE session_messages (
    message_id    UUID PRIMARY KEY NOT NULL,
    session_id    UUID NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    position      INTEGER NOT NULL,
    role          VARCHAR(20) NOT NULL,
    content       TEXT NOT NULL DEFAULT '',
    tool_calls    JSONB,
    tool_call_id  VARCHAR,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT message_role_check CHECK (role IN ('system', 'user', 'assistant', 'tool')),
    CONSTRAINT unique_session_position UNIQUE (session_id, position)
);
```

Each message is a row. This allows pagination, selective loading (last N messages for context
window management), and clean queries.

### Lifecycle

- **Created** when the user opens a conversation on a graph
- **Active** while the user is sending messages
- **Completed** when: user/frontend sends `CloseSession`, agent calls `done` on the last
  exchange, or admin cleanup after N days of inactivity
- **Completed sessions** are read-only history. A new session can be opened on the same graph.

### Stateful server-side

The frontend sends **only the new user message**. The server:
1. Loads existing history from `session_messages` (via metadata gRPC)
2. Loads current schemas from metadata (via metadata gRPC)
3. Builds the system prompt
4. Runs the agent loop
5. Persists all new messages (user message + assistant responses + tool results) via metadata gRPC
6. Streams events to the client throughout

---

## Streaming

The agent streams events to the client via gRPC server-side streaming as it works.

```protobuf
message AgentEventProto {
  oneof event {
    AgentTextProto       text        = 1;
    AgentToolCallProto   tool_call   = 2;
    AgentToolResultProto tool_result = 3;
    AgentDoneProto       done        = 4;
    AgentErrorProto      error       = 5;
  }
}
```

**LLM token streaming** is enabled from day one. OpenRouter supports SSE (`stream: true`).
The AI service consumes the SSE stream from OpenRouter, parses incremental tokens, and
forwards them as `AgentText` events on the gRPC stream.

---

## Tools

### Read tools (all users)

| Tool | Parameters | Returns |
|---|---|---|
| `search_nodes` | `query` (string), `node_key?` (string), `limit?` (int, default 10) | `NodeSummary[]` — id, schema, key properties |
| `get_node` | `node_id` (string) | Full node with all properties |
| `get_neighbors` | `node_id` (string), `edge_key?` (string), `depth?` (int, default 1) | Subgraph: connected nodes + edges |
| `find_paths` | `from_id` (string), `to_id` (string), `max_depth?` (int, default 5) | `Path[]` — sequences of nodes and edges |

`search_nodes` uses **vector search** (embeddings). When `node_key` is omitted, it searches
across all node schemas in the graph and merges results by distance.

### Write tools (write-role users only)

| Tool | Parameters | Returns |
|---|---|---|
| `create_schema` | `name` (string), `description` (string) | Schema with generated key |
| `create_edge_schema` | `name` (string), `description` (string) | Schema with generated key |
| `create_node` | `node_key` (string), `properties` (object) | Node (+ entity resolution warnings) |
| `create_edge` | `edge_key` (string), `from_id` (string), `to_id` (string), `properties?` (object) | Edge |
| `update_node` | `node_id` (string), `properties` (object) | Updated node |

### Session tools (all users)

| Tool | Parameters | Returns |
|---|---|---|
| `done` | `summary` (string) | Terminates the loop |

### Loop termination

The agent loop ends when:
1. The agent calls `done` — for document ingestion, the summary reports what was created
2. The agent responds with **text only, no tool calls** — for question answering
3. **50 tool calls** reached — hard safety limit

### Schema validation

The tool executor validates that the `node_key` / `edge_key` exists in metadata
before forwarding `create_node` or `create_edge` to the knowledge service. On failure,
it returns a descriptive error as the tool result so the LLM can self-correct:

    "Unknown node schema 'xInvalid'. Valid schemas: ESVhRs9k (Person), dudFcexv (Company)."

No property validation — properties are free-form.

Why validate in the AI service:
- Schemas live in metadata Postgres — knowledge has no access.
- Validation errors are conversational: the LLM retries, not crashes.

### Rights enforcement

Enforced **server-side in the tool execution layer**, never by the prompt. If a read-only
user's agent calls `create_node`, the tool returns an error to the model:
`"Permission denied: you have read-only access to this graph."`

### Security: no LLM-generated queries

The LLM **never** writes Cypher or SQL. Every tool maps to a **predefined query template**
with parameterized inputs. The LLM provides parameter values, the server plugs them into
hardcoded queries. Injection is impossible by construction.

The node schema key is validated against schemas before being interpolated into Cypher (it's a
label, not a parameter — Cypher doesn't support parameterized labels).

---

## Ingestion Modes

| Mode | How it works |
|---|---|
| **From a document** | User sends document text in `SendMessage` → agent analyzes schemas needed → creates schemas if missing → extracts entities and relationships → creates nodes and edges |
| **From model knowledge** | User asks "add what you know about Albert Einstein" → agent uses parametric knowledge → creates schemas + nodes + edges |
| **Question answering** | Agent calls `search_nodes`, `get_neighbors`, `find_paths` → reasons over results → responds in text |

Document text is sent **inline in the gRPC request**. No file upload, no object storage.

---

## Vector Search / Embeddings

### What an embedding is

An array of 1536 floats produced by passing text through a model trained to map meaning onto
geometry. Similar texts → numerically close arrays.

```
"Marcus the lighthouse keeper" → [0.21, -0.87, ...]
"the keeper of the light"      → [0.22, -0.85, ...]  ← very close
"banana bread recipe"          → [-0.91, 0.33, ...]  ← very far
```

### Embedding model

`openai/text-embedding-3-small` via **OpenRouter** (`/api/v1/embeddings`). Same API key as
the chat model. 1536 dimensions, $0.02 per 1M tokens.

### How embeddings are used

**At `create_node` time:**
1. Serialize the node to text: `"{type_name}: {prop1}={value1}, {prop2}={value2}"`
2. Call embedding model → `Vec<f32>`
3. **Run entity resolution** — search for similar existing nodes
4. Store the embedding as a property on the Memgraph node

**At `update_node` time:**
1. Re-compute the embedding from updated properties
2. Update the node in Memgraph

**At `search_nodes` time:**
1. Embed the agent's query string
2. Run Memgraph vector nearest-neighbor search
3. Return top N nodes with their properties

### Memgraph vector indexes

```cypher
CREATE VECTOR INDEX ON :ESVhRs9k(embedding)
OPTIONS { dimension: 1536, capacity: 10000, metric: "cos" }
IF NOT EXISTS;
```

Created via `InitializeSchema` RPC — one index per node schema. Called when a new node
schema is created (via `create_node_schema` in metadata service).

Edges do **not** get vector indexes. Edge deduplication is structural.

### Embedding storage

Stored **in Memgraph**, on the node, alongside all other properties. One database = one
source of truth.

**Memory:** ~6KB per node (1536 × f32). At 100K nodes ≈ 600MB. Acceptable.

---

## Session Tagging

Every node and edge created by the agent carries a `session_id` property in Memgraph.

- **Undo a session:** `MATCH (n { session_id: $id }) DETACH DELETE n`
- **Audit trail:** which session created what
- **Future branch system:** session writes = a branch. Merge = clear the session tag.

---

## System Prompt Design

The system prompt determines extraction quality, tool usage, and entity resolution quality.
It is **built dynamically** at the start of each message processing.

### Structure

```
[Identity]
You are a knowledge graph assistant. You help users build and query their knowledge graph
by reading documents, extracting information, and answering questions.

[Schemas — injected fresh each message]
The graph has the following schemas:

Node schemas:
- Person (key: ESVhRs9k): "Any human individual mentioned, including indirect references"
- Company (key: dudFcexv): "Organizations, corporations, and businesses"

Edge schemas:
- WorksAt (key: xR4kLm2p): "Employment, contract work, or affiliation"

[Capabilities — depends on user role]
You can search the graph, inspect nodes, and find paths.
{write_role → "You can also create schemas, create and update nodes, and create edges."}
{read_role → "You cannot modify the graph."}

[Rules]
- Before extracting from a document, analyze what schemas of entities and relationships exist.
  Reuse existing schemas. Create new schemas only when no existing schema fits.
- Keep the number of schemas manageable. Prefer a broader schema over a narrow one.
- Entity resolution is automatic: when you create a node, the system will search for similar
  existing nodes and show them to you. If a match is found, use update_node to merge.
- Use schema keys (like ESVhRs9k) in tool calls, not human-readable names.
- Process the ENTIRE document before finishing.
- Keep nodes focused on one concept. If a node has many properties, consider splitting it.
  If an edge needs many properties, promote it to a node.
- For questions, search the graph first, then reason. Do not fabricate information.
```

### Schema injection

Schemas are loaded from the metadata service (via gRPC `GetSchema`) at the **start of each
message** (not once per session). If the AI created a new schema in the previous message, it
sees it immediately.

Format: **human-readable text**, not JSON.

---

## Changes Required

### Protos (`bric-a-brac-protos`)

**`Ai` service:**
```protobuf
rpc SendMessage(SendMessageRequest) returns (stream AgentEventProto);
```

**`Knowledge` service (already implemented):**
```protobuf
rpc InitializeSchema(InitializeSchemaRequest) returns (InitializeSchemaResponse);
rpc InsertNode(InsertNodeRequest) returns (NodeDataProto);
rpc UpdateNode(UpdateNodeRequest) returns (NodeDataProto);
rpc InsertEdge(InsertEdgeRequest) returns (EdgeDataProto);
rpc SearchNodes(SearchNodesRequest) returns (SearchNodesResponse);
rpc GetNode(GetNodeRequest) returns (NodeDataProto);
rpc GetNeighbors(GetNeighborsRequest) returns (GetNeighborsResponse);
rpc FindPaths(FindPathsRequest) returns (FindPathsResponse);
```

**Removed:** `GenerateSchema` / `GenerateSchemaRequest` / `CreateGraphSchemaProto` and all
property schema proto messages.

---

### Knowledge service

| Handler | What it does |
|---|---|
| `InitializeSchema` | Creates vector indexes per node label |
| `InsertNode` | Single node insert with `session_id` + `embedding` property |
| `UpdateNode` | SET properties on existing node, update `embedding` |
| `InsertEdge` | Single edge insert with `session_id` |
| `SearchNodes` | Vector nearest-neighbor search |
| `GetNode` | Fetch one node by ID |
| `GetNeighbors` | Cypher traversal from a node, depth N |
| `FindPaths` | `shortestPath` between two nodes |

All queries use **predefined Cypher templates** with parameterized values.

---

### Metadata service

- **Migration:** `sessions` + `session_messages` tables added to existing single migration
  (no property_schemas, no user_role on sessions)
- **New proto:** `metadata.proto` — 8 RPCs for AI→metadata gRPC communication (session CRUD,
  schema CRUD, get schema)
- **gRPC server** running alongside HTTP via `tokio::select!`
- **Session management:** one active session per graph enforcement
- **Schema CRUD:** `create_node_schema`, `create_edge_schema` via gRPC — generates key/color,
  hooks into knowledge for vector index initialization
- **No HTTP endpoints for session or schema CRUD** — AI communicates via gRPC only

---

### AI service

**Removed:** `SchemaService` / `GenerateSchema` (no more schema generation).

**`OpenRouterClient` changes (implemented):**
- `chat_stream()` method: SSE streaming with tool call accumulation via `ToolCallBuilder`
- `Message` type with `system()`, `user()`, `assistant()`, `tool()` constructors
- `ToolDefinition`, `ToolCall`, `FunctionDefinition`, `FunctionCall`, `StreamChatResult` types
- reqwest `"stream"` feature for `bytes_stream()`

**`EmbeddingClient` (implemented):**
- `embed(texts: Vec<String>) → Vec<Vec<f32>>` (batch)
- `embed_one(text: String) → Vec<f32>`
- Same API key, configurable embedding model (`OPENROUTER_EMBEDDING_MODEL`)

**New `AgentService`** (Step 5) — the core agent loop with entity resolution built in.

**New tools:** `create_schema`, `create_edge_schema` alongside existing node/edge tools.

**gRPC clients:**
- `MetadataClient` (Step 5) — gRPC client using `metadata.proto` for session/schema management
- `KnowledgeClient` (Step 5) — gRPC client for knowledge service
- Config: `KnowledgeServerConfig` with `KNOWLEDGE_GRPC_SERVER_URL`

---

## Decisions

| Question | Decision |
|---|---|
| LLM model | `openai/gpt-4.1` via OpenRouter |
| Embedding model | `openai/text-embedding-3-small` via OpenRouter |
| Embedding storage | In Memgraph alongside nodes |
| LLM streaming | SSE from day one |
| Property schemas | **Removed** — properties are free-form |
| Schema generation | **Removed** — AI creates schemas on its own |
| Schema validation | Schema existence only — no property validation |
| Entity resolution | Automatic in `create_node` — search + neighbor context |
| Graph normalization | Prompt-guided, no hard limits |
| Batch inserts | **Open question** — start single, measure, then decide |
| Session ownership | Metadata service (Postgres) |
| Message storage | One row per message in `session_messages` |
| Document delivery | Text inline in `SendMessage` request |
| LLM-generated queries | **Never** — predefined parameterized templates |
| Edge vector indexes | No — edge dedup is structural |
| Schema creation | AI creates schemas mid-conversation via tools |
| Loop termination | `done` tool, text-only response, or 50 tool-call limit |
| Concurrency per graph | One active session at a time |
| AI→metadata | gRPC via `metadata.proto` (not HTTP) |

### Open: Batch inserts

Start with single inserts, no parallel tool calling. Measure actual latency on real
document ingestion. If round-trips are the bottleneck:
1. First explore parallel tool calling (model emits multiple calls, we execute concurrently)
2. If still not enough, add explicit batch tools (`create_nodes`, `create_edges`)

No new knowledge RPCs needed — batch tools loop over single inserts in the executor.

---

## Future Work

- **Two-phase extraction** — separate "free extraction" LLM call before graph-mapping,
  for better entity coverage on complex documents
- **Token truncation** — summarize older messages when history exceeds context window
- **Vector index cleanup** — reconcile existing indexes against current schemas, drop orphans
- **Branch/diff system** — session writes as a reviewable branch before merging
- **Web search tools** — `web_search` + `fetch_page` for info beyond training cutoff
- **Multi-session concurrency** — distributed locking or optimistic concurrency
- **Embedding dimension reduction** — 512 dims if memory becomes a concern
- **Deferred entity resolution** — let the AI flag uncertain matches (`potential_duplicate_of`)
  instead of forcing an immediate merge/keep decision. A post-processing step (or the AI
  at the end of the session) reviews flagged pairs with more context.
- **Chunked document ingestion** — pre-chunk large documents, store as chunk nodes,
  let the agent retrieve relevant chunks via search
- **User confirmation of new schemas** — before the AI creates a schema, ask the user first
- **MCP exposure** — expose graph tools to external agents
