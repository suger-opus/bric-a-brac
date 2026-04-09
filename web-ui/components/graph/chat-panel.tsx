"use client";

import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { useGraph } from "@/contexts/graph-context";
import { type ChatEvent, streamChat } from "@/lib/api/services/chat-service";
import { sessionService } from "@/lib/api/services/session-service";
import { SessionMessageRole, SessionStatus } from "@/types";
import {
  BotIcon,
  ChevronDownIcon,
  LoaderIcon,
  LockIcon,
  PaperclipIcon,
  SendIcon,
  WrenchIcon,
  XIcon
} from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import Markdown from "react-markdown";
import { toast } from "sonner";

type ChatItem =
  | { type: "user"; content: string; }
  | { type: "assistant"; content: string; }
  | { type: "tool"; name: string; arguments?: string; result?: string; }
  | { type: "error"; message: string; };

type ToolChatItem = Extract<ChatItem, { type: "tool"; }>;

const ToolItem = ({
  item,
  isError
}: { item: ToolChatItem; isError: boolean; }) => {
  const [expanded, setExpanded] = useState(false);

  const parsedArgs = (() => {
    if (!item.arguments) { return null; }
    try {
      return JSON.parse(item.arguments) as Record<string, unknown>;
    } catch {
      return null;
    }
  })();

  return (
    <div className="text-xs text-muted-foreground px-1">
      <Button
        variant="ghost"
        size="sm"
        className="flex items-center gap-1.5 w-full justify-start h-auto px-0 py-0 font-normal hover:bg-transparent"
        onClick={() => setExpanded((e) => !e)}
      >
        <WrenchIcon className="h-3 w-3 shrink-0" />
        <span className="font-mono truncate">{item.name}</span>
        {item.result
          && (isError
            ? <XIcon className="h-3 w-3 shrink-0 text-destructive" />
            : <span className="text-green-600 shrink-0">✓</span>)}
        <ChevronDownIcon
          className={`h-3 w-3 shrink-0 ml-auto transition-transform ${
            expanded ? "rotate-180" : ""
          }`}
        />
      </Button>
      {expanded && (
        <div className="ml-4.5 mt-1 space-y-1">
          {parsedArgs && (
            <pre className="bg-muted rounded px-2 py-1 overflow-x-auto whitespace-pre-wrap break-all text-[11px]">
              {JSON.stringify(parsedArgs, null, 2)}
            </pre>
          )}
          {item.result && (
            <pre
              className={`rounded px-2 py-1 overflow-x-auto whitespace-pre-wrap break-all text-[11px] ${
                isError ? "bg-destructive/10 text-destructive" : "bg-muted"
              }`}
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
  const {
    graphId,
    schema,
    refetch,
    addNode,
    addEdge,
    updateNode,
    updateEdge,
    removeNode,
    removeEdge
  } = useGraph();

  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isClosingSession, setIsClosingSession] = useState(false);
  const [items, setItems] = useState<ChatItem[]>([]);
  const [streamingText, setStreamingText] = useState("");
  const [progressText, setProgressText] = useState("");
  const [isStreaming, setIsStreaming] = useState(false);
  const [input, setInput] = useState("");
  const [file, setFile] = useState<File | null>(null);

  const streamingRef = useRef("");
  const scrollRef = useRef<HTMLDivElement>(null);
  const abortRef = useRef<AbortController | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const pendingToolCalls = useRef<Map<string, { name: string; arguments: string; }>>(new Map());

  // Auto-scroll to bottom
  useEffect(() => {
    const el = scrollRef.current;
    if (el) { el.scrollTop = el.scrollHeight; }
  }, [items, streamingText]);

  // Recover active session on mount
  useEffect(() => {
    if (!graphId) { return; }
    let cancelled = false;

    sessionService.list(graphId).then((sessions) => {
      if (cancelled) { return; }
      const session = sessions.find((s) => s.status === SessionStatus.ACTIVE);
      if (!session) { return; }
      setSessionId(session.session_id);
      sessionService.getMessages(session.session_id).then((messages) => {
        if (cancelled) { return; }
        const restored: ChatItem[] = [];
        let inChunkSequence = false;
        for (let i = 0; i < messages.length; i++) {
          const m = messages[i];
          if (m.role === SessionMessageRole.USER) {
            // Skip chunk messages (chunk_index >= 1 means document chunk content)
            if (m.chunk_index != null && m.chunk_index >= 1) {
              inChunkSequence = true;
              continue;
            }
            inChunkSequence = false;
            // Show document name if the message has an attached document
            let display = m.content;
            if (m.document_name) {
              const msgSep = display.indexOf("\n\n[User message]\n");
              display = msgSep >= 0
                ? `\uD83D\uDCCE ${m.document_name}\n${display.slice(msgSep + 16)}`
                : `\uD83D\uDCCE ${m.document_name}\n${display}`;
            }
            restored.push({ type: "user", content: display });
          } else if (m.role === SessionMessageRole.ASSISTANT && m.content) {
            // Skip intermediate assistant responses between chunks;
            // only show the last one (final summary after all chunks)
            if (inChunkSequence) {
              const next = messages[i + 1];
              if (
                next?.role === SessionMessageRole.USER && next.chunk_index != null
                && next.chunk_index >= 1
              ) { continue; }
              inChunkSequence = false;
            }
            restored.push({ type: "assistant", content: m.content });
          }
        }
        if (restored.length > 0) { setItems(restored); }
      }).catch(() => {});
    }).catch(() => {});

    return () => {
      cancelled = true;
    };
  }, [graphId]);

  // Cleanup on unmount: abort any ongoing stream
  useEffect(() => {
    return () => {
      abortRef.current?.abort();
    };
  }, []);

  const applyGraphDelta = useCallback(
    (toolName: string, argsJson: string, resultContent: string) => {
      try {
        switch (toolName) {
          case "create_node": {
            const result = JSON.parse(resultContent) as {
              created?: boolean;
              node_data_id?: string;
              key?: string;
              properties?: Record<string, string | number | boolean>;
            };
            if (result.created && result.node_data_id && result.key) {
              const nodeSchema = schema?.nodes.find((n) => n.key === result.key);
              addNode({
                id: result.node_data_id,
                key: result.key,
                label: nodeSchema?.label ?? result.key,
                color: nodeSchema?.color ?? "#888888",
                properties: result.properties ?? {}
              });
            }
            break;
          }
          case "create_nodes": {
            const results = JSON.parse(resultContent) as Array<{
              index?: number;
              created?: boolean;
              node_data_id?: string;
              key?: string;
              properties?: Record<string, string | number | boolean>;
            }>;
            for (const r of results) {
              if (r.created && r.node_data_id && r.key) {
                const nodeSchema = schema?.nodes.find((n) => n.key === r.key);
                addNode({
                  id: r.node_data_id,
                  key: r.key,
                  label: nodeSchema?.label ?? r.key,
                  color: nodeSchema?.color ?? "#888888",
                  properties: r.properties ?? {}
                });
              }
            }
            break;
          }
          case "create_edge": {
            const args = JSON.parse(argsJson) as {
              edge_key?: string;
              from_node_data_id?: string;
              to_node_data_id?: string;
              properties?: Record<string, string | number | boolean>;
            };
            if (args.edge_key && args.from_node_data_id && args.to_node_data_id) {
              const edgeSchema = schema?.edges.find((e) => e.key === args.edge_key);
              addEdge({
                id: `temp-${args.from_node_data_id}-${args.edge_key}-${args.to_node_data_id}`,
                source: args.from_node_data_id,
                target: args.to_node_data_id,
                key: args.edge_key,
                label: edgeSchema?.label ?? args.edge_key,
                color: edgeSchema?.color ?? "#888888",
                properties: args.properties ?? {}
              });
            }
            break;
          }
          case "create_edges": {
            const results = JSON.parse(resultContent) as Array<{
              index?: number;
              created?: boolean;
              edge_key?: string;
              from_node_data_id?: string;
              to_node_data_id?: string;
            }>;
            for (const r of results) {
              if (r.created && r.edge_key && r.from_node_data_id && r.to_node_data_id) {
                const edgeSchema = schema?.edges.find((e) => e.key === r.edge_key);
                addEdge({
                  id: `temp-${r.from_node_data_id}-${r.edge_key}-${r.to_node_data_id}`,
                  source: r.from_node_data_id,
                  target: r.to_node_data_id,
                  key: r.edge_key,
                  label: edgeSchema?.label ?? r.edge_key,
                  color: edgeSchema?.color ?? "#888888",
                  properties: {}
                });
              }
            }
            break;
          }
          case "delete_node": {
            const args = JSON.parse(argsJson) as { node_data_id?: string; };
            if (args.node_data_id && !resultContent.startsWith("Error")) {
              removeNode(args.node_data_id);
            }
            break;
          }
          case "delete_edge": {
            const args = JSON.parse(argsJson) as { edge_data_id?: string; };
            if (args.edge_data_id && !resultContent.startsWith("Error")) {
              removeEdge(args.edge_data_id);
            }
            break;
          }
          case "update_node": {
            const result = JSON.parse(resultContent) as {
              node_data_id?: string;
              properties?: Record<string, string | number | boolean>;
            };
            if (result.node_data_id && result.properties) {
              updateNode(result.node_data_id, result.properties);
            }
            break;
          }
          case "update_edge": {
            const result = JSON.parse(resultContent) as {
              edge_data_id?: string;
              properties?: Record<string, string | number | boolean>;
            };
            if (result.edge_data_id && result.properties) {
              updateEdge(result.edge_data_id, result.properties);
            }
            break;
          }
        }
      } catch {
        // Non-critical: refetch on "done" will reconcile
      }
    },
    [schema, addNode, addEdge, updateNode, updateEdge, removeNode, removeEdge]
  );

  const handleEvent = useCallback(
    (event: ChatEvent) => {
      switch (event.type) {
        case "progress":
          setProgressText(event.content);
          break;
        case "text":
          setProgressText("");
          streamingRef.current += event.content;
          setStreamingText(streamingRef.current);
          break;
        case "tool_call":
          pendingToolCalls.current.set(event.tool_call_id, {
            name: event.name,
            arguments: event.arguments
          });
          setItems((prev) => [
            ...prev,
            { type: "tool", name: event.name, arguments: event.arguments }
          ]);
          break;
        case "tool_result": {
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

          // Apply incremental graph update
          const call = pendingToolCalls.current.get(event.tool_call_id);
          if (call) {
            pendingToolCalls.current.delete(event.tool_call_id);
            applyGraphDelta(call.name, call.arguments, event.content);
          }
          break;
        }
        case "done": {
          const finalContent = streamingRef.current || event.summary;
          if (finalContent) {
            setItems((prev) => [
              ...prev,
              { type: "assistant", content: finalContent }
            ]);
          }
          streamingRef.current = "";
          setStreamingText("");
          setProgressText("");
          setIsStreaming(false);
          refetch();
          break;
        }
        case "error":
          setItems((prev) => [
            ...prev,
            { type: "error", message: event.message }
          ]);
          toast.error(event.message);
          streamingRef.current = "";
          setStreamingText("");
          setProgressText("");
          setIsStreaming(false);
          break;
      }
    },
    [refetch, applyGraphDelta]
  );

  const cancelStreaming = useCallback(() => {
    abortRef.current?.abort();
    streamingRef.current = "";
    setStreamingText("");
    setProgressText("");
    setIsStreaming(false);
  }, []);

  const closeSession = useCallback(async () => {
    if (!sessionId) { return; }
    setIsClosingSession(true);
    try {
      await sessionService.close(sessionId);
      setSessionId(null);
      setItems([]);
      toast.success("Session closed");
    } catch {
      toast.error("Could not close session");
    } finally {
      setIsClosingSession(false);
    }
  }, [sessionId]);

  const sendMessage = useCallback(async () => {
    if (!graphId || (!input.trim() && !file) || isStreaming) { return; }

    const content = input.trim();
    const currentFile = file;
    setInput("");
    setFile(null);

    const displayContent = currentFile
      ? content
        ? `📎 ${currentFile.name}\n${content}`
        : `📎 ${currentFile.name}`
      : content;
    setItems((prev) => [...prev, { type: "user", content: displayContent }]);
    setIsStreaming(true);
    streamingRef.current = "";
    setStreamingText("");

    try {
      let sid = sessionId;
      if (!sid) {
        // Check for existing active session first (recover from refresh)
        const sessions = await sessionService.list(graphId);
        const active = sessions.find((s) => s.status === SessionStatus.ACTIVE);
        if (active) {
          sid = active.session_id;
        } else {
          const session = await sessionService.create(graphId);
          sid = session.session_id;
        }
        setSessionId(sid);
      }

      abortRef.current = streamChat(graphId, sid, content, handleEvent, undefined, (error) => {
        setItems((prev) => [
          ...prev,
          { type: "error", message: error.message }
        ]);
        setIsStreaming(false);
      }, currentFile ?? undefined);
    } catch {
      setItems((prev) => [
        ...prev,
        { type: "error", message: "Failed to start chat" }
      ]);
      toast.error("Failed to start chat");
      setIsStreaming(false);
    }
  }, [graphId, input, file, isStreaming, sessionId, handleEvent]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className="flex flex-col h-full">
      {sessionId && (
        <div className="flex items-center justify-between px-3 py-1.5 border-b">
          <span className="text-xs text-muted-foreground">Active session</span>
          <Button
            variant="ghost"
            size="sm"
            className="h-6 text-xs gap-1 text-destructive hover:text-destructive hover:bg-destructive/10"
            onClick={closeSession}
            disabled={isClosingSession || isStreaming}
          >
            <LockIcon className="h-3 w-3" />
            Close
          </Button>
        </div>
      )}
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
                    <div className="bg-muted rounded-lg px-3 py-2 max-w-[85%] text-sm prose prose-sm dark:prose-invert prose-p:my-1 prose-ul:my-1 prose-ol:my-1 prose-li:my-0.5 prose-headings:my-2">
                      <Markdown>{item.content}</Markdown>
                    </div>
                  </div>
                );
              case "tool": {
                const isError = item.result?.startsWith("Error")
                  || item.result?.startsWith("error");
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
              <div className="bg-muted rounded-lg px-3 py-2 max-w-[85%] text-sm prose prose-sm dark:prose-invert prose-p:my-1 prose-ul:my-1 prose-ol:my-1 prose-li:my-0.5 prose-headings:my-2">
                <Markdown>{streamingText}</Markdown>
                <span className="animate-pulse">▍</span>
              </div>
            </div>
          )}

          {isStreaming && !streamingText && items.at(-1)?.type !== "tool" && (
            <div className="flex items-center gap-2 text-sm text-muted-foreground px-1">
              <LoaderIcon className="h-4 w-4 animate-spin" />
              {progressText || "Thinking..."}
            </div>
          )}
        </div>
      </div>

      <div className="border-t p-3">
        {file && (
          <div className="flex items-center gap-2 mb-2 text-xs text-muted-foreground bg-muted rounded px-2 py-1">
            <PaperclipIcon className="h-3 w-3 shrink-0" />
            <span className="truncate">{file.name}</span>
            <Button
              variant="ghost"
              size="icon-sm"
              className="ml-auto shrink-0 h-5 w-5"
              onClick={() => setFile(null)}
            >
              <XIcon className="h-3 w-3" />
            </Button>
          </div>
        )}
        <div className="flex gap-2">
          <input
            ref={fileInputRef}
            type="file"
            accept=".pdf,.txt,application/pdf,text/plain"
            className="hidden"
            onChange={(e) => {
              const selected = e.target.files?.[0];
              if (selected) { setFile(selected); }
              e.target.value = "";
            }}
          />
          <Button
            size="icon"
            variant="ghost"
            onClick={() => fileInputRef.current?.click()}
            disabled={isStreaming || !graphId}
            title="Attach a file"
          >
            <PaperclipIcon className="h-4 w-4" />
          </Button>
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Tell the AI what to build..."
            disabled={isStreaming || !graphId}
            rows={1}
            className="flex-1 resize-none min-h-0 py-2 text-sm shadow-none"
          />
          {isStreaming
            ? (
              <Button
                size="icon"
                variant="destructive"
                onClick={cancelStreaming}
              >
                <XIcon className="h-4 w-4" />
              </Button>
            )
            : (
              <Button
                size="icon"
                onClick={sendMessage}
                disabled={!input.trim() && !file || !graphId}
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
