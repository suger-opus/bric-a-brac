import { config } from "@/lib/config";
import { userId } from "@/lib/api/client";

export type ChatEvent =
  | { type: "text"; content: string }
  | { type: "tool_call"; tool_call_id: string; name: string; arguments: string }
  | { type: "tool_result"; tool_call_id: string; content: string }
  | { type: "done"; summary: string }
  | { type: "error"; message: string };

/**
 * Send a chat message (with optional file) and stream back AI agent events via SSE.
 * Returns an AbortController so the caller can cancel the stream.
 */
export function streamChat(
  graphId: string,
  sessionId: string,
  content: string,
  onEvent: (event: ChatEvent) => void,
  onDone?: () => void,
  onError?: (error: Error) => void,
  file?: File,
): AbortController {
  const controller = new AbortController();

  const url = `${config.env.API_URL}/graphs/${encodeURIComponent(graphId)}/chat`;

  const formData = new FormData();
  formData.append("session_id", sessionId);
  if (content) formData.append("content", content);
  if (file) formData.append("file", file);

  fetch(url, {
    method: "POST",
    headers: {
      user_id: userId,
    },
    body: formData,
    signal: controller.signal,
  })
    .then(async (response) => {
      if (!response.ok) {
        throw new Error(`Chat request failed: ${response.status}`);
      }

      const reader = response.body?.getReader();
      if (!reader) {
        throw new Error("No response body");
      }

      const decoder = new TextDecoder();
      let buffer = "";

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        buffer += decoder.decode(value, { stream: true });

        // SSE parsing: split on double newline
        const parts = buffer.split("\n\n");
        buffer = parts.pop() ?? "";

        for (const part of parts) {
          const lines = part.split("\n");
          let eventType = "";
          let data = "";

          for (const line of lines) {
            if (line.startsWith("event:")) {
              eventType = line.slice(6).trim();
            } else if (line.startsWith("data:")) {
              data = line.slice(5).trim();
            }
          }

          if (!eventType || !data) continue;

          try {
            const parsed = JSON.parse(data) as Record<string, string>;

            switch (eventType) {
              case "text":
                onEvent({ type: "text", content: parsed.content ?? "" });
                break;
              case "tool_call":
                onEvent({
                  type: "tool_call",
                  tool_call_id: parsed.tool_call_id ?? "",
                  name: parsed.name ?? "",
                  arguments: parsed.arguments ?? "",
                });
                break;
              case "tool_result":
                onEvent({
                  type: "tool_result",
                  tool_call_id: parsed.tool_call_id ?? "",
                  content: parsed.content ?? "",
                });
                break;
              case "done":
                onEvent({ type: "done", summary: parsed.summary ?? "" });
                break;
              case "error":
                onEvent({ type: "error", message: parsed.message ?? "" });
                break;
            }
          } catch {
            // Skip malformed events
          }
        }
      }

      onDone?.();
    })
    .catch((error: unknown) => {
      if (controller.signal.aborted) return;
      onError?.(error instanceof Error ? error : new Error(String(error)));
    });

  return controller;
}
