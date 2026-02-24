import { useGraph } from "@/contexts/graph-context";
import { CreateEdgeSchemaDto, CreateNodeSchemaDto } from "@/lib/api/dtos";
import { ApiProvider } from "@/lib/api/provider";
import { CreateEdgeSchema, CreateGraphSchema, CreateNodeSchema } from "@/types";
import { useState } from "react";
import * as v from "valibot";

type UseGraphSchemaFormReturn = {
  nodesSchemas: { id: string; isSaved: boolean; value: CreateNodeSchema; }[];
  edgesSchemas: { id: string; isSaved: boolean; value: CreateEdgeSchema; }[];
  loadGraphSchema: (graphSchema: CreateGraphSchema) => void;
  validateGraphSchema: () => boolean;
  graphSchemaErrors: Record<string, string | null>;
  submitGraphSchema: () => Promise<void>;
};

// TODO: enable edition
export const useGraphSchemaForm = (): UseGraphSchemaFormReturn => {
  const { graphService } = ApiProvider;
  const { metadata } = useGraph();
  const [nodesSchemas, setNodesSchemas] = useState<
    { id: string; isSaved: boolean; value: CreateNodeSchema; }[]
  >([]);
  const [edgesSchemas, setEdgesSchemas] = useState<
    { id: string; isSaved: boolean; value: CreateEdgeSchema; }[]
  >([]);
  const [graphSchemaErrors, setGraphSchemaErrors] = useState<Record<string, string | null>>({});

  const loadGraphSchema = (graphSchema: CreateGraphSchema) => {
    const loadedNodes = graphSchema.nodes.map((node) => ({
      id: crypto.randomUUID(),
      isSaved: true,
      value: node
    }));
    const loadedEdges = graphSchema.edges.map((edge) => ({
      id: crypto.randomUUID(),
      isSaved: true,
      value: edge
    }));
    setNodesSchemas(loadedNodes);
    setEdgesSchemas(loadedEdges);
  };

  const validateGraphSchema = () => {
    let isSuccess = true;
    nodesSchemas.forEach((node) => {
      const validation = v.safeParse(CreateNodeSchemaDto, node.value);
      if (!validation.success) {
        isSuccess = false;
        setGraphSchemaErrors((prev) => ({
          ...prev,
          [node.id]: validation.issues[0].message
        }));
      }
    });
    edgesSchemas.forEach((edge) => {
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

  const submitGraphSchema = async () => {
    graphService.createSchema(
      metadata!.graph_id,
      {
        nodes: nodesSchemas.map((node) => node.value),
        edges: edgesSchemas.map((edge) => edge.value)
      }
    );
  };

  return {
    nodesSchemas,
    edgesSchemas,
    graphSchemaErrors,
    validateGraphSchema,
    loadGraphSchema,
    submitGraphSchema
  };
};
