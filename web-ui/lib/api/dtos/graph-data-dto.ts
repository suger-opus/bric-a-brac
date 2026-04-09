import * as v from "valibot";
import { EdgeDataDto } from "./edge-data-dto";
import { NodeDataDto } from "./node-data-dto";

export const GraphDataDto = v.object({
  nodes: v.array(NodeDataDto),
  edges: v.array(EdgeDataDto)
});
