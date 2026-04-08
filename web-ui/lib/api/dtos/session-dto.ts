import * as v from "valibot";

export const SessionDto = v.object({
  session_id: v.string(),
  graph_id: v.string(),
  status: v.string(),
  role: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp())
});

export const SessionMessageDto = v.object({
  message_id: v.string(),
  role: v.string(),
  content: v.string(),
  tool_calls: v.optional(v.nullable(v.any())),
  document_id: v.optional(v.nullable(v.string())),
  document_name: v.optional(v.nullable(v.string())),
  chunk_index: v.optional(v.nullable(v.number())),
  created_at: v.pipe(v.string(), v.isoTimestamp())
});
