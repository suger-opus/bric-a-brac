import { get, post } from "@/lib/api/client";
import { SessionDto, SessionMessageDto } from "@/lib/api/dtos";
import * as v from "valibot";

export const sessionService = {
  create: (graphId: string) =>
    post("/sessions", { graph_id: graphId }, SessionDto),

  get: (sessionId: string) =>
    get(`/sessions/${sessionId}`, SessionDto),

  close: (sessionId: string) =>
    post(`/sessions/${sessionId}/close`, {}, SessionDto),

  getMessages: (sessionId: string) =>
    get(`/sessions/${sessionId}/messages`, v.array(SessionMessageDto)),
};
