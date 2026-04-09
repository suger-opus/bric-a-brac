import * as v from "valibot";

export const NodeSchemaDto = v.object({
  node_schema_id: v.string(),
  graph_id: v.string(),
  label: v.string(),
  key: v.string(),
  color: v.string(),
  description: v.string(),
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp())
});
