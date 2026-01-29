/* eslint-disable no-console */
import { graphData, graphMetadata, graphs, graphSchema } from "@/lib/api/data";
import {
  EdgeData,
  EdgeSchema,
  GraphData,
  GraphMetadata,
  GraphSchema,
  NodeData,
  NodeSchema,
  PropertyValue,
  RequestEdgeData,
  RequestEdgeSchema,
  RequestGraph,
  RequestNodeData,
  RequestNodeSchema,
  Role
} from "@/types";

export interface GraphService {
  post(request: RequestGraph): Promise<GraphMetadata>;
  getMetadata(graph_id: string): Promise<GraphMetadata>;
  getData(graph_id: string): Promise<GraphData>;
  getSchema(graph_id: string): Promise<GraphSchema>;
  postNodeSchema(graph_id: string, nodeSchema: RequestNodeSchema): Promise<NodeSchema>;
  postEdgeSchema(graph_id: string, edgeSchema: RequestEdgeSchema): Promise<EdgeSchema>;
  postNodeData(graph_id: string, nodeData: RequestNodeData): Promise<NodeData>;
  postEdgeData(graph_id: string, edgeData: RequestEdgeData): Promise<EdgeData>;
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

  async postEdgeSchema(graph_id: string, edgeSchema: RequestEdgeSchema): Promise<EdgeSchema> {
    console.log("Adding edge schema to graph:", graph_id, "with label:", edgeSchema.label);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const newEdgeSchema: EdgeSchema = {
      edge_id: `edge-schema-${Math.floor(Math.random() * 1000)}`,
      label: edgeSchema.label,
      formatted_label: edgeSchema.formatted_label,
      color: edgeSchema.color,
      properties: edgeSchema.properties.map((prop, index) => ({
        property_id: `property-${index + 1}`,
        label: prop.label,
        formatted_label: prop.formatted_label,
        metadata: prop.metadata
      }))
    };
    return newEdgeSchema;
  }

  async postNodeData(graph_id: string, nodeData: RequestNodeData): Promise<NodeData> {
    console.log("Adding node data to graph:", graph_id);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const newNodeData: NodeData = {
      graph_id,
      node_id: `node-${Math.floor(Math.random() * 1000)}`,
      formatted_label: "New Node",
      properties: nodeData.properties.reduce((acc, prop) => {
        acc[prop.property_id] = prop.value;
        return acc;
      }, {} as Record<string, PropertyValue>)
    };
    return newNodeData;
  }

  async postEdgeData(graph_id: string, edgeData: RequestEdgeData): Promise<EdgeData> {
    console.log("Adding edge data to graph:", graph_id);
    await new Promise((resolve) => setTimeout(resolve, 1000));
    const newEdgeData: EdgeData = {
      graph_id,
      edge_id: `edge-${Math.floor(Math.random() * 1000)}`,
      from_id: edgeData.from_id,
      to_id: edgeData.to_id,
      formatted_label: "New Edge",
      properties: edgeData.properties.reduce((acc, prop) => {
        acc[prop.property_id] = prop.value;
        return acc;
      }, {} as Record<string, PropertyValue>)
    };
    return newEdgeData;
  }
}
