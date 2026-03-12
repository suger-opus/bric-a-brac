import * as v from "valibot";
import { CreateEdgeSchemaDto, EdgeSchemaDto } from "./edge-schema-dto";
import { CreateNodeSchemaDto, NodeSchemaDto } from "./node-schema-dto";

export const GraphSchemaDto = v.object({
  nodes: v.array(NodeSchemaDto),
  edges: v.array(EdgeSchemaDto)
});

export const CreateGraphSchemaDto = v.object({
  nodes: v.array(CreateNodeSchemaDto),
  edges: v.array(CreateEdgeSchemaDto)
});
