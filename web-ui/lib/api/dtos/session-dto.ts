import * as v from "valibot";

export const SessionDto = v.object({
  session_id: v.string(),
  graph_id: v.string(),
  status: v.string(),
  role: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
});

export const SessionMessageDto = v.object({
  message_id: v.string(),
  role: v.string(),
  content: v.string(),
  tool_calls: v.optional(v.nullable(v.any())),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
});
