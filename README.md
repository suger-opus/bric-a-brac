# Bric-a-brac

A conversational AI that builds and queries knowledge graphs from unstructured documents.
Upload a PDF, chat with the AI, and watch it extract entities, resolve duplicates, and
connect concepts into a navigable 3D graph — in real time.

<!-- TODO: Add a GIF/screenshot of the demo here -->

---

## Why This Exists

Traditional RAG pipelines treat your data as opaque chunks in a vector store. You can't
inspect what was retained, trace wrong answers, or correct mistakes. Bric-a-brac makes
AI knowledge **tangible and editable**: a structured graph you can see, verify, and steer.

The AI builds the graph while you chat. You provide documents or topics, the AI proposes
entity types, extracts structured data, resolves duplicates via vector similarity + graph
context, and connects everything — all visible in real time on a 3D force-directed graph.

> See [docs/AI_AGENT_DESIGN.md](docs/AI_AGENT_DESIGN.md) for the full technical design
> document covering architecture decisions, entity resolution, the agent loop, streaming
> protocol, and security model.

---

## Architecture

```
┌─────────────┐      HTTP + SSE       ┌──────────────────┐
│   Next.js   │ ◄──────────────────► │    Metadata       │
│   Web UI    │                       │  (Rust · Axum)    │
│             │                       │  :8080 HTTP       │
│  • 3D graph │                       │  :50052 gRPC      │
│  • AI chat  │                       │  PostgreSQL       │
│  • Schema   │                       └────┬────────┬─────┘
│    browser  │                            │ gRPC   │ gRPC
└─────────────┘                            ▼        ▼
                                   ┌───────────┐  ┌───────────────┐
                                   │ Knowledge  │  │      AI       │
                                   │  (Rust)    │  │   (Rust)      │
                                   │  :50051    │  │   :50053      │
                                   │  Memgraph  │  │   Stateless   │
                                   └───────────┘  └───────┬───────┘
                                                          │
                                                    OpenRouter API
                                                   (GPT-4.1 + embeddings)
```

Three Rust microservices communicating over **gRPC** (with Protocol Buffers), a **Next.js**
frontend, and two databases:

| Service | Role | Stack |
|---------|------|-------|
| [**Metadata**](metadata/) | Control plane — users, graphs, schemas, sessions, HTTP API, SSE chat bridge | Rust · Axum · SQLx · PostgreSQL |
| [**Knowledge**](knowledge/) | Data plane — graph storage, vector search, CRUD | Rust · Tonic · neo4rs · Memgraph |
| [**AI**](ai/) | Agent — LLM tool-calling loop, entity resolution, document chunking | Rust · Tonic · reqwest · OpenRouter |
| [**Web UI**](web-ui/) | Frontend — dashboard, 3D graph, AI chat panel, schema browser | Next.js 16 · React 19 · shadcn/ui |

Shared Rust crates:

| Crate | Purpose |
|-------|---------|
| [**bric-a-brac-protos**](crates/bric-a-brac-protos/) | Protobuf definitions, generated gRPC code, auth interceptors, retry helpers |
| [**bric-a-brac-dtos**](crates/bric-a-brac-dtos/) | Data transfer objects, Proto↔DTO↔Model conversions |
| [**bric-a-brac-id**](crates/bric-a-brac-id/) | Type-safe UUIDv7 ID newtypes via `id!()` macro |

---

## Key Features

- **AI-powered extraction** — upload a PDF or paste text; the AI proposes entity types,
  extracts structured data, and builds the graph autonomously using a
  [5-phase workflow](docs/AI_AGENT_DESIGN.md#system-prompt-design)
- **Entity resolution** — automatic duplicate detection baked into `create_node`: vector
  similarity search + neighbor context, with `force` override.
  [Details](docs/AI_AGENT_DESIGN.md#entity-resolution)
- **15 agent tools** — search, create, update, delete nodes/edges/schemas, batch operations
  (up to 50 per call), path finding, document reading
- **Real-time streaming** — SSE streams every token, tool call, and result to the browser.
  Nodes appear on the 3D graph as the AI creates them (optimistic UI)
- **Document chunking** — large documents split at paragraph boundaries (~8K chars),
  processed sequentially with cross-chunk entity resolution and context window management
  (500K char budget with smart trimming)
- **Defense-in-depth security** — role-based tool filtering + tool executor guard + graph
  isolation. No LLM-generated queries — all Cypher and SQL use parameterized templates
- **Dark mode** — theme toggle with full 3D graph background adaptation
- **Interactive 3D graph** — force-directed visualization with schema-colored nodes,
  labeled edges, and click-to-inspect

---

## Tech Stack

| Layer | Technologies |
|-------|-------------|
| **Backend** | Rust · Tokio · Tonic (gRPC) · Axum (HTTP) · SQLx (Postgres, offline mode) · neo4rs (Memgraph) · Protocol Buffers |
| **Frontend** | Next.js 16 · React 19 (with React Compiler) · TypeScript 5 · Tailwind CSS v4 · shadcn/ui · Valibot · react-force-graph-3d · Three.js |
| **AI** | GPT-4.1 via OpenRouter · text-embedding-3-small (1536 dims) · tool-calling agent loop with SSE streaming |
| **Databases** | PostgreSQL 18 (metadata, sessions, schemas) · Memgraph (graph data, vector indexes) |
| **Infrastructure** | Docker Compose · multi-stage Dockerfiles · mise (task runner) |

---

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) 1.93+ (for local development)
- [Bun](https://bun.sh/) 1.x (for the web UI)
- [mise](https://mise.jdx.dev/) (task runner, or manually set env vars)
- An [OpenRouter](https://openrouter.ai/) API key

### Quick Start with Docker

```bash
# Clone
git clone https://github.com/your-username/bric-a-brac.git
cd bric-a-brac

# Configure environment — copy mise.toml and uncomment/fill in the values
cp mise.toml mise.local.toml
# Then edit mise.local.toml: set OPENROUTER_API_KEY and your desired passwords

# Start everything
mise run up -- all
# or without mise:
# docker compose --profile knowledge --profile metadata --profile ai --profile web-ui up -d
```

Open [http://localhost:3000](http://localhost:3000).

### Local Development

For developing individual services without Docker:

```bash
# 1. Start databases only
mise run up -- knowledge-db metadata-db

# 2. Configure: copy mise.toml to mise.local.toml and fill in your values
#    Each service also has its own mise.local.toml for service-specific overrides
#    (e.g. KNOWLEDGE_GRPC_SERVER_URL for the AI service)

# 3. Run Rust services (from each service directory)
cd knowledge && cargo run    # gRPC on :50051
cd metadata && cargo run     # HTTP on :8080, gRPC on :50052
cd ai && cargo run           # gRPC on :50053

# 4. Run the web UI
cd web-ui && bun install && bun dev   # http://localhost:3000
```

> Each service has its own README with detailed environment variables and instructions.
> See [ai/](ai/), [knowledge/](knowledge/), [metadata/](metadata/), [web-ui/](web-ui/).

### Configuration

All environment variables are documented with defaults in [`mise.toml`](mise.toml) at the
root of the repository. Copy it to `mise.local.toml` and uncomment the values you need
to override. At minimum you need to set:

- `OPENROUTER_API_KEY` — your [OpenRouter](https://openrouter.ai/) key
- Database credentials (`METADATA_DB_USER`, `METADATA_DB_PASSWORD`, etc.)
- `INTERNAL_SERVICES_AUTH_TOKEN` — any string, shared across all services

---

## Project Structure

```
bric-a-brac/
├── ai/                          # AI agent microservice
├── knowledge/                   # Knowledge graph microservice
├── metadata/                    # Metadata & API gateway microservice
├── web-ui/                      # Next.js frontend
├── crates/
│   ├── bric-a-brac-protos/      # Protobuf definitions & gRPC utilities
│   ├── bric-a-brac-dtos/        # Shared data transfer objects
│   └── bric-a-brac-id/          # Type-safe UUIDv7 ID macro
├── compose/                     # Docker Compose service files
├── docs/
│   ├── AI_AGENT_DESIGN.md       # Full technical design document
│   └── PRIORITIES.md            # Roadmap
├── compose.yaml                 # Root orchestrator (includes all services)
└── Cargo.toml                   # Rust workspace manifest
```

---

## How It Works

1. **User uploads a PDF** (or pastes text) through the chat panel
2. **Metadata service** stores the document, creates a session, and bridges the request
   via SSE to the AI service over gRPC
3. **AI service** chunks the document (~8K chars at paragraph boundaries) and enters its
   agent loop:
   - **Phase 1 — Propose:** Summarize content, propose entity types, ask to confirm
   - **Phase 2 — Create schemas:** Create node and edge schemas via metadata gRPC
   - **Phase 3 — Store entities:** Extract with batch tools (up to 50 per call), each
     going through entity resolution (vector similarity + neighbor inspection)
   - **Phase 4 — Connect:** Create edges between entities
   - **Phase 5 — Done:** Generate a summary and call `done`
4. Every tool call and result is **streamed in real time** — nodes appear on the 3D graph
   as the AI creates them
5. **Questions** trigger a different path: the AI searches the graph (vector + traversal),
   reasons over results, and responds conversationally

---

## Security Model

- **No LLM-generated queries** — all Cypher and SQL are parameterized templates. The LLM
  provides parameter values only
- **Role-based access control** — Owner, Admin, Editor, Viewer, None. Enforced at two
  independent layers: tool filtering (the LLM doesn't see write tools) and tool executor
  guard (server-side check before execution)
- **Graph isolation** — every query is scoped to a `graph_id`. No cross-graph access by
  construction
- **Service authentication** — shared bearer token on all internal gRPC calls

> See the [security section](docs/AI_AGENT_DESIGN.md#user-authentication--resource-protection)
> in the design document.

---

## Improvements & Future Work

- **Authentication** — OAuth2 social login (Google, Reddit). Currently `user_id` is passed
  as a header
- **Bulk document import** — batch upload multiple files at once
- **Graph maintenance tools** — orphan detection, schema conformance, relationship
  consistency validation
- **Knowledge service batch endpoints** — reduce gRPC call volume for batch operations
  (currently O(N) per node)
- **Direct LLM provider keys** — bypass OpenRouter for lower latency and dedicated rate
  limits at scale
- **Graph data limits** — per-plan node/edge caps for fair resource sharing and billing
- **`remove_properties` tool** — selectively remove properties without delete + recreate
- **Demo recording** — 2-minute video of the full loop

---

## Documentation

| Document | Description |
|----------|-------------|
| [AI Agent Design](docs/AI_AGENT_DESIGN.md) | Comprehensive technical design — architecture, entity resolution, agent loop, streaming, RBAC, prompt engineering |
| [Priorities](docs/PRIORITIES.md) | Roadmap and priorities |
| [AI Service](ai/README.md) | Agent microservice |
| [Knowledge Service](knowledge/README.md) | Graph storage microservice |
| [Metadata Service](metadata/README.md) | Control plane & HTTP API |
| [Web UI](web-ui/README.md) | Next.js frontend |
| [Protobuf Definitions](crates/bric-a-brac-protos/README.md) | gRPC service contracts |
| [DTOs](crates/bric-a-brac-dtos/README.md) | Shared data transfer objects |
| [ID Macro](crates/bric-a-brac-id/README.md) | Type-safe UUIDv7 newtype generator |

---

## License

MIT