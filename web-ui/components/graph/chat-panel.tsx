"use client";

import { Button } from "@/components/ui/button";
import { useGraph } from "@/contexts/graph-context";
import { type ChatEvent, streamChat } from "@/lib/api/services/chat-service";
import { sessionService } from "@/lib/api/services/session-service";
import {
  BotIcon,
  ChevronDownIcon,
  LoaderIcon,
  SendIcon,
  WrenchIcon,
  XIcon,
} from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";

type ChatItem =
  | { type: "user"; content: string }
  | { type: "assistant"; content: string }
  | { type: "tool"; name: string; arguments?: string; result?: string }
  | { type: "error"; message: string };

type ToolChatItem = Extract<ChatItem, { type: "tool" }>;

const ToolItem = ({
  item,
  isError,
}: { item: ToolChatItem; isError: boolean }) => {
  const [expanded, setExpanded] = useState(false);

  const parsedArgs = (() => {
    if (!item.arguments) return null;
    try {
      return JSON.parse(item.arguments) as Record<string, unknown>;
    } catch {
      return null;
    }
  })();

  return (
    <div className="text-xs text-muted-foreground px-1">
      <button
        type="button"
        className="flex items-center gap-1.5 w-full text-left hover:text-foreground transition-colors"
        onClick={() => setExpanded((e) => !e)}
      >
        <WrenchIcon className="h-3 w-3 shrink-0" />
        <span className="font-mono truncate">{item.name}</span>
        {item.result &&
          (isError ? (
            <XIcon className="h-3 w-3 shrink-0 text-destructive" />
          ) : (
            <span className="text-green-600 shrink-0">✓</span>
          ))}
        <ChevronDownIcon
          className={`h-3 w-3 shrink-0 ml-auto transition-transform ${expanded ? "rotate-180" : ""}`}
        />
      </button>
      {expanded && (
        <div className="ml-4.5 mt-1 space-y-1">
          {parsedArgs && (
            <pre className="bg-muted rounded px-2 py-1 overflow-x-auto whitespace-pre-wrap break-all text-[11px]">
              {JSON.stringify(parsedArgs, null, 2)}
            </pre>
          )}
          {item.result && (
            <pre
              className={`rounded px-2 py-1 overflow-x-auto whitespace-pre-wrap break-all text-[11px] ${isError ? "bg-destructive/10 text-destructive" : "bg-muted"}`}
            >
              {item.result}
            </pre>
          )}
        </div>
      )}
    </div>
  );
};

const ChatPanel = () => {
  const { graphId, refetch } = useGraph();

  const [sessionId, setSessionId] = useState<string | null>(null);
  const [items, setItems] = useState<ChatItem[]>([]);
  const [streamingText, setStreamingText] = useState("");
  const [isStreaming, setIsStreaming] = useState(false);
  const [input, setInput] = useState("");

  const streamingRef = useRef("");
  const scrollRef = useRef<HTMLDivElement>(null);
  const abortRef = useRef<AbortController | null>(null);

  // Auto-scroll to bottom
  useEffect(() => {
    const el = scrollRef.current;
    if (el) el.scrollTop = el.scrollHeight;
  }, [items, streamingText]);

  // Recover active session on mount
  useEffect(() => {
    if (!graphId) return;
    let cancelled = false;

    sessionService.getActiveSession(graphId).then((session) => {
      if (cancelled || !session) return;
      setSessionId(session.session_id);
      sessionService.getMessages(session.session_id).then((messages) => {
        if (cancelled) return;
        const restored: ChatItem[] = messages
          .filter((m) => m.role === "user" || m.role === "assistant")
          .map((m) => ({ type: m.role as "user" | "assistant", content: m.content }));
        if (restored.length > 0) setItems(restored);
      }).catch(() => {});
    }).catch(() => {});

    return () => { cancelled = true; };
  }, [graphId]);

  // Cleanup on unmount: abort streaming & close session
  useEffect(() => {
    const sid = sessionId;
    return () => {
      abortRef.current?.abort();
      if (sid) sessionService.close(sid).catch(() => {});
    };
  }, [sessionId]);

  const handleEvent = useCallback(
    (event: ChatEvent) => {
      switch (event.type) {
        case "text":
          streamingRef.current += event.content;
          setStreamingText(streamingRef.current);
          break;
        case "tool_call":
          setItems((prev) => [
            ...prev,
            { type: "tool", name: event.name, arguments: event.arguments },
          ]);
          break;
        case "tool_result":
          setItems((prev) => {
            const updated = [...prev];
            for (let i = updated.length - 1; i >= 0; i--) {
              const item = updated[i];
              if (item.type === "tool" && !item.result) {
                updated[i] = { ...item, result: event.content };
                break;
              }
            }
            return updated;
          });
          break;
        case "done": {
          const finalContent = streamingRef.current || event.summary;
          if (finalContent) {
            setItems((prev) => [
              ...prev,
              { type: "assistant", content: finalContent },
            ]);
          }
          streamingRef.current = "";
          setStreamingText("");
          setIsStreaming(false);
          refetch();
          break;
        }
        case "error":
          setItems((prev) => [
            ...prev,
            { type: "error", message: event.message },
          ]);
          toast.error(event.message);
          streamingRef.current = "";
          setStreamingText("");
          setIsStreaming(false);
          break;
      }
    },
    [refetch],
  );

  const cancelStreaming = useCallback(() => {
    abortRef.current?.abort();
    streamingRef.current = "";
    setStreamingText("");
    setIsStreaming(false);
  }, []);

  const sendMessage = useCallback(async () => {
    if (!graphId || !input.trim() || isStreaming) return;

    const content = input.trim();
    setInput("");
    setItems((prev) => [...prev, { type: "user", content }]);
    setIsStreaming(true);
    streamingRef.current = "";
    setStreamingText("");

    try {
      let sid = sessionId;
      if (!sid) {
        const session = await sessionService.create(graphId);
        sid = session.session_id;
        setSessionId(sid);
      }

      abortRef.current = streamChat(graphId, sid, content, handleEvent, undefined, (error) => {
        setItems((prev) => [
          ...prev,
          { type: "error", message: error.message },
        ]);
        setIsStreaming(false);
      });
    } catch {
      setItems((prev) => [
        ...prev,
        { type: "error", message: "Failed to start chat" },
      ]);
      toast.error("Failed to start chat");
      setIsStreaming(false);
    }
  }, [graphId, input, isStreaming, sessionId, handleEvent]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className="flex flex-col h-full">
      <div ref={scrollRef} className="flex-1 overflow-y-auto p-3">
        <div className="space-y-3">
          {items.length === 0 && !isStreaming && (
            <div className="text-center text-muted-foreground text-sm py-8">
              <BotIcon className="mx-auto mb-2 h-8 w-8 opacity-50" />
              <p>Ask the AI agent to build your graph.</p>
              <p className="text-xs mt-1">
                It can create schemas, insert nodes & edges.
              </p>
            </div>
          )}

          {items.map((item, i) => {
            const key = `${item.type}-${i}`;
            switch (item.type) {
              case "user":
                return (
                  <div key={key} className="flex justify-end">
                    <div className="bg-primary text-primary-foreground rounded-lg px-3 py-2 max-w-[85%] text-sm">
                      {item.content}
                    </div>
                  </div>
                );
              case "assistant":
                return (
                  <div key={key} className="flex justify-start">
                    <div className="bg-muted rounded-lg px-3 py-2 max-w-[85%] text-sm whitespace-pre-wrap">
                      {item.content}
                    </div>
                  </div>
                );
              case "tool": {
                const isError =
                  item.result?.startsWith("Error") ||
                  item.result?.startsWith("error");
                return <ToolItem key={key} item={item} isError={!!isError} />;
              }
              case "error":
                return (
                  <div
                    key={key}
                    className="text-sm text-destructive bg-destructive/10 rounded-lg px-3 py-2"
                  >
                    {item.message}
                  </div>
                );
            }
          })}

          {isStreaming && streamingText && (
            <div className="flex justify-start">
              <div className="bg-muted rounded-lg px-3 py-2 max-w-[85%] text-sm whitespace-pre-wrap">
                {streamingText}
                <span className="animate-pulse">▍</span>
              </div>
            </div>
          )}

          {isStreaming && !streamingText && items.at(-1)?.type !== "tool" && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground px-1">
              <LoaderIcon className="h-4 w-4 animate-spin" />
              Thinking...
            </div>
          )}
        </div>
      </div>

      <div className="border-t p-3">
        <div className="flex gap-2">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Tell the AI what to build..."
            disabled={isStreaming || !graphId}
            rows={1}
            className="flex-1 resize-none rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
          />
          {isStreaming ? (
            <Button
              size="icon"
              variant="destructive"
              onClick={cancelStreaming}
            >
              <XIcon className="h-4 w-4" />
            </Button>
          ) : (
            <Button
              size="icon"
              onClick={sendMessage}
              disabled={!input.trim() || !graphId}
            >
              <SendIcon className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>
    </div>
  );
};

export default ChatPanel;
