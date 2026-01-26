import {
  requestEdgeSchema,
  requestGraph,
  requestNodeSchema,
  requestProperty,
  requestSearch
} from "@/lib/api/schemas/request-schemas";
import {
  edgeData,
  edgeSchema,
  graphData,
  graphMetadata,
  graphSchema,
  nodeData,
  nodeSchema,
  property,
  PropertyType,
  propertyValue,
  Role,
  user
} from "@/lib/api/schemas/response-schemas";
import * as v from "valibot";

export { Role };
export { PropertyType };
export type PropertyValue = v.InferOutput<typeof propertyValue>;
export type Property = v.InferOutput<typeof property>;
export type NodeSchema = v.InferOutput<typeof nodeSchema>;
export type EdgeSchema = v.InferOutput<typeof edgeSchema>;
export type NodeData = v.InferOutput<typeof nodeData>;
export type EdgeData = v.InferOutput<typeof edgeData>;
export type GraphSchema = v.InferOutput<typeof graphSchema>;
export type GraphData = v.InferOutput<typeof graphData>;
export type GraphMetadata = v.InferOutput<typeof graphMetadata>;
export type User = v.InferOutput<typeof user>;

export type RequestProperty = v.InferOutput<typeof requestProperty>;
export type RequestEdgeSchema = v.InferOutput<typeof requestEdgeSchema>;
export type RequestNodeSchema = v.InferOutput<typeof requestNodeSchema>;
export type RequestGraph = v.InferOutput<typeof requestGraph>;
export type RequestSearch = v.InferOutput<typeof requestSearch>;

// --- Processed Graph Data for Visualization ---
export type ProcessedGraphData = {
  nodes: ProcessedNodeData[];
  links: ProcessedEdgeData[];
};
export type ProcessedNodeData = {
  id: string;
  formatted_label: string;
  color: string;
  properties: NodeData["properties"];
};
export type ProcessedEdgeData = {
  id: string;
  source: string;
  target: string;
  formatted_label: string;
  color: string;
  properties: EdgeData["properties"];
};

// --- Form Inputs for Validation ---
export type FormInput<T> = {
  value: T;
  setValue: (value: T) => void;
  validate: () => boolean;
  error: string | null;
  reset: () => void;
};

export type FormInputs<T> = {
  value: { id: string; isSaved: boolean; value: T; }[];
  setValue: (value: { id: string; isSaved: boolean; value: T; }[]) => void;
  validateAll: () => boolean;
  validateOne: (id: string) => boolean;
  errors: Record<string, string | null>;
  reset: () => void;
};
