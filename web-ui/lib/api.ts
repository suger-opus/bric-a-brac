/* eslint-disable no-console */
import { graphData, graphMetadata, graphs, graphSchema, users } from "@/lib/data";
import { GraphData, GraphMetadata, GraphSchema, Role } from "@/types/graph";
import { User } from "@/types/user";
import { z } from "zod";

export class ApiProvider {
  public static get userService(): UserService {
    return new ApiUserService();
  }

  public static get graphService(): GraphService {
    return new ApiGraphService();
  }

  public static get searchService(): SearchService {
    return new ApiSearchService();
  }

  public static get bookmarkService(): BookmarkService {
    return new ApiBookmarkService();
  }

  public static get accessService(): AccessService {
    return new ApiAccessService();
  }

  public static get cheerService(): CheerService {
    return new ApiCheerService();
  }
}

// --- User ---

interface UserService {
  getProfile(): Promise<User>;
}

class ApiUserService implements UserService {
  async getProfile(): Promise<User> {
    console.log("Fetching user profile");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return users[0];
  }
}

// --- Graph ---

export const requestCreateGraphSchema = z.object({
  name: z.string()
    .min(5, "Name must be at least 5 characters long.")
    .max(50, "Name must be at most 50 characters long."),
  description: z.string()
    .min(25, "Description must be at least 25 characters long.")
    .max(250, "Description must be at most 250 characters long.")
});
export type RequestCreateGraph = z.infer<typeof requestCreateGraphSchema>;

interface GraphService {
  create(request: RequestCreateGraph): Promise<GraphMetadata>;
  getMetadata(graph_id: string): Promise<GraphMetadata>;
  getData(graph_id: string): Promise<GraphData>;
  getSchema(graph_id: string): Promise<GraphSchema>;
}

class ApiGraphService implements GraphService {
  async create(request: RequestCreateGraph): Promise<GraphMetadata> {
    console.log("Creating graph with name:", request.name);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const newGraph: GraphMetadata = {
      graph_id: `graph-${graphs.length + 1}`,
      owner_username: "username",
      user_role: Role.OWNER,
      is_bookmarked_by_user: false,
      is_cheered_by_user: false,
      created_at: new Date(),
      updated_at: new Date(),
      name: request.name,
      description: request.description,
      is_public: false,
      nb_data_nodes: 0,
      nb_data_edges: 0,
      nb_cheers: 0,
      nb_bookmarks: 0
    };
    graphs.push(newGraph);
    return newGraph;
  }

  async getMetadata(graph_id: string): Promise<GraphMetadata> {
    console.log("Fetching metadata for graph:", graph_id);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphMetadata;
  }

  async getData(graph_id: string): Promise<GraphData> {
    console.log("Fetching data for graph:", graph_id);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphData;
  }

  async getSchema(graph_id: string): Promise<GraphSchema> {
    console.log("Fetching schema for graph:", graph_id);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphSchema;
  }
}

// --- Accesses ---

interface AccessService {
  list(): Promise<GraphMetadata[]>;
}

class ApiAccessService implements AccessService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching accessed graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.user_role !== Role.NONE);
  }
}

// --- Search ---

export const requestSearchSchema = z.object({
  keyword: z.string()
    .min(5, "Keyword must be at least 5 characters long.")
    .max(50, "Keyword must be at most 50 characters long.")
});
export type RequestSearch = z.infer<typeof requestSearchSchema>;

interface SearchService {
  search(request: RequestSearch): Promise<GraphMetadata[]>;
}

class ApiSearchService implements SearchService {
  async search(request: RequestSearch): Promise<GraphMetadata[]> {
    console.log("Searching graphs with keyword:", request.keyword);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs;
  }
}

// --- Bookmarks ---

interface BookmarkService {
  list(): Promise<GraphMetadata[]>;
}

class ApiBookmarkService implements BookmarkService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching bookmarked graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.is_bookmarked_by_user);
  }
}

// --- Cheers ---

interface CheerService {
  list(): Promise<GraphMetadata[]>;
}

class ApiCheerService implements CheerService {
  async list(): Promise<GraphMetadata[]> {
    console.log("Fetching cheered graphs");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    return graphs.filter((graph) => graph.is_cheered_by_user);
  }
}
