/* eslint-disable no-console */
import { graphData, graphMetadata, graphs, graphSchema } from "@/lib/api/data";
import {
  GraphData,
  GraphMetadata,
  GraphSchema,
  NodeSchema,
  RequestGraph,
  RequestNodeSchema,
  Role
} from "@/types";

export interface GraphService {
  post(request: RequestGraph): Promise<GraphMetadata>;
  getMetadata(graph_id: string): Promise<GraphMetadata>;
  getData(graph_id: string): Promise<GraphData>;
  getSchema(graph_id: string): Promise<GraphSchema>;
  postNodeSchema(graph_id: string, nodeSchema: RequestNodeSchema): Promise<NodeSchema>;
}

export class ApiGraphService implements GraphService {
  async post(request: RequestGraph): Promise<GraphMetadata> {
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

  async postNodeSchema(graph_id: string, nodeSchema: RequestNodeSchema): Promise<NodeSchema> {
    console.log("Adding node schema to graph:", graph_id, "with label:", nodeSchema.label);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const newNodeSchema: NodeSchema = {
      node_id: `node-schema-${Math.floor(Math.random() * 1000)}`,
      label: nodeSchema.label,
      formatted_label: nodeSchema.formatted_label,
      color: nodeSchema.color,
      properties: nodeSchema.properties.map((prop, index) => ({
        property_id: `property-${index + 1}`,
        label: prop.label,
        formatted_label: prop.formatted_label,
        metadata: prop.metadata
      }))
    };
    return newNodeSchema;
  }
}
