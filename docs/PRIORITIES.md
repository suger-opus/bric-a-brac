# Priorities

**Goal: make bric-a-brac a portfolio-ready demo in 12 days.**

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

### 2. Dead code removal ✅
Deleted all code for features that won't ship: Search/Bookmarks/Cheers cards,
Reddit/subreddit references (frontend + backend models, DTOs, SQL, migrations),
settings card dead buttons, empty forms folder, sample graph data, seed binary +
dataset + csv dependency.

### 3. Code cleanup ✅
Codebase cleaned for presentation:
- Removed all `TODO`, `FIXME`, `HACK` comments across all Rust crates
- Removed all commented-out code (seed.rs, http_error.rs forbidden fn, CSS vars)
- Removed dead utility functions (`pluralize`, `filterLabel`)
- `cargo clippy --fix` applied across workspace; noisy pedantic lints allowed;
  remaining ~80 warnings are intentional safety guardrails (indexing, unwrap, casts)
- `npx tsc --noEmit` — clean
- `npx eslint .` — clean (1 TanStack Table library compat warning, not our code)
- No `console.log` found; `console.error` kept intentionally

### 4. Gmail authentication
A real product needs login. Hardcoded `user_id` is the single most obvious "this is a
prototype" signal.
- OAuth2 with Google (Gmail) — standard authorization code flow
- On first login, create a `users` row in metadata Postgres
- Session token (cookie or JWT) on subsequent requests
- Wrap metadata HTTP endpoints with auth middleware that resolves `user_id` from token
- Settings card: show user email/name from Google profile, working Log Out button
- Remove the hardcoded `user_id` from `client.ts`
- Auth across micro-services

### 5. Dashboard cleanup
The dashboard is the first thing a visitor sees.
- After dead code removal: only "Your Graphs" card remains + "Create Graph" CTA
- Replace `mx-40` with responsive padding (`max-w-5xl mx-auto px-4` or similar)
- Clean layout that showcases the graph list

### 6. Small screen gate
The 3D graph doesn't work on phones/tablets. Instead of a broken experience, show a
friendly message.
- Detect viewport width (e.g. `< 1024px`)
- Show a centered message: "Bric-à-brac is designed for desktop. Please use a larger
  screen." with the logo
- Apply to all pages, not just the graph

### 7. Branding & navigation
No header, no logo, no navigation. Feels like a dev build.
- Add a minimal top bar: logo/name on the left, user avatar/email on the right (from
  Gmail auth), breadcrumbs on graph pages
- Pick a name treatment for "bric-a-brac" — simple text logo is fine
- Set a favicon + proper `<title>` per page

### 8. One-command local setup
Currently 6+ manual steps across 3 directories. Create a root `docker-compose.yaml` that
starts everything (Memgraph, Postgres, knowledge, metadata, ai). Add a
`mise.local.example.toml` (or `.env.example`) with placeholders. The README "Getting
Started" should be: clone → copy env → `docker compose up` → open browser.

### 9. Documentation
Everything a reviewer or contributor needs to understand the project.

**Root README** (already rewritten — needs screenshots/GIF after demo recording):
- Product description, architecture diagram, tech stack, getting started, design doc link

**Per-service READMEs** (ai, knowledge, metadata):
- What the service does (one paragraph)
- Environment variables reference (what goes in `mise.local.toml`)
- How to run locally (with and without Docker)
- Key entry points (main.rs, important modules)

**Shared crate READMEs** (bric-a-brac-protos, bric-a-brac-dtos, bric-a-brac-id):
- What the crate provides
- How it's used by the services
- For protos: how to regenerate (build.rs, proto files)

**Design doc** (AI_AGENT_DESIGN.md):
- Add a 10-line TL;DR at the top
- Make sure it reads as a design document, not a changelog

### 10. Rotate OpenRouter API key
The key in `ai/mise.local.toml` may be in git history. Rotate it on OpenRouter. Verify
with `git log --all -p -- '*/mise.local.toml'` that it's not committed.

### 11. Product presentation
The project needs a way to show what it does without running it.
- A short product page or landing section (can be a dedicated route in the Next.js app,
  or a standalone page) with: tagline, 3-4 feature highlights with visuals, architecture
  overview, "Get Started" CTA
- Alternatively: a polished README with embedded GIF + screenshots is the minimum
- Consider a brief slide deck (PDF or Google Slides) for sharing in applications — 5-6
  slides: problem, solution, demo screenshots, architecture, tech stack

---

## Tier 2 — Should-have (makes it feel polished, not just functional)

### 12. Loading skeletons
Replace spinners with skeleton loaders on the dashboard and graph sidebar. The difference
between "prototype" and "product."

### 13. Graph page polish
- `confirm()` for graph delete looks raw — use a styled dialog
- Hover states, transitions, micro-interactions where they matter

### 14. Demo recording
Record a 2-minute screen recording: create a graph, upload a PDF, watch the AI extract
entities, ask a question. This goes in the README and can be shared standalone.

---

## Tier 3 — Nice-to-have (if time permits)

- 404 page
- Dark mode toggle (CSS vars already exist, just not wired)
- Graph background color match theme
- `remove_properties` tool
- Orphan node detection

---

## Won't do (explicitly descoped)

- Search / Bookmarks / Cheers features — code deleted
- Reddit integration — code deleted
- Branch/diff system, MCP exposure, multi-session concurrency
- Knowledge service batch endpoints
- Mobile/tablet support (gated instead)

---

## Done (for reference)

- [x] delete_node tool (full pipeline)
- [x] delete_edge tool (full pipeline)
- [x] Edge uniqueness (MERGE upsert)
- [x] Pre-check entity resolution
- [x] Active session recovery
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
- [x] Schema proposal prompts (Phase 1 of 5-phase workflow)
- [x] Document name in messages (document_name derived via JOIN, displayed as 📎 filename)
- [x] Chunk persistence (chunk_index on session_messages, all chunks persisted to DB)
- [x] Context window management (500K char budget, chunk compression, old message trimming)
- [x] Batch tools (create_nodes, create_edges — up to 50 per call, single batch embed)
- [x] 5-phase workflow prompt (propose → create categories → store → connect → done)
- [x] Property display selection (sidebar eye toggle, graph context displayProperty)
- [x] Human-readable property names (prompt rule: "Founded Date" not "founded_date")
- [x] Force property fix (tool params filtered from stored properties)
- [x] Simplified node labels (removed zoom-based property levels)
