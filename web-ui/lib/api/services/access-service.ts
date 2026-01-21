/* eslint-disable no-console */
import { graphs } from "@/lib/api/data";
import { GraphMetadata, Role } from "@/types";

export interface AccessService {
  list(): Promise<GraphMetadata[]>;
}

export class ApiAccessService implements AccessService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching accessed graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.user_role !== Role.NONE);
  }
}
