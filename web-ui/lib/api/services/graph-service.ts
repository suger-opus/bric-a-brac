import {
  CreateGraphDataDto,
  CreateGraphSchemaDto,
  GraphDataDto,
  GraphMetadataDto,
  GraphSchemaDto
} from "@/lib/api/dtos";
import { proxy } from "@/lib/api/proxy";
import {
  CreateGraph,
  CreateGraphData,
  CreateGraphSchema,
  GraphData,
  GraphMetadata,
  GraphSchema
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
  generateData(graph_id: string, file_content: File, file_type: string): Promise<CreateGraphData>;
  createSchema(graph_id: string, body: CreateGraphSchema): Promise<GraphSchema>;
  createData(graph_id: string, body: CreateGraphData): Promise<GraphData>;
}

export class ApiGraphService implements GraphService {
  private api(path: string) {
    return proxy(`/graphs${path}`);
  }

  async getAllMetadata(): Promise<GraphMetadata[]> {
    try {
      const response = await this.api("").get();
      const metadata = v.safeParse(v.array(GraphMetadataDto), response);
      if (!metadata.success) {
        console.error("Validation errors:", metadata.issues);
        throw new Error("Failed to parse graph metadata");
      }
      return metadata.output;
    } catch (error) {
      console.error("Failed to get all graph metadata:", error);
      throw error;
    }
  }

  async getOneMetadata(graph_id: string): Promise<GraphMetadata> {
    try {
      const response = await this.api(`/${graph_id}`).get();
      const metadata = v.safeParse(GraphMetadataDto, response);
      if (!metadata.success) {
        console.error("Validation errors:", metadata.issues);
        throw new Error("Failed to parse graph metadata");
      }
      return metadata.output;
    } catch (error) {
      console.error("Failed to get one graph metadata:", error);
      throw error;
    }
  }

  async getData(graph_id: string): Promise<GraphData> {
    try {
      const response = await this.api(`/${graph_id}/data`).get();
      const data = v.safeParse(GraphDataDto, response);
      if (!data.success) {
        console.error("Validation errors:", data.issues);
        throw new Error("Failed to parse graph data");
      }
      return data.output;
    } catch (error) {
      console.error("Failed to get graph data:", error);
      throw error;
    }
  }

  async getSchema(graph_id: string): Promise<GraphSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema`).get();
      const validation = v.safeParse(GraphSchemaDto, response);
      if (!validation.success) {
        console.error("Validation errors:", validation.issues);
        throw new Error("Invalid response format");
      }
      return validation.output;
    } catch (error) {
      console.error("Failed to get graph schema:", error);
      throw error;
    }
  }

  async createGraph(request: CreateGraph): Promise<GraphMetadata> {
    try {
      const response = await this.api("").post(request);
      const metadata = v.safeParse(GraphMetadataDto, response);
      if (!metadata.success) {
        console.error("Validation errors:", metadata.issues);
        throw new Error("Failed to parse graph metadata");
      }
      return metadata.output;
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
      const schema = v.safeParse(CreateGraphSchemaDto, response);
      if (!schema.success) {
        console.error("Validation errors:", schema.issues);
        throw new Error("Failed to parse create graph schema");
      }
      return schema.output;
    } catch (error) {
      console.error("Failed to generate graph schema:", error);
      throw error;
    }
  }

  async generateData(
    graph_id: string,
    file_content: File,
    file_type: string
  ): Promise<CreateGraphData> {
    try {
      const formData = new FormData();
      // TODO: shouldn't it be inside metadata ?
      formData.append("file", file_content);
      if (file_type === "text/csv") {
        formData.append("file_type", "csv");
      } else {
        formData.append("file_type", "txt");
      }
      const response = await this.api(`/${graph_id}/data/generate`).post(formData);
      const data = v.safeParse(CreateGraphDataDto, response);
      if (!data.success) {
        console.error("Validation errors:", data.issues);
        throw new Error("Failed to parse create graph data");
      }
      return data.output;
    } catch (error) {
      console.error("Failed to generate graph data:", error);
      throw error;
    }
  }

  async createSchema(graph_id: string, body: CreateGraphSchema): Promise<GraphSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema`).post(body);
      const schema = v.safeParse(GraphSchemaDto, response);
      if (!schema.success) {
        console.error("Validation errors:", schema.issues);
        throw new Error("Failed to parse graph schema");
      }
      return schema.output;
    } catch (error) {
      console.error("Failed to create graph schema:", error);
      throw error;
    }
  }

  async createData(graph_id: string, body: CreateGraphData): Promise<GraphData> {
    try {
      const response = await this.api(`/${graph_id}/data`).post(body);
      const data = v.safeParse(GraphDataDto, response);
      if (!data.success) {
        console.error("Validation errors:", data.issues);
        throw new Error("Failed to parse create graph data");
      }
      return data.output;
    } catch (error) {
      console.error("Failed to create graph data:", error);
      throw error;
    }
  }
}
