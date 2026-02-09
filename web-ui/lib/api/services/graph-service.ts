import { proxy } from "@/lib/api/proxy";
import {
  edgeData,
  edgeSchema,
  graphData,
  graphMetadata,
  graphSchema,
  nodeData,
  nodeSchema
} from "@/lib/api/schemas/response-schemas";
import {
  EdgeData,
  EdgeSchema,
  GraphData,
  GraphMetadata,
  GraphSchema,
  NodeData,
  NodeSchema,
  RequestEdgeData,
  RequestEdgeSchema,
  RequestGraph,
  RequestNodeData,
  RequestNodeSchema
} from "@/types";
import * as v from "valibot";

export interface GraphService {
  getAllMetadata(): Promise<GraphMetadata[]>;
  getOneMetadata(graph_id: string): Promise<GraphMetadata>;
  getData(graph_id: string): Promise<GraphData>;
  getSchema(graph_id: string): Promise<GraphSchema>;
  post(request: RequestGraph): Promise<GraphMetadata>;
  postNodeSchema(graph_id: string, body: RequestNodeSchema): Promise<NodeSchema>;
  postEdgeSchema(graph_id: string, body: RequestEdgeSchema): Promise<EdgeSchema>;
  postNodeData(graph_id: string, body: RequestNodeData): Promise<NodeData>;
  postEdgeData(graph_id: string, body: RequestEdgeData): Promise<EdgeData>;
}

export class ApiGraphService implements GraphService {
  private api(path: string) {
    return proxy(`/graphs${path}`);
  }

  async getAllMetadata(): Promise<GraphMetadata[]> {
    try {
      const response = await this.api("").get();
      return v.parse(v.array(graphMetadata), response);
    } catch (error) {
      console.error("Failed to get all graph metadata:", error);
      throw error;
    }
  }

  async getOneMetadata(graph_id: string): Promise<GraphMetadata> {
    try {
      const response = await this.api(`/${graph_id}/metadata`).get();
      return v.parse(graphMetadata, response);
    } catch (error) {
      console.error("Failed to get one graph metadata:", error);
      throw error;
    }
  }

  async getData(graph_id: string): Promise<GraphData> {
    try {
      const response = await this.api(`/${graph_id}/data`).get();
      return v.parse(graphData, response);
    } catch (error) {
      console.error("Failed to get graph data:", error);
      throw error;
    }
  }

  async getSchema(graph_id: string): Promise<GraphSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema`).get();
      return v.parse(graphSchema, response);
    } catch (error) {
      console.error("Failed to get graph schema:", error);
      throw error;
    }
  }

  async post(request: RequestGraph): Promise<GraphMetadata> {
    try {
      const response = await this.api("").post(request);
      return v.parse(graphMetadata, response);
    } catch (error) {
      console.error("Failed to post graph:", error);
      throw error;
    }
  }

  async postNodeSchema(graph_id: string, body: RequestNodeSchema): Promise<NodeSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema/nodes`).post(body);
      return v.parse(nodeSchema, response);
    } catch (error) {
      console.error("Failed to post node schema:", error);
      throw error;
    }
  }

  async postEdgeSchema(graph_id: string, body: RequestEdgeSchema): Promise<EdgeSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema/edges`).post(body);
      return v.parse(edgeSchema, response);
    } catch (error) {
      console.error("Failed to post edge schema:", error);
      throw error;
    }
  }

  async postNodeData(graph_id: string, body: RequestNodeData): Promise<NodeData> {
    try {
      const response = await this.api(`/${graph_id}/data/nodes`).post(body);
      return v.parse(nodeData, response);
    } catch (error) {
      console.error("Failed to post node data:", error);
      throw error;
    }
  }

  async postEdgeData(graph_id: string, body: RequestEdgeData): Promise<EdgeData> {
    try {
      const response = await this.api(`/${graph_id}/data/edges`).post(body);
      return v.parse(edgeData, response);
    } catch (error) {
      console.error("Failed to post edge data:", error);
      throw error;
    }
  }
}
