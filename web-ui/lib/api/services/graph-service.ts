import { get, post } from "@/lib/api/client";
import { GraphDataDto, GraphMetadataDto, GraphSchemaDto } from "@/lib/api/dtos";
import type { CreateGraph } from "@/types";
import * as v from "valibot";

export const graphService = {
  getAllMetadata: () =>
    get("/graphs", v.array(GraphMetadataDto)),

  getOneMetadata: (graphId: string) =>
    get(`/graphs/${graphId}`, GraphMetadataDto),

  getSchema: (graphId: string) =>
    get(`/graphs/${graphId}/schema`, GraphSchemaDto),

  getData: (graphId: string) =>
    get(`/graphs/${graphId}/data`, GraphDataDto),

  createGraph: (body: CreateGraph) =>
    post("/graphs", body, GraphMetadataDto),
};
