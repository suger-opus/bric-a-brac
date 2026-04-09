import { get, post } from "@/lib/api/client";
import { SessionDto, SessionMessageDto } from "@/lib/api/dtos";
import * as v from "valibot";

export const sessionService = {
  create: (graphId: string) => post(`/graphs/${graphId}/sessions`, {}, SessionDto),

  list: (graphId: string) => get(`/graphs/${graphId}/sessions`, v.array(SessionDto)),

  close: (sessionId: string) => post(`/sessions/${sessionId}/close`, {}, SessionDto),

  getMessages: (sessionId: string) =>
    get(`/sessions/${sessionId}/messages`, v.array(SessionMessageDto))
};
