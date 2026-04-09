import * as v from "valibot";

export enum SessionStatus {
  ACTIVE = "Active",
  COMPLETED = "Completed",
  FAILED = "Failed"
}
export const SessionStatusDto = v.enum(SessionStatus);

export enum SessionMessageRole {
  SYSTEM = "System",
  USER = "User",
  ASSISTANT = "Assistant",
  TOOL = "Tool"
}
export const SessionMessageRoleDto = v.enum(SessionMessageRole);

export const SessionDto = v.object({
  session_id: v.string(),
  graph_id: v.string(),
  user_id: v.string(),
  status: SessionStatusDto,
  role: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp())
});

export const SessionMessageDto = v.object({
  message_id: v.string(),
  session_id: v.string(),
  position: v.number(),
  role: SessionMessageRoleDto,
  content: v.string(),
  tool_calls: v.optional(v.nullable(v.string())),
  tool_call_id: v.optional(v.nullable(v.string())),
  document_id: v.optional(v.nullable(v.string())),
  document_name: v.optional(v.nullable(v.string())),
  chunk_index: v.optional(v.nullable(v.number())),
  created_at: v.pipe(v.string(), v.isoTimestamp())
});
