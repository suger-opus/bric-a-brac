"use client";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { useGraph } from "@/contexts/graph-context";
import { sessionService } from "@/lib/api/services/session-service";
import type { Session, SessionMessage } from "@/types";
import { SessionMessageRole, SessionStatus } from "@/types";
import { ArrowLeftIcon, BotIcon, MessageSquareIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import Markdown from "react-markdown";
import { toast } from "sonner";

type SessionsPanelProps = {
  onSwitchToChat?: () => void;
};

const SessionsPanel = ({ onSwitchToChat }: SessionsPanelProps) => {
  const { graphId } = useGraph();
  const [sessions, setSessions] = useState<Session[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [viewingSession, setViewingSession] = useState<Session | null>(null);
  const [messages, setMessages] = useState<SessionMessage[]>([]);
  const [isLoadingMessages, setIsLoadingMessages] = useState(false);

  const fetchSessions = useCallback(async () => {
    if (!graphId) { return; }
    try {
      setIsLoading(true);
      const data = await sessionService.list(graphId);
      setSessions(
        data.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      );
    } catch {
      toast.error("Could not load sessions");
    } finally {
      setIsLoading(false);
    }
  }, [graphId]);

  useEffect(() => {
    fetchSessions();
  }, [fetchSessions]);

  const viewMessages = useCallback(async (session: Session) => {
    setViewingSession(session);
    setIsLoadingMessages(true);
    try {
      const msgs = await sessionService.getMessages(session.session_id);
      setMessages(msgs);
    } catch {
      toast.error("Failed to load messages");
    } finally {
      setIsLoadingMessages(false);
    }
  }, []);

  if (viewingSession) {
    return (
      <div className="flex flex-col h-full">
        <div className="flex items-center gap-2 p-3 border-b">
          <Button
            variant="ghost"
            size="icon-sm"
            onClick={() => {
              setViewingSession(null);
              setMessages([]);
            }}
          >
            <ArrowLeftIcon className="h-4 w-4" />
          </Button>
          <div className="flex-1 min-w-0">
            <p className="text-xs font-medium truncate">
              {new Date(viewingSession.created_at).toLocaleDateString(undefined, {
                month: "short",
                day: "numeric",
                hour: "2-digit",
                minute: "2-digit"
              })}
            </p>
          </div>
          <Badge
            variant={viewingSession.status === SessionStatus.ACTIVE ? "default" : "secondary"}
            className="text-[10px] shrink-0"
          >
            {viewingSession.status}
          </Badge>
        </div>
        <div className="flex-1 overflow-y-auto p-3">
          {isLoadingMessages
            ? (
              <div className="space-y-3">
                {[1, 2, 3].map((i) => <Skeleton key={i} className="h-12" />)}
              </div>
            )
            : messages.length === 0
            ? (
              <p className="text-center text-muted-foreground text-sm py-8">
                No messages in this session.
              </p>
            )
            : (
              <div className="space-y-3">
                {messages.map((msg) => {
                  if (msg.chunk_index != null && msg.chunk_index >= 1) { return null; }
                  if (msg.role === SessionMessageRole.USER) {
                    return (
                      <div key={msg.message_id} className="flex justify-end">
                        <div className="bg-primary text-primary-foreground rounded-lg px-3 py-2 max-w-[85%] text-sm whitespace-pre-wrap">
                          {msg.document_name
                            ? (() => {
                              const sep = msg.content.indexOf("\n\n[User message]\n");
                              return sep >= 0
                                ? `\uD83D\uDCCE ${msg.document_name}\n${
                                  msg.content.slice(sep + 16)
                                }`
                                : `\uD83D\uDCCE ${msg.document_name}\n${msg.content}`;
                            })()
                            : msg.content}
                        </div>
                      </div>
                    );
                  }
                  if (msg.role === SessionMessageRole.ASSISTANT && msg.content) {
                    return (
                      <div key={msg.message_id} className="flex justify-start">
                        <div className="bg-muted rounded-lg px-3 py-2 max-w-[85%] text-sm prose prose-sm dark:prose-invert prose-p:my-1 prose-ul:my-1 prose-ol:my-1 prose-li:my-0.5 prose-headings:my-2">
                          <Markdown>{msg.content}</Markdown>
                        </div>
                      </div>
                    );
                  }
                  return null;
                })}
              </div>
            )}
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between p-3 border-b">
        <span className="text-sm font-medium">Sessions ({sessions.length})</span>
        <Button
          variant="outline"
          size="sm"
          onClick={fetchSessions}
          disabled={isLoading}
        >
          Refresh
        </Button>
      </div>
      <div className="flex-1 overflow-y-auto">
        {isLoading
          ? (
            <div className="p-3 space-y-2">
              {[1, 2, 3].map((i) => <Skeleton key={i} className="h-14" />)}
            </div>
          )
          : sessions.length === 0
          ? (
            <div className="text-center text-muted-foreground text-sm py-8 px-3">
              <MessageSquareIcon className="mx-auto mb-2 h-8 w-8 opacity-50" />
              <p>No sessions yet.</p>
              <p className="text-xs mt-1">
                Start chatting with the AI to create your first session.
              </p>
            </div>
          )
          : (
            <div className="p-2 space-y-1">
              {sessions.map((session) => (
                <Button
                  key={session.session_id}
                  variant="ghost"
                  className="w-full justify-start h-auto rounded-md border px-3 py-2.5 font-normal"
                  onClick={() => {
                    if (session.status === SessionStatus.ACTIVE) {
                      onSwitchToChat?.();
                    } else {
                      viewMessages(session);
                    }
                  }}
                >
                  <div className="flex flex-col w-full">
                    <div className="flex items-center gap-2">
                      <BotIcon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
                      <span className="text-xs font-medium">
                        {new Date(session.created_at).toLocaleDateString(undefined, {
                          month: "short",
                          day: "numeric",
                          year: "numeric",
                          hour: "2-digit",
                          minute: "2-digit"
                        })}
                      </span>
                      <Badge
                        variant={session.status === SessionStatus.ACTIVE ? "default" : "secondary"}
                        className="ml-auto text-[10px]"
                      >
                        {session.status}
                      </Badge>
                    </div>
                    <p className="text-[11px] text-muted-foreground mt-1 capitalize text-left">
                      {session.role}
                    </p>
                  </div>
                </Button>
              ))}
            </div>
          )}
      </div>
    </div>
  );
};

export default SessionsPanel;
