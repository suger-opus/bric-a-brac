# Web UI

The **Next.js frontend** — a single-page application with a dashboard, interactive 3D
graph visualization, real-time AI chat panel, and schema browser. Communicates with the
metadata service over HTTP and SSE.

---

## Stack

| Technology | Version | Role |
|-----------|---------|------|
| **Next.js** | 16.1 | Framework (App Router, React Compiler, standalone output) |
| **React** | 19.2 | UI library |
| **TypeScript** | 5 | Type safety |
| **Tailwind CSS** | v4 | Styling |
| **shadcn/ui** | — | Component library (Radix primitives, new-york style) |
| **react-force-graph-3d** | 1.29 | 3D force-directed graph visualization |
| **Three.js** | 0.182 | 3D rendering engine |
| **three-spritetext** | 1.10 | Text labels on 3D nodes/edges |
| **Valibot** | 1.2 | Runtime DTO validation |
| **mande** | 2.0 | HTTP client |
| **next-themes** | 0.4 | Dark/light theme switching |
| **Sonner** | 2.0 | Toast notifications |
| **Bun** | 1.x | Package manager and runtime |

---

## Project Structure

```
web-ui/
├── app/
│   ├── layout.tsx                   # Root layout: fonts, ThemeProvider, Toaster
│   ├── page.tsx                     # Dashboard: welcome, graph list, settings
│   ├── globals.css                  # Tailwind v4 imports + shadcn tokens
│   └── graph/
│       └── [graph_id]/
│           └── page.tsx             # Graph page: 3D view + sidebar
├── components/
│   ├── dashboard/
│   │   ├── cards/
│   │   │   ├── accesses-card.tsx    # Graph list with DataTable
│   │   │   └── settings-card.tsx    # User info + dark mode toggle
│   │   ├── contents/
│   │   │   └── create-dialog.tsx    # New graph creation dialog
│   │   └── tables/
│   │       ├── data-table.tsx       # Reusable DataTable (TanStack Table)
│   │       └── graph-cols.tsx       # Column definitions + delete action
│   ├── graph/
│   │   ├── graph.tsx                # 3D force graph (react-force-graph-3d)
│   │   ├── graph-sidebar.tsx        # Sidebar: header + tabs (chat/sessions/schema)
│   │   ├── chat-panel.tsx           # Chat UI: SSE streaming, file upload, tool calls
│   │   ├── sessions-panel.tsx       # Session list with switch-to-chat
│   │   ├── graph-dialog.tsx         # Node/edge detail dialog
│   │   └── items/                   # Schema items, element data display
│   ├── theme-provider.tsx           # next-themes wrapper
│   ├── small-screen-gate.tsx        # Block mobile (3D graph needs space)
│   └── ui/                          # shadcn/ui primitives
├── contexts/
│   └── graph-context.tsx            # GraphProvider: metadata, schema, data, optimistic mutations
├── hooks/
│   └── use-mobile.ts                # Screen size detection
├── lib/
│   ├── config.ts                    # Env validation with Valibot
│   ├── utils.ts                     # cn() helper
│   └── api/
│       ├── client.ts                # Typed HTTP client (mande + Valibot validation)
│       ├── dtos/                    # TypeScript DTOs with Valibot schemas
│       │   ├── user-dto.ts
│       │   ├── graph-dto.ts
│       │   ├── graph-schema-dto.ts
│       │   ├── graph-data-dto.ts
│       │   ├── node-data-dto.ts
│       │   ├── edge-data-dto.ts
│       │   ├── node-schema-dto.ts
│       │   ├── edge-schema-dto.ts
│       │   ├── session-dto.ts
│       │   ├── access-dto.ts
│       │   └── property-data-dto.ts
│       └── services/
│           ├── user-service.ts      # GET /users/me
│           ├── graph-service.ts     # Graph CRUD + schema + data
│           ├── session-service.ts   # Session lifecycle + messages
│           └── chat-service.ts      # SSE streaming via fetch()
├── types/
│   └── index.ts                     # Processed graph types for 3D rendering
└── public/                          # Static assets
```

---

## Pages

### Dashboard (`/`)

- Displays the current user's name and a welcome message
- **AccessesCard** — DataTable listing all graphs the user has access to. Columns: name,
  visibility badge (Public/Private), role badge, delete action (Owner only). Rows are
  clickable to navigate to the graph page
- **SettingsCard** — user info (member since, username, email, user ID) + dark mode toggle

### Graph Page (`/graph/[graph_id]`)

- Wrapped in `GraphProvider` (fetches metadata, schema, data in parallel)
- **Left panel** — full-screen 3D force-directed graph with schema-colored nodes,
  SpriteText labels, and themed background (dark/light)
- **Right sidebar:**
  - Header: graph name, owner, dates, visibility, node/edge counts
  - **Chat tab** — AI conversation panel with SSE streaming, file upload (PDF/TXT),
    tool call visualization, session recovery on mount
  - **Sessions tab** — list of past sessions with timestamps and status
  - **Schema tab** — collapsible node and edge schema lists with property display toggles

---

## API Client

`lib/api/client.ts` wraps [mande](https://github.com/posva/mande) with runtime validation:

```typescript
// Every response is validated with Valibot
const user = await get("/users/me", UserDtoSchema);
const graphs = await get("/graphs", v.array(GraphDtoSchema));
```

- **`get<T>(path, schema)`** — GET with Valibot parse
- **`getOptional<T>(path, schema)`** — GET, returns `null` on 404
- **`post<T>(path, body, schema)`** — POST with Valibot parse
- **`del(path)`** — DELETE (no response body)

Chat uses raw `fetch()` with manual SSE parsing (POST body required — `EventSource` only
supports GET). Returns an `AbortController` for cancel/stop.

Error handling: API client logs to `console.error`. UI components catch errors and show
user-friendly toast messages via Sonner.

---

## Real-Time Features

### SSE Chat Streaming

The chat panel opens a `POST /graphs/{graph_id}/chat` connection and parses SSE events:

| Event | UI Effect |
|-------|-----------|
| `text` | Append tokens to the assistant message (blinking cursor) |
| `tool_call` | Show tool name and arguments in a collapsible block |
| `tool_result` | Show result, trigger optimistic graph update |
| `done` | Reset streaming state, `refetch()` graph data |
| `error` | Show error toast |

### Optimistic Graph Updates

`GraphContext` exposes mutation methods (`addNode`, `addEdge`, `updateNode`, etc.) that
update the 3D graph immediately when a `tool_result` event arrives — before the full
`refetch()` on `done`. This makes nodes appear on the graph in real time as the AI
creates them.

---

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `NEXT_PUBLIC_API_URL` | Metadata HTTP URL | **Yes** (`http://localhost:8080`) |
| `NEXT_PUBLIC_USER_ID` | User ID (no auth yet) | **Yes** |

Create a `.env.local` file:

```bash
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_USER_ID=00000000-0000-0000-0000-000000000001
```

---

## Running

### Development

```bash
bun install
bun dev          # http://localhost:3000 (Turbopack)
```

### Production Build

```bash
bun run build    # Standalone output in .next/standalone
bun run start    # Production server on :3000
```

### With Docker

```bash
docker compose --profile web-ui up -d
```

The Dockerfile uses a multi-stage build: Bun builder (`oven/bun:1`) for build, then
`node:22-alpine` for the standalone server runtime.