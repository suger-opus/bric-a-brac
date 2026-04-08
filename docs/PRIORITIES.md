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
