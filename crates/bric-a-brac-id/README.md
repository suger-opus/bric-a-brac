# bric-a-brac-id

A macro crate that generates **type-safe ID newtypes** wrapping UUIDv7.

---

## Why

Passing raw `Uuid` values between functions is error-prone — nothing stops you from passing
a `user_id` where a `graph_id` is expected. This crate generates distinct newtype structs
that the compiler enforces:

```rust
use bric_a_brac_id::id;

id!(UserId);
id!(GraphId);

fn get_graph(id: GraphId) -> Graph { /* ... */ }

let user_id = UserId::new();
// get_graph(user_id);  // Compile error: expected GraphId, found UserId
```

---

## The `id!()` Macro

```rust
id!(MyEntityId);
```

Generates:

```rust
pub struct MyEntityId(uuid::Uuid);
```

With the following implementations:

| Category | Traits |
|----------|--------|
| **Standard** | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord` |
| **Serialization** | `Serialize`, `Deserialize` (serde) |
| **Database** | `sqlx::Type` (transparent — maps to PostgreSQL `UUID`) |
| **Display** | `Display`, `FromStr`, `TryFrom<&str>` |
| **Access** | `AsRef<Uuid>`, `Deref` to inner UUID, `From<Uuid>` |
| **OpenAPI** | `utoipa::ToSchema` |

### Methods

- `MyEntityId::new()` — generates a new UUIDv7 (timestamp-sortable)
- `MyEntityId::nil()` — returns the nil UUID
- `MyEntityId::default()` — same as `new()`

---

## ID Types Used in the Project

```rust
id!(UserId);
id!(GraphIdDto);
id!(SessionIdDto);
id!(SessionMessageIdDto);
id!(SessionDocumentIdDto);
id!(NodeSchemaIdDto);
id!(EdgeSchemaIdDto);
```

---

## Dependencies

- `uuid` — UUID generation (v7 feature)
- `serde` — Serialization
- `sqlx` — PostgreSQL type mapping
- `utoipa` — OpenAPI schema
- `derive_more` — `From` derive
