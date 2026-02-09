import * as v from "valibot";

export enum Role {
  OWNER = "Owner",
  ADMIN = "Admin",
  EDITOR = "Editor",
  VIEWER = "Viewer",
  NONE = "None"
}
export const role = v.enum(Role);

export enum PropertyType {
  NUMBER = "Number",
  // INTEGER = "integer",
  // FLOAT = "float",
  STRING = "String",
  BOOLEAN = "Boolean",
  // DATE = "date",
  // TIME = "time",
  // RANGE = "range",
  SELECT = "Select"
  // MULTISELECT = "multiselect"
}
export const propertyType = v.enum(PropertyType);

export const propertyValue = v.union([
  v.pipe(v.string(), v.maxLength(50, "String value must be at most 250 characters long.")),
  v.number(),
  v.boolean()
]);

export const property = v.object({
  property_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  property_type: propertyType,
  metadata: v.object({
    // min: v.nullable(v.number()),
    // max: v.nullable(v.number()),
    options: v.nullable(v.array(v.string()))
    // required: v.boolean()
  })
});

export const nodeSchema = v.object({
  node_schema_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  color: v.string(),
  properties: v.array(property)
});

export const edgeSchema = v.object({
  edge_schema_id: v.string(),
  label: v.string(),
  formatted_label: v.string(),
  color: v.string(),
  properties: v.array(property)
});

export const nodeData = v.object({
  node_data_id: v.string(),
  formatted_label: v.string(),
  properties: v.record(v.string(), propertyValue)
});

export const edgeData = v.object({
  edge_data_id: v.string(),
  from_node_data_id: v.string(),
  to_node_data_id: v.string(),
  formatted_label: v.string(),
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
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp()),
  name: v.string(),
  description: v.string(),
  user_role: role,
  is_public: v.boolean(),
  reddit: v.object({}),
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
  created_at: v.pipe(v.string(), v.isoTimestamp()),
  updated_at: v.pipe(v.string(), v.isoTimestamp())
});
