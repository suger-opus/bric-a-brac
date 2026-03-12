import { useGraph } from "@/contexts/graph-context";
import {
  CreateEdgeDataDto,
  CreateEdgeSchemaDto,
  CreateNodeDataDto,
  CreateNodeSchemaDto
} from "@/lib/api/dtos";
import { ApiProvider } from "@/lib/api/provider";
import {
  CreateEdgeData,
  CreateEdgeSchema,
  CreateNodeData,
  CreateNodeSchema,
  GraphSchema
} from "@/types";
import { useState } from "react";
import * as v from "valibot";

type GraphSchemaForm = {
  nodes: { id: string; isSaved: boolean; value: CreateNodeSchema; }[];
  edges: { id: string; isSaved: boolean; value: CreateEdgeSchema; }[];
};

type GraphDataForm = {
  nodes: { id: string; isSaved: boolean; value: CreateNodeData; }[];
  edges: { id: string; isSaved: boolean; value: CreateEdgeData; }[];
};

type UseGraphSchemaFormReturn = {
  graphSchema: GraphSchemaForm;
  savedGraphSchema: GraphSchema;
  graphData: GraphDataForm;
  validateGraphSchema: () => boolean;
  validateGraphData: () => boolean;
  graphSchemaErrors: Record<string, string | null>;
  graphDataErrors: Record<string, string | null>;
  generateGraphSchema: (file_content: File, file_type: string) => Promise<void>;
  generateGraphData: (file_content: File, file_type: string) => Promise<void>;
  submitGraphSchema: () => Promise<void>;
  submitGraphData: () => Promise<void>;
};

export const useGraphSchemaForm = (graph_id: string): UseGraphSchemaFormReturn => {
  const { metadata } = useGraph();
  const { graphService } = ApiProvider;
  const [graphSchema, setGraphSchema] = useState<GraphSchemaForm>({ nodes: [], edges: [] });
  const [savedGraphSchema, setSavedGraphSchema] = useState<GraphSchema>({ nodes: [], edges: [] });
  const [graphData, setGraphData] = useState<GraphDataForm>({ nodes: [], edges: [] });
  const [graphSchemaErrors, setGraphSchemaErrors] = useState<Record<string, string | null>>({});
  const [graphDataErrors, setGraphDataErrors] = useState<Record<string, string | null>>({});

  const validateGraphSchema = () => {
    let isSuccess = true;
    graphSchema.nodes.forEach((node) => {
      const validation = v.safeParse(CreateNodeSchemaDto, node.value);
      if (!validation.success) {
        isSuccess = false;
        setGraphSchemaErrors((prev) => ({
          ...prev,
          [node.id]: validation.issues[0].message
        }));
      }
    });
    graphSchema.edges.forEach((edge) => {
      const validation = v.safeParse(CreateEdgeSchemaDto, edge.value);
      if (!validation.success) {
        isSuccess = false;
        setGraphSchemaErrors((prev) => ({
          ...prev,
          [edge.id]: validation.issues[0].message
        }));
      }
    });
    return isSuccess;
  };

  const validateGraphData = () => {
    let isSuccess = true;
    graphData.nodes.forEach((node) => {
      const validation = v.safeParse(CreateNodeDataDto, node.value);
      if (!validation.success) {
        isSuccess = false;
        setGraphDataErrors((prev) => ({
          ...prev,
          [node.id]: validation.issues[0].message
        }));
      }
    });
    graphData.edges.forEach((edge) => {
      const validation = v.safeParse(CreateEdgeDataDto, edge.value);
      if (!validation.success) {
        isSuccess = false;
        setGraphDataErrors((prev) => ({
          ...prev,
          [edge.id]: validation.issues[0].message
        }));
      }
    });
    return isSuccess;
  };

  const generateGraphSchema = async (file_content: File, file_type: string) => {
    const res = await graphService.generateSchema(graph_id, file_content, file_type);
    setGraphSchema({
      nodes: res.nodes.map((node) => ({ id: crypto.randomUUID(), isSaved: false, value: node })),
      edges: res.edges.map((edge) => ({ id: crypto.randomUUID(), isSaved: false, value: edge }))
    });
  };

  const generateGraphData = async (file_content: File, file_type: string) => {
    const res = await graphService.generateData(graph_id, file_content, file_type);
    setGraphData({
      nodes: res.nodes.map((node) => ({ id: crypto.randomUUID(), isSaved: false, value: node })),
      edges: res.edges.map((edge) => ({ id: crypto.randomUUID(), isSaved: false, value: edge }))
    });
  };

  const submitGraphSchema = async () => {
    const res = await graphService.createSchema(
      metadata!.graph_id,
      {
        nodes: graphSchema.nodes.map((node) => node.value),
        edges: graphSchema.edges.map((edge) => edge.value)
      }
    );
    setSavedGraphSchema(res);
  };

  const submitGraphData = async () => {
    await graphService.createData(
      metadata!.graph_id,
      {
        nodes: graphData.nodes.map((node) => node.value),
        edges: graphData.edges.map((edge) => edge.value)
      }
    );
  };

  return {
    graphSchema,
    savedGraphSchema,
    graphData,
    validateGraphSchema,
    validateGraphData,
    graphSchemaErrors,
    graphDataErrors,
    generateGraphSchema,
    generateGraphData,
    submitGraphSchema,
    submitGraphData
  };
};
