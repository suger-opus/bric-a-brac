/* eslint-disable no-console */
import { graphs } from "@/lib/api/data";
import { GraphMetadata, RequestSearch } from "@/types";

export interface SearchService {
  search(request: RequestSearch): Promise<GraphMetadata[]>;
}

export class ApiSearchService implements SearchService {
  async search(request: RequestSearch): Promise<GraphMetadata[]> {
    console.log("Searching graphs with keyword:", request.keyword);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs;
  }
}
