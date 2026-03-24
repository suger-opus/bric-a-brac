import type { InferOutput } from "valibot";

import type {
  CreateGraphDto,
  EdgeDataDto,
  EdgeSchemaDto,
  GraphDataDto,
  GraphMetadataDto,
  GraphSchemaDto,
  NodeDataDto,
  NodeSchemaDto,
  PropertiesDataDto,
  SessionDto,
  SessionMessageDto,
  UserDto,
} from "@/lib/api/dtos";

export { Role } from "@/lib/api/dtos";

export type NodeSchema = InferOutput<typeof NodeSchemaDto>;
export type EdgeSchema = InferOutput<typeof EdgeSchemaDto>;
export type NodeData = InferOutput<typeof NodeDataDto>;
export type EdgeData = InferOutput<typeof EdgeDataDto>;
export type GraphSchema = InferOutput<typeof GraphSchemaDto>;
export type GraphData = InferOutput<typeof GraphDataDto>;
export type GraphMetadata = InferOutput<typeof GraphMetadataDto>;
export type PropertiesData = InferOutput<typeof PropertiesDataDto>;
export type CreateGraph = InferOutput<typeof CreateGraphDto>;
export type User = InferOutput<typeof UserDto>;
export type Session = InferOutput<typeof SessionDto>;
export type SessionMessage = InferOutput<typeof SessionMessageDto>;

// --- Processed Graph Data for Visualization ---

export type ProcessedNodeData = {
  id: string;
  key: string;
  label: string;
  color: string;
  properties: PropertiesData;
};

export type ProcessedEdgeData = {
  id: string;
  source: string;
  target: string;
  key: string;
  label: string;
  color: string;
  properties: PropertiesData;
};

export type ProcessedGraphData = {
  nodes: ProcessedNodeData[];
  links: ProcessedEdgeData[];
};
