# Knowledge Service

The **data plane microservice** for graph storage and vector search. It owns all graph data
in Memgraph — nodes, edges, properties, and vector embeddings — and exposes CRUD + search
operations over gRPC.

Every node stores a 1536-dimensional embedding (from `text-embedding-3-small`) alongside
its properties. Vector indexes enable semantic similarity search, which powers entity
resolution in the AI agent.

---

## Architecture

```
src/
├── main.rs                              # Entry point: connect to Memgraph, start gRPC
├── lib.rs                               # Wire up layers, start gRPC server
├── application/
│   ├── services/
│   │   ├── query_service.rs             # Read operations: load, search, neighbors, paths
│   │   └── mutate_service.rs            # Write operations: create, update, delete, schema init
│   ├── dtos/                            # Data transfer objects
│   └── error.rs
├── domain/
│   └── models/                          # Domain models (node, edge, graph data)
├── infrastructure/
│   ├── database.rs                      # Memgraph connection (neo4rs)
│   ├── repositories/
│   │   ├── query_repository.rs          # Cypher read queries
│   │   ├── mutate_repository.rs         # Cypher write queries
│   │   └── conversions/                 # Memgraph row → domain model conversions
│   └── config/                          # Environment-based configuration
└── presentation/
    └── service.rs                       # gRPC Knowledge trait implementation
```

### Layers

| Layer | Responsibility |
|-------|---------------|
| **Presentation** | gRPC `Knowledge` trait: 13 RPC methods. Validates inputs (depth 1-10, required fields) |
| **Application** | Business logic: query orchestration, mutation transactions |
| **Domain** | Models: `NodeDataModel`, `EdgeDataModel`, `GraphDataModel`, `NodeSearchModel` |
| **Infrastructure** | `neo4rs::Graph` transactions, parameterized Cypher queries, Memgraph connection pooling |

---

## gRPC API

Defined in [`knowledge.proto`](../crates/bric-a-brac-protos/protos/knowledge.proto):

| RPC | Description |
|-----|-------------|
| `LoadGraph` | Load all nodes and edges for a graph |
| `InitializeSchema` | Create vector indexes for node schema keys |
| `CreateNode` | Insert a node with embedding and properties |
| `UpdateNode` | Merge properties + recompute embedding |
| `DeleteNode` | `DETACH DELETE` (removes node + all connected edges) |
| `CreateEdge` | Insert/upsert edge (`MERGE` semantics — one edge per key between two nodes) |
| `UpdateEdge` | Merge edge properties |
| `DeleteEdge` | Delete a single relationship |
| `SearchNodes` | Vector nearest-neighbor search (cosine similarity) |
| `GetNode` | Fetch a single node with all properties |
| `GetNeighbors` | Subgraph traversal (configurable depth, optional edge key filter) |
| `FindPaths` | Shortest paths between two nodes (configurable max depth) |
| `DeleteGraph` | Drop all nodes/edges for a graph + drop vector indexes |

### Vector Indexes

One vector index per node schema, created via `InitializeSchema`:

```cypher
CREATE VECTOR INDEX idx_{key}_embedding ON :{key}(embedding)
WITH CONFIG { "dimension": 1536, "capacity": 10000, "metric": "cos" };
```

Search uses Memgraph's `vector_search.search()`:

```cypher
CALL vector_search.search("idx_{key}_embedding", {limit}, {query_embedding})
YIELD node, distance
```

### Graph Isolation

Every Cypher query is scoped to a `graph_id` property. Nodes are labeled with their
schema key and carry a `graph_id` property. Cross-graph access is impossible by
construction.

### Security

All Cypher queries are **parameterized templates** — the AI service provides parameter
values, never raw query strings. Schema keys (used as Memgraph labels) are validated
against known schemas before interpolation.

---

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `KNOWLEDGE_SERVER_HOST` | Bind address | `0.0.0.0` |
| `KNOWLEDGE_GRPC_SERVER_PORT` | gRPC listen port | `50051` |
| `KNOWLEDGE_DB_HOST` | Memgraph hostname | `localhost` |
| `KNOWLEDGE_DB_PORT` | Memgraph Bolt port | `7687` |
| `KNOWLEDGE_DB_USER` | Memgraph username | **Yes** |
| `KNOWLEDGE_DB_PASSWORD` | Memgraph password | **Yes** |
| `KNOWLEDGE_DB_NAME` | Memgraph database name | `memgraph` |
| `KNOWLEDGE_DB_MAX_CONNECTIONS` | Connection pool size | `100` |
| `KNOWLEDGE_DB_FETCH_SIZE` | Bolt fetch size | `1000` |
| `INTERNAL_SERVICES_AUTH_TOKEN` | Shared gRPC auth token | **Yes** |
| `RUST_LOG` | Tracing filter | `info` |

For local development, configure in `mise.local.toml`:

```toml
[env]
KNOWLEDGE_DB_HOST = "localhost"
KNOWLEDGE_DB_PORT = "7687"
KNOWLEDGE_DB_USER = "admin"
KNOWLEDGE_DB_PASSWORD = "admin"
INTERNAL_SERVICES_AUTH_TOKEN = "dev-token"
```

---

## Running

### Start the database

```bash
docker compose --profile knowledge-db up -d
# Memgraph on :7687, Memgraph Lab UI on :7777
```

### Run locally

```bash
cargo run   # gRPC server on :50051
```

### With Docker

```bash
docker compose --profile knowledge up -d
```

The Dockerfile uses a multi-stage build: `rust:1.93` builder with `protobuf-compiler`,
then `debian:bookworm-slim` runtime.