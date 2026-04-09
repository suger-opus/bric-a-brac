# bric-a-brac-dtos

Shared crate providing **data transfer objects** (DTOs) used across all Rust services.
DTOs are the currency between layers — services accept and return DTOs, converting to/from
domain models internally.

---

## What's Inside

### DTO Modules

| File | Contents |
|------|----------|
| `user_dto.rs` | `UserDto`, `CreateUserDto` |
| `graph_dto.rs` | `GraphMetadataDto`, `CreateGraphDto` |
| `graph_schema_dto.rs` | `GraphSchemaDto`, `NodeSchemaDto`, `EdgeSchemaDto`, `CreateNodeSchemaDto`, `CreateEdgeSchemaDto` |
| `graph_data_dto.rs` | `GraphDataDto` (processed graph for API responses) |
| `node_data_dto.rs` | `NodeDataDto`, `CreateNodeDataDto`, `UpdateNodeDataDto`, `NodeSearchDto` |
| `edge_data_dto.rs` | `EdgeDataDto`, `CreateEdgeDataDto`, `UpdateEdgeDataDto` |
| `property_data_dto.rs` | `PropertyValueDto` (string, number, or bool) |
| `session_dto.rs` | `SessionDto`, `CreateSessionDto`, `SessionMessageDto`, `CreateSessionMessageDto`, `SessionDocumentDto`, `CreateSessionDocumentDto` |
| `access_dto.rs` | `AccessDto`, `CreateAccessDto`, `RoleDto` |
| `agent_event_dto.rs` | `AgentEventDto` (text, tool call, tool result, done, error) |
| `primitives.rs` | Shared primitives and type aliases |

### Conversions

Each DTO file implements `From` trait conversions:

- **Proto → DTO** — for gRPC responses received from other services
- **DTO → Proto** — for gRPC requests sent to other services
- **Model → DTO** — for returning data from service methods
- **DTO → Model** — for passing data to repositories

This keeps conversion logic co-located with the types and ensures a clean boundary between
the protobuf, application, and domain layers.

### ID Types

DTOs use type-safe ID newtypes from `bric-a-brac-id` (e.g. `GraphIdDto`, `SessionIdDto`)
to prevent accidentally mixing IDs of different entities.

### Validation

DTOs with creation semantics (e.g. `CreateGraphDto`, `CreateUserDto`) use the `validator`
crate for field validation (min length, format, etc.).

### OpenAPI

DTOs derive `utoipa::ToSchema` for automatic OpenAPI documentation generation in the
metadata service.

---

## Usage

```rust
use bric_a_brac_dtos::{GraphMetadataDto, CreateGraphDto, NodeDataDto, SessionDto};
```

All DTOs are re-exported from the crate root via `pub use dtos::*`.

---

## Dependencies

- `bric-a-brac-protos` — Proto types for conversions
- `bric-a-brac-id` — Type-safe ID newtypes
- `serde` — Serialization/deserialization
- `sqlx` — Database type mappings
- `utoipa` — OpenAPI schema derivation
- `validator` — Field validation
- `chrono` — Timestamps
- `prost-types` — Protobuf timestamp conversions
