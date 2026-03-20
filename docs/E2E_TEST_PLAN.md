# End-to-End Test Plan

> **Prerequisites**: All 3 services running (metadata, knowledge, ai) + databases (Postgres,
> Memgraph) + web UI (`bun dev`).

---

## Test 1 — Dashboard loads

1. Open the web UI in the browser.
2. **Report**: Does the dashboard load? Do you see the graph list (possibly empty)?

## Test 2 — Create a graph

1. Click the "New graph" button.
2. Enter any name (e.g. "Test Graph").
3. **Report**: Does it redirect to the graph page? Do you see the 3D canvas + sidebar?

## Test 3 — Chat creates schemas

1. In the Chat tab, type:
   > Create a knowledge graph about ancient Rome. Start by creating schemas for Person,
   > City, and Event, with a "LivedIn" edge schema connecting Person to City.
2. **Report**:
   - Do you see streaming text appearing?
   - Do tool_call/tool_result items appear (wrench icons)?
   - Do toast notifications pop up for each tool result?
   - Does it end with a "done" event?

## Test 4 — Graph updates after chat

1. After the done event from Test 3.
2. Switch to the "Schema" tab in the sidebar.
3. **Report**: Do you see the newly created schemas (Person, City, Event, LivedIn)?
4. **Report**: Does the 3D canvas show anything? (It shouldn't yet — no data, only schemas)

## Test 5 — Chat creates data

1. Switch back to "Chat" tab.
2. Type:
   > Add Marcus Aurelius (born 121 AD, died 180 AD, title Emperor) who lived in Rome.
   > Also add Julius Caesar (born 100 BC, died 44 BC, title Dictator) who also lived in
   > Rome. Add the event "Fall of the Republic" in 27 BC.
3. **Report**:
   - Same as Test 3 — streaming, tool calls, toasts.
   - After "done": does the 3D graph now show nodes? Are they colored? Do they have labels?
   - Can you rotate/zoom the 3D graph?

## Test 6 — Question answering

1. Type:
   > What do you know about Marcus Aurelius?
2. **Report**: Does the AI respond with text (no tool calls, or search tool calls followed
   by text)? Does it use information from the graph?

## Test 7 — Error handling

1. Open browser DevTools → Network tab.
2. Stop the AI service (kill the process).
3. Type any message in chat.
4. **Report**: Do you see an error toast? Does the chat show an error message? Does the UI
   remain functional (not stuck in "Thinking..." state)?
5. Restart the AI service.

---

For each test, report **pass/fail** and paste any error messages (browser console or toast).
