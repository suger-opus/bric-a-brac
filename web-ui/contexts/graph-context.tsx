"use client";

import { graphService } from "@/lib/api/services/graph-service";
import type { GraphData, GraphMetadata, GraphSchema, ProcessedEdgeData, ProcessedGraphData, ProcessedNodeData } from "@/types";
import { createContext, useCallback, useContext, useEffect, useState } from "react";
import { toast } from "sonner";

type GraphContextType = {
  graphId: string | null;
  metadata: GraphMetadata | null;
  schema: GraphSchema | null;
  data: GraphData | null;
  processedData: ProcessedGraphData | null;
  isLoading: boolean;
  isLoaded: boolean;
  error: string | null;
  focusNode: string | null;
  setFocusNode: (nodeId: string | null) => void;
  focusEdge: string | null;
  setFocusEdge: (edgeId: string | null) => void;
  refetch: () => void;
  addNode: (node: ProcessedNodeData) => void;
  addEdge: (edge: ProcessedEdgeData) => void;
  updateNode: (nodeId: string, properties: Record<string, string | number | boolean>) => void;
  updateEdge: (edgeId: string, properties: Record<string, string | number | boolean>) => void;
  removeNode: (nodeId: string) => void;
  removeEdge: (edgeId: string) => void;
};

const GraphContext = createContext<GraphContextType | undefined>(undefined);

function processGraphData(graphData: GraphData, graphSchema: GraphSchema): ProcessedGraphData {
  const nodes = graphData.nodes.map((node) => {
    const schema = graphSchema.nodes.find((n) => n.key === node.key);
    return {
      id: node.node_data_id,
      key: node.key,
      color: schema?.color ?? "#888888",
      properties: node.properties,
    };
  });

  const links = graphData.edges.map((edge) => {
    const schema = graphSchema.edges.find((e) => e.key === edge.key);
    return {
      id: edge.edge_data_id,
      source: edge.from_node_data_id,
      target: edge.to_node_data_id,
      key: edge.key,
      color: schema?.color ?? "#888888",
      properties: edge.properties,
    };
  });

  return { nodes, links };
}

export const GraphProvider = ({ graphId, children }: { graphId: string | null; children: React.ReactNode }) => {
  const [metadata, setMetadata] = useState<GraphMetadata | null>(null);
  const [data, setData] = useState<GraphData | null>(null);
  const [schema, setSchema] = useState<GraphSchema | null>(null);
  const [processedData, setProcessedData] = useState<ProcessedGraphData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoaded, setIsLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [focusNode, setFocusNode] = useState<string | null>(null);
  const [focusEdge, setFocusEdge] = useState<string | null>(null);
  const [fetchTrigger, setFetchTrigger] = useState(0);

  const refetch = useCallback(() => setFetchTrigger((n) => n + 1), []);

  const addNode = useCallback((node: ProcessedNodeData) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      if (prev.nodes.some((n) => n.id === node.id)) return prev;
      return { ...prev, nodes: [...prev.nodes, node] };
    });
  }, []);

  const addEdge = useCallback((edge: ProcessedEdgeData) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      if (prev.links.some((l) => l.id === edge.id)) return prev;
      return { ...prev, links: [...prev.links, edge] };
    });
  }, []);

  const updateNode = useCallback((nodeId: string, properties: Record<string, string | number | boolean>) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        nodes: prev.nodes.map((n) => n.id === nodeId ? { ...n, properties } : n),
      };
    });
  }, []);

  const updateEdge = useCallback((edgeId: string, properties: Record<string, string | number | boolean>) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      return {
        ...prev,
        links: prev.links.map((l) => l.id === edgeId ? { ...l, properties } : l),
      };
    });
  }, []);

  const removeNode = useCallback((nodeId: string) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      return {
        nodes: prev.nodes.filter((n) => n.id !== nodeId),
        links: prev.links.filter((l) => l.source !== nodeId && l.target !== nodeId),
      };
    });
  }, []);

  const removeEdge = useCallback((edgeId: string) => {
    setProcessedData((prev) => {
      if (!prev) return prev;
      return { ...prev, links: prev.links.filter((l) => l.id !== edgeId) };
    });
  }, []);

  useEffect(() => {
    if (!graphId) {
      setError("No graph ID provided");
      setIsLoading(false);
      return;
    }

    let cancelled = false;

    const fetchGraph = async () => {
      try {
        setError(null);
        setIsLoading(true);

        const [metadataRes, schemaRes, dataRes] = await Promise.all([
          graphService.getOneMetadata(graphId),
          graphService.getSchema(graphId),
          graphService.getData(graphId),
        ]);

        if (cancelled) return;

        setMetadata(metadataRes);
        setSchema(schemaRes);
        setData(dataRes);
        setProcessedData(processGraphData(dataRes, schemaRes));
        setIsLoaded(true);
      } catch {
        if (cancelled) return;
        setError("Failed to load graph.");
        toast.error("Failed to load graph");
      } finally {
        if (!cancelled) setIsLoading(false);
      }
    };

    fetchGraph();
    return () => { cancelled = true; };
  }, [graphId, fetchTrigger]);

  return (
    <GraphContext.Provider
      value={{
        graphId,
        metadata, schema, data, processedData,
        isLoading, isLoaded, error,
        focusNode, setFocusNode,
        focusEdge, setFocusEdge,
        refetch,
        addNode, addEdge, updateNode, updateEdge, removeNode, removeEdge,
      }}
    >
      {children}
    </GraphContext.Provider>
  );
};

export const useGraph = () => {
  const context = useContext(GraphContext);
  if (!context) throw new Error("useGraph must be used within a GraphProvider");
  return context;
};
