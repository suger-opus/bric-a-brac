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
  createSchema(graph_id: string, body: CreateGraphSchema): Promise<GraphSchema>;
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
      // const response = await this.api(`/${graph_id}/schema/generate`).post(formData);
      const response = {
        "nodes": [
          {
            "label": "Person",
            "color": "#28B463",
            "properties": [
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Name",
                "property_type": "String",
                "metadata": {
                  "options": null
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Location",
                "property_type": "String",
                "metadata": {
                  "options": null
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Years of Experience",
                "property_type": "Number",
                "metadata": {
                  "options": null
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Role",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Keeper",
                    "Witness"
                  ]
                }
              }
            ]
          },
          {
            "label": "Element",
            "color": "#FFC300",
            "properties": [
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Type",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Fog",
                    "Sea",
                    "Light",
                    "Gannet"
                  ]
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Color",
                "property_type": "String",
                "metadata": {
                  "options": null
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Behavior",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Indifferent",
                    "Responsive"
                  ]
                }
              }
            ]
          },
          {
            "label": "Object",
            "color": "#C70039",
            "properties": [
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Name",
                "property_type": "String",
                "metadata": {
                  "options": null
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Type",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Lighthouse",
                    "Logbook"
                  ]
                }
              }
            ]
          }
        ],
        "edges": [
          {
            "label": "Records",
            "color": "#FF5733",
            "properties": [
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Type",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Routine",
                    "Observation"
                  ]
                }
              },
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Frequency",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Daily",
                    "Weekly",
                    "Monthly"
                  ]
                }
              }
            ]
          },
          {
            "label": "Communicates With",
            "color": "#3355FF",
            "properties": [
              {
                "node_schema_id": null,
                "edge_schema_id": null,
                "label": "Connection Type",
                "property_type": "Select",
                "metadata": {
                  "options": [
                    "Parent-Child",
                    "Profession"
                  ]
                }
              }
            ]
          }
        ]
      };
      const schema = v.safeParse(CreateGraphSchemaDto, response);
      if (!schema.success) {
        console.error("Validation errors:", schema.issues);
        throw new Error("Invalid response format");
      }
      return schema.output;
    } catch (error) {
      console.error("Failed to generate graph schema:", error);
      throw error;
    }
  }

  async createSchema(graph_id: string, body: CreateGraphSchema): Promise<GraphSchema> {
    try {
      const response = await this.api(`/${graph_id}/schema`).post(body);
      return v.parse(GraphSchemaDto, response);
    } catch (error) {
      console.error("Failed to create graph schema:", error);
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
