/* eslint-disable no-console */
import { graphs } from "@/lib/api/data";
import { GraphMetadata } from "@/types";

export interface BookmarkService {
  list(): Promise<GraphMetadata[]>;
}

export class ApiBookmarkService implements BookmarkService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching bookmarked graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.is_bookmarked_by_user);
  }
}
