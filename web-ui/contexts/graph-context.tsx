"use client";

import { graphService } from "@/lib/api/services/graph-service";
import type { GraphData, GraphMetadata, GraphSchema, ProcessedGraphData } from "@/types";
import { createContext, useCallback, useContext, useEffect, useState } from "react";

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
