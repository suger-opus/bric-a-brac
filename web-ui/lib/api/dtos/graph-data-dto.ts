import * as v from "valibot";
import { CreateEdgeDataDto, EdgeDataDto } from "./edge-data-dto";
import { CreateNodeDataDto, NodeDataDto } from "./node-data-dto";

export const GraphDataDto = v.object({
  nodes: v.array(NodeDataDto),
  edges: v.array(EdgeDataDto)
});

export const CreateGraphDataDto = v.object({
  nodes: v.array(CreateNodeDataDto),
  edges: v.array(CreateEdgeDataDto)
});
