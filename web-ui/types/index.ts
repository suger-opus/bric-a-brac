import * as v from "valibot";

import {
  CreateEdgeDataDto,
  CreateEdgeSchemaDto,
  CreateGraphDto,
  CreateGraphSchemaDto,
  CreateNodeDataDto,
  CreateNodeSchemaDto,
  CreatePropertySchemaDto,
  EdgeDataDto,
  EdgeSchemaDto,
  GraphDataDto,
  GraphMetadataDto,
  GraphSchemaDto,
  NodeDataDto,
  NodeSchemaDto,
  PropertiesDataDto,
  PropertySchemaDto,
  PropertyType,
  PropertyValueDto,
  Role,
  SearchGraphDto,
  UserDto
} from "@/lib/api/dtos";

// --- API Requests & Response Types ---
export { Role };
export { PropertyType };
export type PropertyValue = v.InferOutput<typeof PropertyValueDto>;
export type PropertySchema = v.InferOutput<typeof PropertySchemaDto>;
export type NodeSchema = v.InferOutput<typeof NodeSchemaDto>;
export type EdgeSchema = v.InferOutput<typeof EdgeSchemaDto>;
export type NodeData = v.InferOutput<typeof NodeDataDto>;
export type EdgeData = v.InferOutput<typeof EdgeDataDto>;
export type GraphSchema = v.InferOutput<typeof GraphSchemaDto>;
export type GraphData = v.InferOutput<typeof GraphDataDto>;
export type GraphMetadata = v.InferOutput<typeof GraphMetadataDto>;
export type User = v.InferOutput<typeof UserDto>;

export type CreatePropertySchema = v.InferOutput<typeof CreatePropertySchemaDto>;
export type PropertiesData = v.InferOutput<typeof PropertiesDataDto>;
export type CreateNodeSchema = v.InferOutput<typeof CreateNodeSchemaDto>;
export type CreateEdgeSchema = v.InferOutput<typeof CreateEdgeSchemaDto>;
export type CreateNodeData = v.InferOutput<typeof CreateNodeDataDto>;
export type CreateEdgeData = v.InferOutput<typeof CreateEdgeDataDto>;
export type CreateGraph = v.InferOutput<typeof CreateGraphDto>;
export type CreateSearch = v.InferOutput<typeof SearchGraphDto>;
export type CreateGraphSchema = v.InferOutput<typeof CreateGraphSchemaDto>;

// --- Processed Graph Data for Visualization ---
export type ProcessedGraphData = {
  nodes: ProcessedNodeData[];
  links: ProcessedEdgeData[];
};
export type ProcessedNodeData = {
  id: string;
  key: string;
  color: string;
  properties: NodeData["properties"];
};
export type ProcessedEdgeData = {
  id: string;
  source: string;
  target: string;
  key: string;
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

// --- Available Actions ---
export enum Action {
  FIND_NODE = "FIND_NODES",
  FIND_PATH = "FIND_PATH",
  ASK_AI = "ASK_AI",
  BUILD_WITH_AI = "BUILD_WITH_AI",
  NEW_NODE_TYPE = "NEW_NODE_TYPE",
  NEW_EDGE_TYPE = "NEW_EDGE_TYPE",
  MANAGE_NODE_TYPES = "MANAGE_NODE_TYPES",
  MANAGE_EDGE_TYPES = "MANAGE_EDGE_TYPES",
  INSERT_NODE = "INSERT_NODE",
  INSERT_EDGE = "INSERT_EDGE",
  MANAGE_NODES = "MANAGE_NODES",
  MANAGE_EDGES = "MANAGE_EDGES",
  METADATA = "METADATA",
  ACCESSES = "ACCESSES",
  VISIBILITY = "VISIBILITY",
  ANALYTICS = "ANALYTICS",
  DELETE_GRAPH = "DELETE_GRAPH"
}
