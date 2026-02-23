import {
  CreateGraphSchemaDto,
  EdgeDataDto,
  EdgeSchemaDto,
  GraphDataDto,
  GraphMetadataDto,
  GraphSchemaDto,
  NodeDataDto,
  NodeSchemaDto
} from "@/lib/api/dtos";
import { proxy } from "@/lib/api/proxy";
import {
  CreateEdgeData,
  CreateEdgeSchema,
  CreateGraph,
  CreateGraphSchema,
  CreateNodeData,
  CreateNodeSchema,
  EdgeData,
  EdgeSchema,
  GraphData,
  GraphMetadata,
  GraphSchema,
  NodeData,
  NodeSchema
} from "@/types";
import * as v from "valibot";

export interface GraphService {
  getAllMetadata(): Promise<GraphMetadata[]>;
  getOneMetadata(graph_id: string): Promise<GraphMetadata>;
  getData(graph_id: string): Promise<GraphData>;
  getSchema(graph_id: string): Promise<GraphSchema>;
  createGraph(request: CreateGraph): Promise<GraphMetadata>;
  generateSchema(
    graph_id: string,
    file_content: File,
    file_type: string
  ): Promise<CreateGraphSchema>;
  createNodeSchema(graph_id: string, body: CreateNodeSchema): Promise<NodeSchema>;
  createEdgeSchema(graph_id: string, body: CreateEdgeSchema): Promise<EdgeSchema>;
  createNodeData(graph_id: string, body: CreateNodeData): Promise<NodeData>;
  createEdgeData(graph_id: string, body: CreateEdgeData): Promise<EdgeData>;
}

export class ApiGraphService implements GraphService {
  private api(path: string) {
    return proxy(`/graphs${path}`);
  }

  async getAllMetadata(): Promise<GraphMetadata[]> {
    try {
      const response = await this.api("").get();
      return v.parse(v.array(GraphMetadataDto), response);
    } catch (error) {
      console.error("Failed to get all graph metadata:", error);
      throw error;
    }
  }

  async getOneMetadata(graph_id: string): Promise<GraphMetadata> {
    try {
      const response = await this.api(`/${graph_id}`).get();
      return v.parse(GraphMetadataDto, response);
    } catch (error) {
      console.error("Failed to get one graph metadata:", error);
      throw error;
    }
  }

  async getData(graph_id: string): Promise<GraphData> {
    try {
      const response = await this.api(`/${graph_id}/data`).get();
      return v.parse(GraphDataDto, response);
    } catch (error) {
      console.error("Failed to get graph data:", error);
      throw error;
    }
  }

  async getSchema(graph_id: string): Promise<GraphSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema`).get();
      return v.parse(GraphSchemaDto, response);
    } catch (error) {
      console.error("Failed to get graph schema:", error);
      throw error;
    }
  }

  async createGraph(request: CreateGraph): Promise<GraphMetadata> {
    try {
      const response = await this.api("").post(request);
      return v.parse(GraphMetadataDto, response);
    } catch (error) {
      console.error("Failed to create graph:", error);
      throw error;
    }
  }

  async generateSchema(
    graph_id: string,
    file_content: File,
    file_type: string
  ): Promise<CreateGraphSchema> {
    try {
      const formData = new FormData();
      // TODO: shouldn't it be inside metadata ?
      formData.append("file", file_content);
      if (file_type === "text/csv") {
        formData.append("file_type", "csv");
      } else {
        formData.append("file_type", "txt");
      }
      const response = await this.api(`/${graph_id}/schema/generate`).post(formData);
      return v.parse(CreateGraphSchemaDto, response);
    } catch (error) {
      console.error("Failed to generate graph schema:", error);
      throw error;
    }
  }

  async createNodeSchema(graph_id: string, body: CreateNodeSchema): Promise<NodeSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema/nodes`).post(body);
      return v.parse(NodeSchemaDto, response);
    } catch (error) {
      console.error("Failed to create node schema:", error);
      throw error;
    }
  }

  async createEdgeSchema(graph_id: string, body: CreateEdgeSchema): Promise<EdgeSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema/edges`).post(body);
      return v.parse(EdgeSchemaDto, response);
    } catch (error) {
      console.error("Failed to create edge schema:", error);
      throw error;
    }
  }

  async createNodeData(graph_id: string, body: CreateNodeData): Promise<NodeData> {
    try {
      const response = await this.api(`/${graph_id}/data/nodes`).post(body);
      return v.parse(NodeDataDto, response);
    } catch (error) {
      console.error("Failed to create node data:", error);
      throw error;
    }
  }

  async createEdgeData(graph_id: string, body: CreateEdgeData): Promise<EdgeData> {
    try {
      const response = await this.api(`/${graph_id}/data/edges`).post(body);
      return v.parse(EdgeDataDto, response);
    } catch (error) {
      console.error("Failed to create edge data:", error);
      throw error;
    }
  }
}
