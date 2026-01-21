import * as v from "valibot";

export enum Role {
  OWNER = "owner",
  ADMIN = "admin",
  EDITOR = "editor",
  VIEWER = "viewer",
  NONE = "none"
}
export const role = v.enum(Role);

export enum PropertyType {
  NUMBER = "number",
  // INTEGER = "integer",
  // FLOAT = "float",
  STRING = "string",
  BOOLEAN = "boolean",
  // DATE = "date",
  // TIME = "time",
  // RANGE = "range",
  SELECT = "select"
  // MULTISELECT = "multiselect"
}
export const propertyType = v.enum(PropertyType);

export const propertyValue = v.union([
  v.string(),
  v.number(),
  v.boolean(),
  v.array(v.string())
]);

export const property = v.object({
  property_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  metadata: v.object({
    property_type: propertyType,
    details: v.object({
      // min: v.nullable(v.number()),
      // max: v.nullable(v.number()),
      options: v.nullable(v.array(v.string())),
      required: v.boolean()
    })
  })
});

export const nodeSchema = v.object({
  node_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  color: v.string(),
  properties: v.array(property)
});

export const edgeSchema = v.object({
  edge_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  color: v.string(),
  properties: v.array(property)
});

export const nodeData = v.object({
  graph_id: v.string(),
  node_id: v.string(),
  label: v.string(),
  properties: v.record(v.string(), propertyValue)
});

export const edgeData = v.object({
  graph_id: v.string(),
  edge_id: v.string(),
  from_id: v.string(),
  to_id: v.string(),
  label: v.string(),
  properties: v.record(v.string(), propertyValue)
});

export const graphSchema = v.object({
  nodes: v.array(nodeSchema),
  edges: v.array(edgeSchema)
});

export const graphData = v.object({
  nodes: v.array(nodeData),
  edges: v.array(edgeData)
});

export const graphMetadata = v.object({
  graph_id: v.string(),
  owner_username: v.string(),
  created_at: v.date(),
  updated_at: v.date(),
  name: v.string(),
  description: v.string(),
  user_role: role,
  is_public: v.boolean(),
  is_bookmarked_by_user: v.boolean(),
  is_cheered_by_user: v.boolean(),
  nb_data_nodes: v.number(),
  nb_data_edges: v.number(),
  nb_cheers: v.number(),
  nb_bookmarks: v.number()
});

export const user = v.object({
  user_id: v.string(),
  username: v.string(),
  email: v.string(),
  created_at: v.date()
});
