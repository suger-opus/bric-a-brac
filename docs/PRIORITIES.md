# Priorities

Organizing principle: **make the core loop undeniable before adding anything else.**

Core loop: talk to AI → graph grows → graph is useful → ask questions → get good answers.

---

## P0 — Phase 1: Make the chat loop excellent

### 1. E2E testing round

Feed the system 5+ real PDFs and stress-test the full loop. Focus areas:
- Extraction quality (does the AI create sensible schemas and nodes?)
- Entity resolution (does pre-check catch duplicates correctly?)
- Question answering (can the AI retrieve and reason over stored knowledge?)
- Edge cases (very short docs, docs with tables, docs with ambiguous entities)
- Chunking quality (do cross-chunk entities merge correctly?)

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

- `remove_properties` tool — AI can work around (delete + recreate)
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
- [x] File upload (PDF + TXT, multipart endpoint, 50MB limit)
- [x] Delete graph (full stack, CASCADE + Memgraph cleanup)
- [x] Document chunking (paragraph-boundary splitting, ~8K char chunks, multi-chunk summary)
- [x] Incremental graph updates (optimistic UI via tool_result SSE events)
- [x] update_edge tool (full pipeline)
- [x] Session documents (session_documents table, read_document tool, document_id on messages)
- [x] Schema proposal prompts (Rule #1 in system prompt, proposal before extraction)
- [x] Document name in messages (document_name derived via JOIN, displayed as 📎 filename)
- [x] Chunk persistence (chunk_index on session_messages, all chunks persisted to DB)
- [x] Context window management (500K char budget, chunk compression, old message trimming)
