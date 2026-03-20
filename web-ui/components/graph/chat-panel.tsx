"use client";

import { Button } from "@/components/ui/button";
import { useGraph } from "@/contexts/graph-context";
import { type ChatEvent, streamChat } from "@/lib/api/services/chat-service";
import { sessionService } from "@/lib/api/services/session-service";
import { BotIcon, LoaderIcon, SendIcon, WrenchIcon } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";

type ChatItem =
  | { type: "user"; content: string }
  | { type: "assistant"; content: string }
  | { type: "tool"; name: string; result?: string }
  | { type: "error"; message: string };

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

  // Cleanup on unmount
  useEffect(() => {
    return () => abortRef.current?.abort();
  }, []);

  const handleEvent = useCallback(
    (event: ChatEvent) => {
      switch (event.type) {
        case "text":
          streamingRef.current += event.content;
          setStreamingText(streamingRef.current);
          break;
        case "tool_call":
          setItems((prev) => [...prev, { type: "tool", name: event.name }]);
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
        case "done":
          if (streamingRef.current) {
            setItems((prev) => [
              ...prev,
              { type: "assistant", content: streamingRef.current },
            ]);
          }
          streamingRef.current = "";
          setStreamingText("");
          setIsStreaming(false);
          refetch();
          break;
        case "error":
          setItems((prev) => [
            ...prev,
            { type: "error", message: event.message },
          ]);
          streamingRef.current = "";
          setStreamingText("");
          setIsStreaming(false);
          break;
      }
    },
    [refetch],
  );

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
              case "tool":
                return (
                  <div
                    key={key}
                    className="flex items-center gap-1.5 text-xs text-muted-foreground px-1"
                  >
                    <WrenchIcon className="h-3 w-3 shrink-0" />
                    <span className="font-mono truncate">{item.name}</span>
                    {item.result && (
                      <span className="text-green-600 shrink-0">✓</span>
                    )}
                  </div>
                );
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
          <Button
            size="icon"
            onClick={sendMessage}
            disabled={isStreaming || !input.trim() || !graphId}
          >
            <SendIcon className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  );
};

export default ChatPanel;
