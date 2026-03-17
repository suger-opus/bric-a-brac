# AI Agent Design

## Overview

The system exposes a unified AI agent that manipulates a knowledge graph on behalf of users.
The agent can read from the graph, write to it, and answer questions — all in the same
conversation. Write access is enforced server-side based on the user's role on the graph.

**Model:** `openai/gpt-4.1` via OpenRouter.

---

## Agent Loop

A single LLM tool-calling loop:
1. Receive user message
2. Build system prompt (identity + graph schema + rules — see [System Prompt Design](#system-prompt-design))
3. Load conversation history from the database
4. Call the model with history + tools
5. **Stream** the model's response token-by-token to the client as `AgentThinking` events
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
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    graph_id    UUID NOT NULL REFERENCES graphs(id) ON DELETE CASCADE,
    user_id     UUID NOT NULL REFERENCES users(id),
    user_role   TEXT NOT NULL CHECK (user_role IN ('read', 'write')),
    status      TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'error')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

### `session_messages` table

```sql
CREATE TABLE session_messages (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id    UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    position      INTEGER NOT NULL,
    role          TEXT NOT NULL CHECK (role IN ('system', 'user', 'assistant', 'tool')),
    content       TEXT,
    tool_calls    JSONB,       -- assistant messages: array of {id, function: {name, arguments}}
    tool_call_id  TEXT,        -- tool messages: which tool_call this is the result of
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_session_messages_session ON session_messages(session_id, position);
```

`REFERENCES sessions(id) ON DELETE CASCADE` is a foreign key constraint: the database
guarantees every message belongs to an existing session. `ON DELETE CASCADE` means deleting
a session automatically deletes all its messages.

Each message is a row. This allows pagination, selective loading (last N messages for context
window management), and clean queries — no JSON surgery on a blob column.

### Lifecycle

- **Created** when the user opens a conversation on a graph
- **Active** while the user is sending messages
- **Completed** when: user/frontend sends `CloseSession`, agent calls `done` on the last
  exchange, or admin cleanup after N days of inactivity
- **Completed sessions** are read-only history. A new session can be opened on the same graph.

### Stateful server-side

The frontend sends **only the new user message**. The server:
1. Loads existing history from `session_messages`
2. Loads the current graph schema from metadata
3. Builds the system prompt
4. Runs the agent loop
5. Persists all new messages (user message + assistant responses + tool results)
6. Streams events to the client throughout

---

## Streaming

The agent streams events to the client via gRPC server-side streaming as it works.

```protobuf
message AgentEventProto {
  oneof event {
    AgentTextProto       text        = 1;  // streamed tokens from the model
    AgentToolCallProto   tool_call   = 2;  // tool name + arguments (emitted before execution)
    AgentToolResultProto tool_result = 3;  // what came back from the tool
    AgentDoneProto       done        = 4;  // summary, counts of writes
    AgentErrorProto      error       = 5;
  }
}
```

**LLM token streaming** is enabled from day one. OpenRouter supports SSE (`stream: true`).
The AI service consumes the SSE stream from OpenRouter, parses incremental tokens, and
forwards them as `AgentText` events on the gRPC stream. The user sees text appearing
word-by-word instead of waiting 5-10 seconds for a wall of text.

When the SSE stream completes, the full response is assembled and checked for tool calls.
If tool calls are present, they are executed and the loop continues.

---

## Tools

### Read tools (all users)

| Tool | Parameters | Returns |
|---|---|---|
| `search_nodes` | `query` (string), `node_type?` (string), `limit?` (int, default 10) | `NodeSummary[]` — id, type, key properties |
| `get_node` | `node_id` (string) | Full node with all properties |
| `get_neighbors` | `node_id` (string), `edge_type?` (string), `depth?` (int, default 1) | Subgraph: connected nodes + edges |
| `find_paths` | `from_id` (string), `to_id` (string), `max_depth?` (int, default 5) | `Path[]` — sequences of nodes and edges |

`search_nodes` uses **vector search** (embeddings). When `node_type` is omitted, it searches
across all node types in the graph and merges results by distance.

### Write tools (write-role users only)

| Tool | Parameters | Returns |
|---|---|---|
| `create_node` | `node_type` (string), `properties` (object) | `node_id` |
| `create_edge` | `edge_type` (string), `from_id` (string), `to_id` (string), `properties?` (object) | `edge_id` |
| `update_node` | `node_id` (string), `properties` (object) | Updated node |

### Session tools (all users)

| Tool | Parameters | Returns |
|---|---|---|
| `done` | `summary` (string) | Terminates the loop |

### Loop termination

The agent loop ends when:
1. The agent calls `done` — for document ingestion, the summary reports what was created
   and what couldn't be mapped to the schema
2. The agent responds with **text only, no tool calls** — for question answering, the text
   is the answer
3. **50 tool calls** reached — hard safety limit

### Rights enforcement

Enforced **server-side in the tool execution layer**, never by the prompt. If a read-only
user's agent calls `create_node`, the tool returns an error to the model:
`"Permission denied: you have read-only access to this graph."` The model then communicates
this to the user.

### Security: no LLM-generated queries

The LLM **never** writes Cypher or SQL. Every tool maps to a **predefined query template**
with parameterized inputs. The LLM provides parameter values (`node_type: "ESVhRs9k"`,
`properties: {name: "Marcus"}`), the server plugs them into hardcoded queries.

This is the same principle as parameterized SQL — injection is impossible by construction.

```
LLM decides:  create_node(node_type="ESVhRs9k", properties={name: "Marcus", age: 42})
Server runs:  UNWIND [{name: $name, age: $age}] AS props CREATE (n:ESVhRs9k) SET n = props
```

The node type label is validated against the schema before being interpolated into Cypher
(it's a label, not a parameter — Cypher doesn't support parameterized labels). The property
values go through neo4rs parameters as usual.

### Schema change handling

The agent has **no schema modification tools**. If data doesn't fit the current schema, the
agent completes what it can and reports mismatches in the `done` summary:

```
"Extracted 12 nodes, 8 edges. Could not map: 'patents' (no matching node type),
 'co-authored' relationship (no matching edge type). Consider adding these to the schema."
```

The user decides whether to update the schema and re-run. Schema changes are human-only:
- Add a node type
- Add an edge type
- Add an optional property to an existing type
- Renaming, deleting, or changing property types require careful migration — not exposed to the agent

---

## Ingestion Modes

| Mode | How it works |
|---|---|
| **From a document** | User sends document text in the `SendMessage` request → agent extracts entities and relationships → creates nodes and edges |
| **From model knowledge** | User asks "add what you know about Albert Einstein" → agent uses parametric knowledge → creates nodes and edges directly |
| **Question answering** | Agent calls `search_nodes`, `get_neighbors`, `find_paths` → reasons over results → responds in text |

Document text is sent **inline in the gRPC request** (option a). No file upload, no object
storage. The frontend reads the file and sends the text content.

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
the chat model — no additional configuration.

1536 dimensions, $0.02 per 1M tokens. Negligible cost.

### How embeddings are used

**At `create_node` time:**
1. Serialize the node to text: `"{human_type_name}: {prop1}={value1}, {prop2}={value2}"`
2. Call embedding model → `Vec<f32>`
3. Store the embedding as a property on the Memgraph node

**At `update_node` time:**
1. If any text-bearing property changed, re-compute the embedding
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

Created at **schema creation time** via `InitializeSchema` RPC — one index per node label.
Memgraph allows creating indexes on empty labels (no data needed yet).

Edges do **not** get vector indexes. Edge deduplication is structural: "does this edge type
already exist between these two specific nodes?"

### Embedding storage

Stored **in Memgraph**, on the node, alongside all other properties. One database = one source
of truth. No cross-database sync to maintain.

**Memory:** ~6KB per node (1536 × f32). At 100K nodes ≈ 600MB for embeddings. Acceptable.

**Scaling path** if memory becomes a concern:
1. **First:** reduce dimensions. `text-embedding-3-small` accepts a `dimensions` parameter.
   `dimensions: 512` → ~2KB/node (3× less) with <1% quality loss on short entity descriptions.
2. **Only if still needed:** externalize to a dedicated vector store.

---

## Session Tagging

Every node and edge created by the agent carries a `session_id` property in Memgraph.

- **Undo a session:** `MATCH (n { session_id: $id }) DETACH DELETE n`
- **Audit trail:** which session created what
- **Future branch system:** session writes = a branch. Merge = clear the session tag.

Pre-agent data and manually inserted data have `session_id: null`.

---

## System Prompt Design

The system prompt determines extraction quality, tool usage patterns, and response style.
It is **built dynamically** at the start of each message processing.

### Structure

```
[Identity]
You are a knowledge graph assistant. You help users build and query their knowledge graph
by reading documents, extracting information, and answering questions.

[Schema — injected fresh each message]
The graph has the following schema:

Node types:
- Person (label: ESVhRs9k): name (string, required), age (integer, optional)
  "Any human individual mentioned, including indirect references (he/she/they, roles)"
- Company (label: dudFcexv): name (string, required), industry (string, optional)
  "Organizations, corporations, and businesses"

Edge types:
- WorksAt (label: xR4kLm2p): from Person → Company, since (string, optional)
  "Employment, contract work, or affiliation — including implied affiliations"

[Capabilities — depends on user role]
You can search the graph, inspect nodes, and find paths.
{write_role → "You can also create and update nodes and edges."}
{read_role → "You cannot modify the graph."}

[Rules]
- Before creating a node, ALWAYS use search_nodes to check for existing duplicates.
  Only create if no match is found.
- Use node/edge type labels (like ESVhRs9k) in tool calls, not human-readable names.
- Be precise with property types. If the schema says integer, pass an integer, not a string.
- Process the ENTIRE document before finishing. Do not stop after the first few entities.
- If data does not fit the schema, skip it and report what you could not map in your summary.
- For questions, search the graph first, then reason. Do not fabricate information.
```

### Schema injection

The schema is loaded from the metadata service at the **start of each message** (not once
per session). If the user added a node type between messages, the agent sees it immediately.

Format: **human-readable text**, not JSON. Models follow natural language instructions more
reliably than raw schema definitions.

Contents:
- Node types: Memgraph label, properties (name, type, required/optional), semantic description
- Edge types: Memgraph label, source → target node types, properties, semantic description

---

## Changes Required

### Protos (`bric-a-brac-protos`)

**New RPCs — `Ai` service:**
```protobuf
rpc SendMessage(SendMessageRequest) returns (stream AgentEventProto);
```

**New RPCs — `Knowledge` service:**
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

**Keep:** `GenerateSchema`, `GenerateData` (deprecated, not yet removed).

---

### Knowledge service

| Handler | What it does |
|---|---|
| `InitializeSchema` | Creates vector indexes per node label |
| `InsertNode` | Single node insert with `session_id` + `embedding` property |
| `UpdateNode` | SET properties on existing node, update `embedding` if needed |
| `InsertEdge` | Single edge insert with `session_id` |
| `SearchNodes` | Vector nearest-neighbor on one label, or all labels if `node_type` omitted (query per label, merge by distance) |
| `GetNode` | Fetch one node by ID |
| `GetNeighbors` | Cypher traversal from a node, depth N |
| `FindPaths` | `shortestPath` between two nodes |

All queries use **predefined Cypher templates** with parameterized values. Node type labels
are validated against the schema before interpolation.

---

### Metadata service

- **Migration:** `sessions` + `session_messages` tables (schemas above)
- **Endpoints:** `POST /graphs/:id/sessions`, `GET /sessions/:id`, `PATCH /sessions/:id`,
  `GET /sessions/:id/messages`, `POST /sessions/:id/messages`
- **Hook:** `create_schema` now also calls `knowledge_client.initialize_schema(node_labels)`

---

### AI service

**Remove:** `DataService` / `GenerateData` handler (replaced by agent).

**Keep:** `SchemaService` / `GenerateSchema`.

**`OpenRouterClient` changes:**
- Tool calling support: `tools` array in request, `tool_calls` in response parsing
- SSE streaming: `stream: true`, parse `data:` lines incrementally
- Provider routing to avoid Amazon Bedrock (which silently ignores `response_format`)

**New `EmbeddingClient`:**
- Calls `openai/text-embedding-3-small` via OpenRouter `/api/v1/embeddings`
- Input: `String` → Output: `Vec<f32>`

**New `AgentService`:**
- Entry point: `send_message(session_id, user_message) → Stream<AgentEvent>`
- Loads session + schema from metadata
- Builds system prompt with schema injected
- Builds tool definitions filtered by user role
- Runs agent loop: stream model → detect tool calls → execute → append to history → repeat
- Persists all new messages on completion

**Tool implementations:**

| Agent tool | Server-side implementation |
|---|---|
| `search_nodes` | Embed query via `EmbeddingClient` → `KnowledgeClient::search_nodes` |
| `get_node` | `KnowledgeClient::get_node` |
| `get_neighbors` | `KnowledgeClient::get_neighbors` |
| `find_paths` | `KnowledgeClient::find_paths` |
| `create_node` | Embed node text → `KnowledgeClient::insert_node` (with embedding + session_id) |
| `create_edge` | `KnowledgeClient::insert_edge` (with session_id) |
| `update_node` | `KnowledgeClient::update_node` (re-embed if text properties changed) |
| `done` | End loop, stream summary |

**New gRPC clients:**
- `KnowledgeClient` (AI → Knowledge — new direction of communication)
- `MetadataClient` (AI → Metadata — for sessions and schema)

**New `SendMessage` handler** in the gRPC presentation layer.

---

### Web UI

- Chat interface: session creation, message input, streaming event display
- Tool call visualization: show what the agent is doing in real time
- Replaces the current "generate data from file" flow
- Schema generation flow unchanged

---

## Decisions

| Question | Decision |
|---|---|
| LLM model | `openai/gpt-4.1` via OpenRouter |
| Embedding model | `openai/text-embedding-3-small` via OpenRouter |
| Embedding storage | In Memgraph alongside nodes — single source of truth |
| LLM streaming | SSE from day one — tokens streamed to client in real time |
| Reasoning tokens | No — not useful for tool-calling agents, adds latency |
| Two-phase extraction | Future — start single-phase, add if quality is insufficient |
| Session ownership | Metadata service (Postgres) |
| Message storage | Separate `session_messages` table, one row per message |
| Document delivery | Text inline in `SendMessage` request |
| LLM-generated queries | **Never** — all tools use predefined parameterized templates |
| Vector index timing | Created at schema creation time |
| Edge vector indexes | No — edge dedup is structural |
| Schema changes from agent | None — agent reports mismatches in summary, human decides |
| Loop termination | `done` tool, text-only response, or 50 tool-call hard limit |
| Concurrency per graph | One active session at a time |
| Session closure | `done`, text-only final response, `CloseSession`, or inactivity cleanup |
| MCP Rust SDK | No |
| Provider routing | Avoid Amazon Bedrock (ignores structured output) |

---

## Future Work

- **Two-phase extraction** — separate "free extraction" LLM call before schema-mapping call,
  for better entity coverage on complex documents
- **Token truncation** — when conversation history exceeds the context window (~128K tokens),
  summarize older messages and keep recent ones in full
- **Branch/diff system** — session writes as a reviewable branch before merging into the graph
- **Web search tools** — `web_search` + `fetch_page` for information beyond the model's
  training cutoff
- **Multi-session concurrency** — allow multiple users to write to the same graph simultaneously
  (needs distributed locking or optimistic concurrency control)
- **Embedding dimension reduction** — switch to 512 dims if Memgraph memory becomes a concern
- **MCP exposure** — expose graph tools to external agents
