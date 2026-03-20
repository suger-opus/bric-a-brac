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

## Positioning: Interactive Graph-Based RAG

### The core problem

The core problem isn't "RAG is hard" — plenty of tools make RAG easier (LangChain,
LlamaIndex, Pinecone, Weaviate). The problem is deeper: **humans can't see, verify, or correct what an AI "knows."**

Today, when you feed documents into a RAG pipeline, you're trusting a black box. You can't
inspect what was retained, what was lost in chunking, what connections exist between
concepts, or whether two chunks contradict each other. When the AI gives a wrong answer,
you can't trace *why* — was the data bad? Was retrieval bad? Was the chunk poorly split?
You have no tools to fix it.

Bric-a-brac makes AI knowledge **tangible and manipulable**. That's the real value
proposition.

### Why it's interesting

**1. The knowledge curation gap is real.** Every company building on RAG hits the same
wall: retrieval quality degrades over time, data gets stale, duplicates accumulate, and
nobody knows what's inside the vector store. The only fix is "re-ingest everything."
Bric-a-brac treats this as a first-class UX problem rather than an engineering problem.

**2. The graph structure is genuinely better for relational knowledge.** Factual,
relational, entity-heavy domains — legal, medical, academic research, competitive
intelligence, internal company knowledge — are poorly served by flat chunk-and-embed.
"Who reports to whom," "which contracts reference this clause," "how are these research
papers connected" — these questions need structure that vector search alone can't provide.

**3. The AI-as-builder pattern is novel.** Most tools are either "AI answers questions
over your data" (RAG) or "you manually build a knowledge base" (Notion, wikis, graph tools
like Neo4j Browser). Bric-a-brac merges both: the AI builds the graph *while* being
conversational, and the user steers it. The pre-check entity resolution is a good example
— it's collaborative knowledge construction.

### Traditional RAG limitations vs bric-a-brac

Traditional RAG follows a rigid pipeline: documents → chunks → embeddings → vector store →
retrieve → answer. Research identifies these well-known limitations:

1. **No web platform exists** to build and manage RAG interactively — pipelines are
   code-only.
2. **Data hygiene is critical** — quality depends entirely on chunking and cleaning, with
   no visibility into the vector store.
3. **Chunk strategy is a trade-off** — fixed-size, semantic, recursive splitting all lose
   information differently.
4. **Data cannot be updated live** — modifying a single fact means re-chunking and
   re-embedding entire documents.
5. **Relationships are lost** — similarity only, no structural queries.

| Pain point | Traditional RAG | Bric-a-brac |
|---|---|---|
| No web platform | Pipelines are code-only | Web UI with 3D graph viz + conversational chat |
| Data hygiene | Hope your chunks are clean | AI-assisted curation + entity resolution that flags duplicates |
| Chunk strategy | Fixed-size / semantic / recursive | Graph nodes with schemas — user and AI define granularity together |
| Can't update live | Re-index entire pipeline | Mutate nodes/edges in real-time, embeddings recompute on the fly |
| Data integrity | Black-box vector store | User sees and validates the graph directly |
| No relationships | Similarity only | Graph traversal + vector search — hybrid retrieval |

The key insight: **the "chunk" is a graph node.** Instead of opaque text fragments, each
unit of knowledge is a structured entity with typed properties, connected to other entities
by named relationships. The chunk strategy problem disappears — the granularity is defined
by schemas, not by a splitting algorithm.

### Risks and honest challenges

**1. "Who is the user?" is hard.** Knowledge graphs appeal to technical users who
understand entities and relationships. But the chat-first UX targets non-technical users.
These audiences want different things. A researcher wants precision and control. A casual
user wants to dump documents and get answers. If we try to serve both, we might serve
neither well. The 3D graph visualization is powerful, but does a marketing manager actually
want to see a force-directed graph of their brand strategy?

**2. Manual curation doesn't scale.** The entity resolution pre-check is smart for small
graphs. But at 10,000 nodes, will users still review duplicate candidates? At 100,000? The
strength of traditional RAG — even with its flaws — is that it's hands-off. Our product's
strength (user control) becomes a weakness when volume exceeds the user's attention. We'll
eventually need an "auto-pilot" mode where the AI resolves duplicates without asking — and
then we're back to a trust problem.

**3. Ingestion is the cold-start bottleneck.** Right now, knowledge enters through
conversation. That's fine for incremental additions, but a user with 500 documents wants
bulk import. Building a document → graph pipeline (see [Document Chunking Pipeline](#future-document-chunking-pipeline))
is essentially building a traditional RAG chunker anyway, just with graph output. The
complexity removed from the query side reappears on the ingestion side.

**4. The competitive landscape is moving fast.** Google NotebookLM does "structured
understanding of documents" with a conversational UI. Notion AI is adding knowledge graphs.
Mem.ai, Obsidian + AI plugins, Capacities — all targeting AI-assisted knowledge management.
None of them do graph-based RAG with user-controlled entity resolution, but the window for
differentiation narrows quickly. Speed to market matters.

**5. Graph maintenance is unsolved UX.** Graphs get messy. Nodes accumulate disconnected.
Schemas drift. Relationships become stale. Traditional databases have decades of tooling
for maintenance. Graph databases have almost none, especially for LLM-generated content.
We'll need to build graph hygiene tools — orphan detection, schema conformance checks,
relationship consistency validation — that don't exist yet.

### The bottom line

The insight that **AI knowledge should be transparent, structured, and user-editable** is
sound — it's the natural next step after the first wave of "just throw documents at a vector
store" RAG products disappoints users who need accuracy and control.

The risk isn't that the problem is uninteresting. The risk is **scope**. We're building a
knowledge graph database, a conversational AI agent, a real-time graph visualization, and
an entity resolution system — simultaneously. Each of those is a product by itself. The
discipline will be in saying no until the core loop — "talk to AI → graph grows → graph is
useful → ask questions → get good answers" — is genuinely excellent.

If that core loop works reliably, there is something valuable here. If it's 80% there,
it'll feel like a demo.

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

### Why no property schemas

Properties on nodes and edges are free-form key-value pairs. Memgraph is schema-free by
design. With an AI as the primary data creator, enforcing property schemas creates constant
friction:
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
4. **If similar nodes found (distance < 0.3):**
   - Fetches their properties + first-degree neighbors (live, not cached)
   - **The node is NOT created.** Instead, the tool returns the candidates:
     ```json
     {
       "created": false,
       "reason": "Similar nodes already exist. Use update_node to merge...",
       "similar_nodes": [
         {
           "node_data_id": "def456",
           "key": "person",
           "properties": { "name": "Marcus Aurelius", "title": "Emperor" },
           "distance": 0.06,
           "neighbors": { "node_count": 4, "edge_count": 3 }
         }
       ]
     }
     ```
   - The AI decides: same entity → `update_node` to merge, different → `create_node`
     again with `force: true`
5. **If no similar nodes:** creates directly
6. **If `force: true` is passed:** skips the check and creates unconditionally

### Why pre-check (not create-first)

The previous approach was create-first: always insert the node, then warn the LLM to
merge and delete the duplicate. This had critical failure modes:

- If the agent errors out, hits the token limit, or the user cancels mid-resolution,
  **orphan duplicates remain forever** in the graph.
- Every merge required 3 tool calls (`create_node` → `update_node` → `delete_node`)
  instead of 1 (`update_node`).

With pre-check, zero data is written until the LLM commits to a decision. The merge case
is a single `update_node` call. The "genuinely new" case costs one extra call
(`create_node` with `force: true`), but this is the less common path.

### Node merge workflow

When the AI determines that the new entity matches an existing node (B):

1. `update_node(B, {merged properties})` — merges new information into the existing node.
   Adds new property keys, overrides existing ones. The embedding is recomputed.

When the AI determines the entity is genuinely new despite candidates being shown:

1. `create_node(node_key, properties, force=true)` — creates the node, skipping the
   duplicate check.

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
9. If 200 tool calls reached → force stop → stream error

The loop runs as an async Tokio task. One active session per graph at a time.

### Concurrency control

A `tokio::sync::Semaphore` (20 permits) caps concurrent LLM calls across all agent sessions.
If 20 agents are already calling OpenRouter simultaneously, the 21st will wait until one
finishes. This prevents overwhelming the LLM provider with too many simultaneous requests,
which would cause 429 rate-limit errors.

### Retry & resilience

- **gRPC calls** (to knowledge and metadata services) use automatic retry with exponential
  backoff (3 attempts, retries on `Unavailable` and `DeadlineExceeded`). gRPC channels use
  `connect_lazy()` for automatic HTTP/2 reconnection.
- **HTTP calls** (to OpenRouter for chat and embeddings) use automatic retry with exponential
  backoff (3 attempts, retries on 429/5xx and transient network errors, respects
  `Retry-After` headers).

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

The user's role is not stored on the session — it's derived from the `accesses` table via
a JOIN when loading the session.

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

### Session concurrency enforcement

One active session per graph. Enforced in `session_service.create_session()` inside a
Postgres transaction:

1. `BEGIN`
2. `has_active_session(graph_id)` — `SELECT EXISTS(... WHERE status = 'active')`
3. If true → return error: "An active session already exists for this graph"
4. If false → `INSERT INTO sessions`
5. `COMMIT`

The transaction ensures no race condition between the check and the insert. If a second
user tries to open a chat on the same graph while a session is active, the frontend
receives an error and should display it.

### Stale session problem

**Problem:** The frontend stores `sessionId` in React state (`useState`). If the user
refreshes the page, the state is lost — but the session stays `active` in Postgres.
When the user sends a new message, the frontend tries to create a new session, which fails
("already active"). The user is blocked.

**Solution — recover the active session on page load:**

1. Add a new endpoint: `GET /graphs/{graph_id}/active-session`
   - Returns the active session for the graph (if any), or 404.
   - Query: `SELECT * FROM sessions WHERE graph_id = $1 AND status = 'active' LIMIT 1`
2. The `ChatPanel` component calls this endpoint on mount (when `sessionId` is null).
   If an active session exists, it sets `sessionId` to the returned session and loads
   its message history via `GET /sessions/{session_id}/messages`.
3. If no active session exists, the frontend proceeds normally (lazy creation on first
   message).

**Not yet implemented.** Currently the frontend does not recover sessions on page load. If
a page refresh happens during an active session, the user must wait for the session to
expire or be cleaned up manually. This is the next feature to implement.

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

OpenRouter supports SSE (`stream: true`). The AI service consumes the SSE stream from
OpenRouter, parses incremental tokens, and forwards them as `AgentText` events on the
gRPC stream.

---

## User Authentication & Resource Protection

### Authentication (future — not yet implemented)

Users will authenticate via OAuth2 social login:
- **Providers:** Google (Gmail), Reddit
- **Flow:** Standard OAuth2 authorization code flow → the web UI redirects to the provider,
  receives a token, exchanges it for a session cookie or JWT.
- **User creation:** On first login, a `users` row is created in metadata Postgres. Subsequent
  logins match by provider + provider ID.
- **Session token:** After authentication, every API request carries a session token (cookie
  or `Authorization: Bearer` header). The metadata HTTP layer validates the token and extracts
  the `user_id` before processing the request.

Currently, `user_id` is passed directly in requests (no authentication layer). Adding OAuth
will wrap existing endpoints with a middleware that resolves `user_id` from the token.

### Role-Based Access Control (RBAC)

Every graph has an access control list stored in the `accesses` table:

```sql
CREATE TABLE accesses (
    user_id   UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    graph_id  UUID NOT NULL REFERENCES graphs(graph_id) ON DELETE CASCADE,
    role      role_type NOT NULL DEFAULT 'None',
    PRIMARY KEY (user_id, graph_id)
);

CREATE TYPE role_type AS ENUM ('Owner', 'Admin', 'Editor', 'Viewer', 'None');
```

**Role hierarchy:**

| Role | Read graph | Write data | Manage schema | Share graph | Delete graph |
|---|---|---|---|---|---|
| **Owner** | Yes | Yes | Yes | Yes | Yes |
| **Admin** | Yes | Yes | Yes | Yes | No |
| **Editor** | Yes | Yes | No | No | No |
| **Viewer** | Yes | No | No | No | No |
| **None** | No | No | No | No | No |

- **Owner** is assigned automatically when a user creates a graph. Cannot be transferred.
- **Admin** can invite other users and assign roles (except Owner).
- **Editor** can create/update/delete nodes, edges, and schemas via the AI agent.
- **Viewer** can query the graph (search, get_node, get_neighbors, find_paths) but cannot
  modify it.
- **None** is the default when no access record exists — no access at all.

### Resource protection: how roles are enforced

Roles are enforced at **two layers** — defense-in-depth:

#### Layer 1 — Tool filtering (AI service)

When the agent loop starts, the user's role is loaded from the session (which JOINs the
`accesses` table). Based on the role, the agent receives different tool sets:

- **Owner / Admin / Editor:** read tools + write tools + session tools
- **Viewer / None:** read tools + session tools only

The LLM literally does not see write tools in its tool list. It cannot call what it doesn't
know exists.

#### Layer 2 — Tool executor guard (AI service, defense-in-depth)

Even if the LLM somehow calls a write tool (e.g., through prompt injection or a bug in tool
filtering), the tool executor independently checks the user's role before executing:

```
if WRITE_TOOLS.contains(tool_name) && !is_write_role(user_role) {
    return "Permission denied: role 'Viewer' cannot use tool 'create_node'."
}
```

This is a server-side guard — not prompt-based, not bypassable by the LLM.

#### Layer 3 — Graph-level isolation (knowledge service)

Every Memgraph query is scoped to a `graph_id`. A user's session is bound to a specific
graph. There is no mechanism (in the tool executor or knowledge client) to query a different
graph than the one the session was created for. Cross-graph data leakage is impossible by
construction.

### Security: no LLM-generated queries

The LLM **never** writes Cypher or SQL. Every tool maps to a **predefined query template**
with parameterized inputs. The LLM provides parameter values, the server plugs them into
hardcoded queries. Injection is impossible by construction.

The node schema key is validated against known schemas before being interpolated into Cypher
(it's a label, not a parameter — Cypher doesn't support parameterized labels).

### Public graphs

Graphs can be made public by their admin. Public graphs are read-only for unauthenticated
users — they can browse the graph, search nodes, and view data. They cannot modify anything
or create sessions with write access.

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

| Tool | Parameters | Returns | Status |
|---|---|---|---|
| `create_schema` | `name` (string), `description` (string) | Schema with generated key | **Implemented** |
| `create_edge_schema` | `name` (string), `description` (string) | Schema with generated key | **Implemented** |
| `create_node` | `node_key` (string), `properties` (object), `force?` (boolean) | Node or similar-node candidates | **Implemented** |
| `create_edge` | `edge_key` (string), `from_id` (string), `to_id` (string), `properties?` (object) | Edge | **Implemented** |
| `update_node` | `node_id` (string), `properties` (object) | Updated node | **Implemented** |
| `update_edge` | `edge_id` (string), `properties` (object) | Updated edge | **Not implemented** |
| `remove_properties` | `element_id` (string), `keys` (string[]) | Updated node or edge | **Not implemented** |
| `delete_node` | `node_id` (string) | Confirmation (detach-deletes edges) | **Implemented** |
| `delete_edge` | `edge_data_id` (string) | Confirmation | **Implemented** |

#### `update_node` / `update_edge` semantics

**Merge, not replace.** `update_node` is implemented. `update_edge` is not yet implemented.

For each key in the provided `properties` object:
- If the key exists on the node/edge → override the value.
- If the key does not exist → add the property.

Properties not mentioned in the call are left untouched. To remove a property, use
`remove_properties`.

After an `update_node`, the embedding is recomputed from the merged property set.

#### `remove_properties`

Removes one or more properties from a node or edge by key. Accepts a list of keys to
enable removing multiple properties in a single tool call (saves round-trips). The element
is identified by `element_id` which can be either a node ID or edge ID.

After removing properties from a node, the embedding is recomputed.

#### `delete_node`

Deletes a node and all its edges (`DETACH DELETE` in Memgraph). Used during entity
resolution when the AI decides two nodes are the same entity — after merging properties
into the surviving node, the duplicate is deleted.

#### `delete_edge`

Deletes a single edge by its `edge_data_id`. Unlike `delete_node`, this does not cascade
— only the targeted relationship is removed. Useful when the AI needs to correct a
wrong relationship or when the user explicitly asks to remove a connection.

#### Edge uniqueness

**Enforced.** `create_edge` uses `MERGE` (not `CREATE`) with `ON CREATE SET` / `ON MATCH
SET` in Memgraph. A relationship between two nodes is unique per edge schema key: two nodes
can have multiple relationships, but only one per key. If the AI calls `create_edge` with a
(from, to, key) triple that already exists, Memgraph matches the existing edge and updates
its properties (upsert). No duplicate edges are created.

### Session tools (all users)

| Tool | Parameters | Returns |
|---|---|---|
| `done` | `summary` (string) | Terminates the loop |

### Loop termination

The agent loop ends when:
1. The agent calls `done` — for document ingestion, the summary reports what was created.
2. The agent responds with **text only, no tool calls** — for question answering. The
   server automatically emits a `Done` event after the text.
3. **200 tool calls** reached — hard safety limit. The server emits an `Error` event.

The gRPC stream **always** terminates with either a `Done` or `Error` event. The frontend
relies on this: it resets streaming state and triggers `refetch()` on `done`, and shows an
error message on `error`. There is no scenario where the stream ends silently.

### Schema validation

The tool executor validates that the `node_key` / `edge_key` exists in metadata
before forwarding `create_node` or `create_edge` to the knowledge service. On failure,
it returns a descriptive error as the tool result so the LLM can self-correct:

    "Unknown node schema 'xInvalid'. Valid schemas: ESVhRs9k (Person), dudFcexv (Company)."

No property validation — properties are free-form.

Why validate in the AI service:
- Schemas live in metadata Postgres — knowledge has no access.
- Validation errors are conversational: the LLM retries, not crashes.

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
3. **Run entity resolution** — search for similar existing nodes; if found, return
   candidates without creating (pre-check approach)
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
CREATE VECTOR INDEX idx_ESVhRs9k_embedding ON :ESVhRs9k(embedding)
WITH CONFIG { "dimension": 1536, "capacity": 10000, "metric": "cos" };
```

Created via `InitializeSchema` RPC — one index per node schema. Called when a new node
schema is created (via `create_node_schema` in metadata service).

**Querying** uses `vector_search.search(index_name, result_set_size, query_vector)`.
`result_set_size` must be a literal integer (not a Cypher parameter).

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
- Entity resolution is automatic: when you create a node, the system searches for similar
  existing nodes. If a match is found, the node is NOT created — you get the candidates back.
  Use update_node to merge into an existing node, or call create_node with force=true.
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

## Service Architecture

Three microservices communicate via gRPC:

```
Web UI → Metadata service (HTTP REST + SSE) — graphs, users, sessions, chat
           ├── → Knowledge service (gRPC) — for vector index initialization
           └── → AI service (gRPC stream) — chat SSE bridge
                   ├── → Knowledge service (gRPC) — graph storage, embeddings
                   └── → Metadata service (gRPC)  — schemas, sessions
```

The web UI only talks HTTP to metadata. The SSE chat endpoint in metadata bridges
to the AI service's gRPC stream.

| Service | Port | Transport | Database |
|---|---|---|---|
| **Knowledge** | 50051 | gRPC | Memgraph |
| **Metadata** | 50052 (gRPC) + HTTP | gRPC + HTTP (Axum) | PostgreSQL |
| **AI** | 50053 | gRPC | None (stateless) |

- **AI → Metadata:** gRPC via `metadata.proto` (session management, schema CRUD)
- **AI → Knowledge:** gRPC via `knowledge.proto` (graph operations)
- **Metadata → Knowledge:** gRPC for vector index initialization on schema creation
- **Web UI → Metadata:** HTTP REST + SSE for all user-facing operations including chat

### Layering rules

These apply across the entire Rust codebase:

- **Handlers** extract request data, call services, return responses. No business logic.
- **Services** take and return **DTOs**. Convert DTO↔Model internally.
- **Repositories** take and return **Models**.
- **gRPC clients** take and return **DTOs**.
- **Proto↔Dto** conversions live in dto files as `From` trait impls.
- **Dto↔Model** conversions live in dto files as `From` trait impls.
- Zero `.expect()` / `.unwrap()` in production code. gRPC client constructors return
  `anyhow::Result<Self>`.
- Type-safe IDs via `id!()` macro (e.g. `SessionIdDto`, `GraphIdDto`).

### Metadata HTTP routes

```
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

GET  /graphs/{graph_id}/active-session  → get active session (or 204)
POST /graphs/{graph_id}/chat            → SSE chat bridge (body: { session_id, content })
```

### Chat SSE bridge (metadata → AI)

The `POST /graphs/{graph_id}/chat` endpoint bridges HTTP to gRPC:

1. Receives `{ session_id, content }` from the browser
2. Calls `ai_service.chat()` which opens a gRPC `SendMessage` stream to the AI service
3. Maps each `AgentEventProto` from the stream to an SSE event
4. Streams events to the browser with `event:` + `data:` format

SSE event types:

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

## Web UI

### Stack

- **Next.js 16.1.1** + **React 19.2.3** + **TypeScript 5** (React Compiler enabled)
- **TailwindCSS v4** + **shadcn/ui** (Radix primitives)
- **Valibot v1.2.0** for DTO runtime validation
- **Mande v2.0.9** as HTTP client
- **react-force-graph-3d** + **three-spritetext** for 3D graph visualization
- **Sonner** for toast notifications
- Config: `NEXT_PUBLIC_API_URL` env var pointing to metadata HTTP

### API client (`lib/api/client.ts`)

Typed `get<T>(path, schema)` and `post<T>(path, body, schema)` wrapping Mande. Every
response is validated at runtime with Valibot `safeParse`. Failed requests show a toast.
Hardcoded `user_id` header (no auth yet).

Chat uses raw `fetch()` with manual SSE parsing (not EventSource — needs POST body).
Returns an `AbortController` for cancellation.

### Page structure

**Dashboard** (`app/page.tsx`) — list + create graphs. Stub cards for search, bookmarks,
cheers, settings.

**Graph page** (`app/graph/[graph_id]/page.tsx`):

```
GraphProvider (context: graphId, metadata, schema, data, processedData, refetch)
├── Graph (3D force graph — react-force-graph-3d, SpriteText labels, schema colors)
└── GraphSidebar
    ├── Header (name, owner, dates, privacy badge, node/edge counts)
    ├── Tabs
    │   ├── "Chat" (default) → ChatPanel
    │   └── "Schema" → Collapsible NodeSchemaItem[] + EdgeSchemaItem[]
    └── Footer (exit to home)
```

**ChatPanel** — lazy session creation on first message, SSE streaming with blinking cursor,
tool call/result visualization, auto-scroll. Calls `refetch()` on "done" event to update
the 3D graph. Toast notifications on tool results and errors.

**GraphContext** — parallel fetch of metadata + schema + data. `refetch()` increments a
`fetchTrigger` counter that re-triggers the fetch effect. Processes graph data by mapping
schema colors onto nodes and edges.

---

## Decisions

| Question | Decision |
|---|---|
| LLM model | `openai/gpt-4.1` via OpenRouter |
| Embedding model | `openai/text-embedding-3-small` via OpenRouter |
| Embedding storage | In Memgraph alongside nodes |
| LLM streaming | SSE from day one |
| Property schemas | None — properties are free-form |
| Schema validation | Schema existence only — no property validation |
| Entity resolution | Pre-check in `create_node` — blocks creation if similar nodes exist, returns candidates with neighbor context. `force=true` to override. |
| Graph normalization | Prompt-guided, no hard limits |
| `update_node` semantics | Merge (override existing, add new) — not replace |
| Batch inserts | Start single, measure, then decide |
| Session ownership | Metadata service (Postgres) |
| Message storage | One row per message in `session_messages` |
| Document delivery | Text inline in `SendMessage` request |
| LLM-generated queries | **Never** — predefined parameterized templates |
| Edge vector indexes | No — edge dedup is structural (unique per type between two nodes) |
| Edge uniqueness | One edge per (from, to, edge_key) triple — upsert on conflict |
| Schema creation | AI creates schemas mid-conversation via tools |
| Loop termination | `done` event always emitted — text-only triggers auto-done, or 200-call error |
| Concurrency per graph | One active session at a time (transaction-guarded) |
| AI→metadata | gRPC via `metadata.proto` (not HTTP) |
| LLM concurrency | Semaphore (20 permits) across all sessions |
| gRPC retry | 3 attempts, exponential backoff on Unavailable/DeadlineExceeded |
| HTTP retry | 3 attempts, exponential backoff on 429/5xx, respects Retry-After |
| gRPC channels | `connect_lazy()` for automatic HTTP/2 reconnection |
| Role enforcement | Two layers: tool filtering + tool executor guard |

### Open: Batch inserts

Start with single inserts, no parallel tool calling. Measure actual latency on real
document ingestion. If round-trips are the bottleneck:
1. First explore parallel tool calling (model emits multiple calls, we execute concurrently)
2. If still not enough, add explicit batch tools (`create_nodes`, `create_edges`)

No new knowledge RPCs needed — batch tools loop over single inserts in the executor.

---

## Implementation Status

What's **built and compiles** vs what's **design-only** (documented above but not in code).

### Implemented

- **Metadata service** — full HTTP API (users, graphs, sessions, schema, data, chat SSE
  bridge), gRPC server, Postgres repositories, migrations. `cargo check` clean.
- **Knowledge service** — gRPC server, Memgraph repositories (nodes, edges, schemas),
  vector index management. `cargo check` clean.
- **AI service** — gRPC server, agent loop, system prompt builder, tool executor, streaming,
  LLM semaphore, HTTP retry, gRPC retry. `cargo check` clean.
- **12 tools**: `search_nodes`, `get_node`, `get_neighbors`, `find_paths`, `create_schema`,
  `create_edge_schema`, `create_node`, `create_edge`, `update_node`, `delete_node`,
  `delete_edge`, `done`.
- **`delete_node` tool** — full pipeline: tool definition, tool executor handler, AI gRPC
  client, knowledge gRPC handler, repository (`DETACH DELETE`).
- **`delete_edge` tool** — full pipeline: tool definition, tool executor handler, AI gRPC
  client, knowledge gRPC handler, repository (`DELETE r` by `edge_data_id`).
- **Entity resolution** — pre-check in `create_node`: similar nodes block creation and
  return candidates with neighbor context. `force=true` to override.
- **Edge uniqueness enforcement** — `create_edge` uses `MERGE` with `ON CREATE SET` /
  `ON MATCH SET` to upsert on (from, to, edge_key) triples. No duplicate edges.
- **Session concurrency** — one active session per graph, transaction-guarded.
- **Active session recovery** — `GET /graphs/{graph_id}/active-session` endpoint returns
  the current active session (or 204). Frontend `ChatPanel` recovers session + messages
  on mount.
- **Session close on unmount** — `ChatPanel` calls `POST /sessions/{id}/close` when the
  component unmounts (navigation away or page close).
- **Streaming cancel button** — "Stop" button replaces "Send" while streaming. Aborts the
  SSE connection and resets streaming state.
- **Chat history recovery** — on page load, if an active session exists, previous messages
  are loaded from `GET /sessions/{id}/messages` and displayed in the chat panel.
- **Web UI** — dashboard, graph page (3D force graph), sidebar with Chat + Schema tabs,
  SSE streaming chat, toast notifications. `tsc --noEmit` clean.
- **Role-based tool filtering** — read-only vs write tools based on user role.
- **Tool executor guard** — defense-in-depth role check before executing write tools.
- **Schema validation** — node_key/edge_key validated against metadata before tool execution.

### Not yet implemented

- **2 tools**: `update_edge`, `remove_properties` — designed above but not
  yet in `tools.rs` or `tool_executor.rs`.
- **Authentication** — OAuth2 login. Currently `user_id` is hardcoded / passed as a header.
- **Loading skeletons** — skeleton loaders for dashboard cards and graph page.
- **Incremental graph updates** — update the 3D graph from `tool_result` events in real
  time instead of waiting for "done".

---

## Remaining Work

See [Implementation Status](#implementation-status) for what's built vs what's not.

- **Implement missing tools** — `update_edge`, `remove_properties` (tool
  definitions + tool executor handlers + knowledge gRPC calls). `delete_node` and
  `delete_edge` are already implemented.
- **Schema colors** — the AI already returns colors when creating schemas. No additional
  work needed.
- **Delete graph** — `DELETE /graphs/{graph_id}` endpoint. CASCADE delete in Postgres
  (removes schemas, sessions, accesses) + drop corresponding nodes/edges in Memgraph.
  Currently no way to delete a graph.
- **Incremental graph updates** — optionally update 3D graph from `tool_result` events
  instead of waiting for "done" (optimistic updates)

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
- **User confirmation of new schemas** — before the AI creates a schema, ask the user first
- **MCP exposure** — expose graph tools to external agents

---

## Future: Document Chunking Pipeline

**Problem:** Currently, documents are sent inline as a single user message. This works for
small-to-medium documents but will fail for large ones (token limits, degraded extraction
quality on long contexts, cost).

### Proposed Architecture

**Two-stage approach: chunk → extract per chunk → deduplicate**

#### Stage 1 — Chunking (AI service, before agent loop)

Split the incoming document into overlapping chunks:

- **Chunk size:** ~2000 tokens (configurable)
- **Overlap:** ~200 tokens (10%) to avoid splitting entities at boundaries
- **Strategy:** Prefer splitting at paragraph/section boundaries when possible
  (detect markdown headers, double newlines). Fall back to token-based splitting.
- Each `Chunk` carries `text`, `index`, and `byte_range` for traceability.

Token counting should use a lightweight tokenizer (e.g., `tiktoken-rs`) rather than
character-based heuristics.

#### Stage 2 — Per-chunk extraction (agent loop, modified)

For each chunk, run the agent loop with the chunk as the user message. The system prompt
instructs the LLM that this is chunk N of M and to extract entities/relationships.

Two options for execution:
1. **Sequential chunks** — simpler, chunks processed one after another in the same session.
   The LLM accumulates context (sees its own schema creations from earlier chunks).
2. **Parallel chunks** — faster, each chunk processed independently in a `tokio::JoinSet`.
   Schema creation is done in a pre-processing step (first chunk or a dedicated schema
   inference pass). Subsequent chunks use the fixed schema.

**Recommended:** Sequential for correctness (option 1). The LLM can build the schema
incrementally and resolve entities across chunks because it sees the full message history.
Parallel extraction is a later optimization.

#### Stage 3 — Cross-chunk entity resolution (post-processing)

After all chunks are processed, run a deduplication pass:
1. For each node created during the session, search for similar nodes (by embedding).
2. Present candidate duplicates to the LLM in a final "resolution" pass.
3. The LLM decides: merge (update_node) or keep separate.

This can reuse the existing entity resolution logic in `create_node` but as a batch
post-processing step over all session-created nodes.

### Open Questions

- Should raw chunk text be stored as nodes in the graph? (Enables "show source" UX)
- Should the chunker be configurable per-graph or global?
- Should very small documents (< 1 chunk) skip the chunking pipeline entirely? (Yes, likely)
- Should the deduplication pass be automatic or user-triggered?

---

## Future: Graph Data Limits

**Problem:** Without limits, a single graph can grow unbounded — degrading Memgraph query
performance, consuming excessive memory, and preventing fair resource sharing between users.
When billing is introduced, users should pay more to store more data.

### Limits to enforce

| Limit | Default | Where enforced | Why |
|---|---|---|---|
| Max properties per node/edge | 50 | Knowledge service (insert/update) | Prevent bloated elements that slow queries and embeddings |
| Max nodes per graph | 10,000 | Tool executor (before insert) | Storage and billing boundary |
| Max edges per graph | 50,000 | Tool executor (before insert) | Same |

### Enforcement layers

**Layer 1 — Tool executor (AI service):**
The tool executor checks graph limits **before** calling the knowledge service. If a limit
is reached, it returns a clear message to the LLM instead of an error:

    "Node limit reached (10000/10000). The graph cannot accept more nodes."

This lets the LLM gracefully stop creating nodes and inform the user, rather than failing
mid-session.

**Layer 2 — Knowledge service (defense-in-depth):**
The knowledge service validates independently — rejects insert requests that would exceed
limits. This protects against bugs in the AI service or direct gRPC calls bypassing the
tool executor.

### Future: billing-based limits

When billing is introduced:

1. **Metadata DB** stores per-graph limits: `graphs.max_nodes`, `graphs.max_edges`,
   `graphs.max_properties` — populated from the user's plan tier.
2. **New RPC** `GetGraphLimits(graph_id)` on the metadata service — or extend `GetSchema`
   response with limit fields.
3. **AI agent** fetches limits alongside the schema at session start. The tool executor
   uses these dynamic limits instead of hardcoded constants.
4. **Plan tiers** example:
   - Free: 500 nodes, 2,500 edges
   - Pro: 50,000 nodes, 250,000 edges
   - Enterprise: unlimited

---

## Future: Direct LLM Provider Keys

**Problem:** OpenRouter is convenient for development (single API key, model switching) but
becomes a bottleneck at scale:
- **Rate limits** — with 100+ concurrent agents, 429 errors will be frequent even with
  retry logic.
- **Latency** — OpenRouter adds a proxy hop (~50-100ms) on every request.
- **Availability** — an extra point of failure between the AI service and the model provider.
- **Cost** — OpenRouter charges a small markup on top of provider pricing.

### Current mitigation

A `tokio::sync::Semaphore` caps concurrent LLM calls in the AI service. This prevents
overwhelming OpenRouter during traffic spikes but limits throughput.

### Migration plan: direct provider APIs

When scaling past ~20 concurrent agents, switch to direct provider keys:

1. **Multi-provider client** — replace `OpenRouterClient` with a `LlmClient` abstraction
   that supports multiple backends (OpenAI, Anthropic, Google) behind a common interface.
2. **Provider routing** — optionally distribute calls across multiple providers/keys
   for higher aggregate throughput (round-robin or least-loaded).
3. **Fallback chain** — if primary provider returns 429 or 5xx, fall back to a secondary
   provider (e.g., OpenAI primary → Anthropic fallback).

### Benefits of direct keys

| Aspect | OpenRouter | Direct provider |
|---|---|---|
| Rate limits | Shared across all OpenRouter users | Dedicated to your account |
| Latency | +50-100ms proxy | Direct connection |
| SLA | Best-effort | Provider SLA (99.9%+) |
| Cost | Provider price + markup | Provider price only |
| Model selection | Any model via one key | One key per provider |
