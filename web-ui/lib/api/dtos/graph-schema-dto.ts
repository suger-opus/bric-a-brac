import * as v from "valibot";
import { EdgeSchemaDto } from "./edge-schema-dto";
import { NodeSchemaDto } from "./node-schema-dto";

export const GraphSchemaDto = v.object({
  nodes: v.array(NodeSchemaDto),
  edges: v.array(EdgeSchemaDto)
});
