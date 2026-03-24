# Bric-à-brac

A conversational AI agent that builds and queries knowledge graphs from unstructured
documents. Upload a PDF, chat with the AI, and watch it extract entities, resolve
duplicates, and connect concepts into a navigable 3D graph — in real time.

<!-- TODO: Add a GIF/screenshot of the demo here -->

## Architecture

```
┌─────────────┐      HTTP + SSE       ┌──────────────────┐
│   Next.js   │ ◄──────────────────── │    Metadata       │
│   Web UI    │ ──────────────────► │  (Rust + Axum)    │
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

**3 Rust microservices** communicating over gRPC, a Next.js frontend, and two databases:

| Service | Role | Stack |
|---------|------|-------|
| **Metadata** | Control plane — users, graphs, schemas, sessions, chat SSE bridge | Rust, Axum, SQLx, PostgreSQL |
| **Knowledge** | Data plane — graph storage, vector search, embeddings | Rust, Tonic, neo4rs, Memgraph |
| **AI** | Agent loop — LLM tool calling, entity resolution, document chunking | Rust, Tonic, OpenRouter (GPT-4.1) |
| **Web UI** | Dashboard, 3D graph visualization, AI chat panel | Next.js 16, React 19, react-force-graph-3d |

## Key Features

- **AI-powered extraction** — upload a PDF or paste text, the AI proposes entity types,
  extracts structured data, and builds the graph autonomously
- **Entity resolution** — automatic duplicate detection via vector similarity search +
  neighbor context. The AI decides whether to merge or create
- **15 agent tools** — search, create, update, delete nodes/edges/schemas, batch operations
  (up to 50 per call), path finding, document reading
- **Real-time streaming** — SSE streams every token, tool call, and result. Nodes appear on
  the 3D graph as the AI creates them
- **5-phase workflow** — structured extraction: propose → create schemas → store entities →
  connect → done. Prevents the common failure of creating data before schemas exist
- **Document chunking** — large documents split at paragraph boundaries (~8K chars),
  processed sequentially with cross-chunk entity resolution
- **Context window management** — 500K char budget with smart trimming (chunk compression,
  old tool call collapsing)
- **Defense-in-depth security** — role-based tool filtering + executor guard + graph
  isolation. No LLM-generated queries — all Cypher/SQL is parameterized templates

## Tech Stack

**Backend:** Rust, Tokio, Tonic (gRPC), Axum (HTTP), SQLx (Postgres), neo4rs (Memgraph),
Protocol Buffers

**Frontend:** Next.js 16, React 19, TypeScript, TailwindCSS v4, shadcn/ui, Valibot,
react-force-graph-3d, three.js

**AI:** GPT-4.1 via OpenRouter, text-embedding-3-small (1536 dims), tool-calling agent loop
with streaming

**Infrastructure:** PostgreSQL, Memgraph (graph DB + vector indexes), Docker Compose, mise

## Getting Started

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) + Docker Compose
- [mise](https://mise.jdx.dev/) (or manually set env vars)
- [Node.js](https://nodejs.org/) 22+ and pnpm
- An [OpenRouter](https://openrouter.ai/) API key

### Setup

```bash
# Clone
git clone https://github.com/your-username/bric-a-brac.git
cd bric-a-brac

# Configure environment
# Copy example configs and fill in your OpenRouter API key:
cp ai/mise.local.example.toml ai/mise.local.toml
cp knowledge/mise.local.example.toml knowledge/mise.local.toml
cp metadata/mise.local.example.toml metadata/mise.local.toml

# Start all services
docker compose up -d

# Start the web UI
cd web-ui
pnpm install
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000).

## Design Documentation

See [docs/AI_AGENT_DESIGN.md](docs/AI_AGENT_DESIGN.md) for the full technical design:
agent architecture, entity resolution strategy, system prompt design, streaming protocol,
security model, and engineering decisions.

See [docs/PRIORITIES.md](docs/PRIORITIES.md) for the current roadmap.

## License

MIT
