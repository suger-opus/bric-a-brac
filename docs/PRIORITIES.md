# Priorities

**Goal: make bric-a-brac a portfolio-ready demo.**

A recruiter or engineer should be able to: clone the repo, read a clear README, run one
command, see a polished UI, and watch the AI build a knowledge graph live. Everything below
serves that goal.

---

## Tier 1 — Must-have (blocks "wow" demo)

### 1. E2E testing round
Feed 3-5 real PDFs and stress-test the full loop. Fix every bug found. This is the
foundation — nothing else matters if the core loop is broken.
- Extraction quality (sensible schemas and nodes?)
- Entity resolution (duplicates caught?)
- Question answering (retrieval + reasoning?)
- Chunking (cross-chunk entities merge?)

### 9. Documentation ✅
Everything a reviewer or contributor needs to understand the project.

- **Root README** ✅ — product description, architecture diagram, tech stack, full env var
  reference, getting started (Docker + local dev), "How it Works" walkthrough, security
  model, improvements section, documentation index
- **Per-service READMEs** ✅ (ai, knowledge, metadata, web-ui) — architecture, layers,
  full env var tables, how to run locally and with Docker
- **Shared crate READMEs** ✅ (bric-a-brac-protos, bric-a-brac-dtos, bric-a-brac-id)
- **Proto documentation** ✅ — inline comments on every service, RPC, message, and field
- **Design doc TL;DR** ✅ — 10-line summary added at the top of AI_AGENT_DESIGN.md
- **MIT LICENSE** ✅

### 11. Product presentation
The project needs a way to show what it does without running it.
- A polished README with embedded GIF + screenshots is the minimum — add after demo recording
- Consider a brief slide deck (PDF or Google Slides) for sharing in applications — 5-6
  slides: problem, solution, demo screenshots, architecture, tech stack

---

## Tier 2 — Should-have (makes it feel polished, not just functional)

### 14. Demo recording
Record a 2-minute screen recording: create a graph, upload a PDF, watch the AI extract
entities, ask a question. This goes in the README and can be shared standalone.

---

## Tier 3 — To consider (engineering completeness)

### 15. GitHub Actions CI
A repo with no CI looks unfinished to a technical reviewer. Minimal pipeline:
- `cargo check --workspace` + `cargo clippy --workspace -- -D warnings`
- `tsc --noEmit` in web-ui
- Trigger on push to main and on PRs

### 16. `remove_properties` tool
The only one of 16 designed tools not yet implemented. Closes the gap between the design
doc and the implementation. Small scope: `tools/remove_properties.rs`, handler in
`tool_executor.rs`, gRPC call to knowledge service (set properties to null or use
`REMOVE n.prop`).

### 17. Health check endpoint
`GET /health` on the metadata HTTP server — standard for any production service. Returns
`200 OK` with service status. Expected by reviewers and required for production
orchestration (Kubernetes readiness probes, load balancers, etc.).
