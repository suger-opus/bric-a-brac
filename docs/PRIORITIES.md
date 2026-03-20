# Priorities

Organizing principle: **make the core loop undeniable before adding anything else.**

Core loop: talk to AI → graph grows → graph is useful → ask questions → get good answers.

---

## P0 — Core loop quality

### File upload

The product currently accepts text inline only. For real E2E testing with varied document
types (PDF, DOCX, research papers, legal contracts, meeting notes), users need to upload
files. This is the prerequisite for meaningful testing.

**Scope:**
- Frontend: file picker in ChatPanel (drag & drop or button)
- Backend: accept file in the chat endpoint, extract text server-side
- Text extraction: PDF → text, DOCX → text (lightweight, no OCR needed initially)
- Pass extracted text to the AI agent as usual (inline in SendMessage)
- No object storage needed — extract text, discard the file

**Open questions:**
- Which Rust crates for extraction? (`pdf-extract`, `docx-rs`, or call an external tool?)
- Should we store the original file for "show source" later?
- Max file size limit?

### Delete graph

`DELETE /graphs/{graph_id}` endpoint. CASCADE delete in Postgres (removes schemas, sessions,
accesses) + drop corresponding nodes/edges in Memgraph. Currently no way to delete a graph —
essential for iterating during testing.

### E2E testing round

After file upload works: feed the system diverse real documents and stress-test the full
loop. Focus areas:
- Extraction quality (does the AI create sensible schemas and nodes?)
- Entity resolution (does pre-check catch duplicates correctly?)
- Question answering (can the AI retrieve and reason over stored knowledge?)
- Edge cases (very short docs, docs with tables, docs with ambiguous entities)

---

## P1 — The "wow" moment (demo-ready)

### Incremental graph updates

Watch nodes appear on the 3D graph in real-time as the AI processes a document. Currently
the graph refreshes all at once on "done." Live building is the most compelling demo moment.

**Scope:** Parse `tool_result` SSE events in the frontend, optimistically add nodes/edges
to the 3D graph as they're created.

### Document chunking pipeline

Without this, the product only works for small documents. A user with a 20-page PDF will
hit token limits. See AI_AGENT_DESIGN.md "Future: Document Chunking Pipeline" for the
design.

**Depends on:** file upload (P0).

---

## P2 — Minimum viable product

### Authentication

OAuth2 with Google. Currently `user_id` is hardcoded. Can't put this in front of anyone
without login.

### Loading skeletons

Skeleton loaders for dashboard cards and graph page. The difference between "prototype" and
"product."

### Graph hygiene basics

Orphan node detection at minimum. Graphs will get messy during testing and users need a way
to see disconnected nodes.

---

## Can wait

- `update_edge` / `remove_properties` tools — AI can work around (delete + recreate)
- Token truncation — only matters for very long sessions
- Branch/diff system — future feature
- MCP exposure — future feature
- Multi-session concurrency — future feature
- Auto-assign schema colors — AI already returns colors when creating schemas

---

## Done (for reference)

- [x] delete_node tool (full pipeline)
- [x] delete_edge tool (full pipeline)
- [x] Edge uniqueness (MERGE upsert)
- [x] Pre-check entity resolution
- [x] Active session recovery
- [x] Session close on unmount
- [x] Streaming cancel button
- [x] Chat history recovery
- [x] Role-based tool filtering + executor guard
- [x] Schema validation
