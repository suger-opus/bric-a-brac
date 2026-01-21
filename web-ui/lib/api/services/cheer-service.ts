/* eslint-disable no-console */
import { graphs } from "@/lib/api/data";
import { GraphMetadata } from "@/types";

export interface CheerService {
  list(): Promise<GraphMetadata[]>;
}

export class ApiCheerService implements CheerService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching cheered graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.is_cheered_by_user);
  }
}
