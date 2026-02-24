"use client";

import { ApiProvider } from "@/lib/api/provider";
import { Action, GraphData, GraphMetadata, GraphSchema, ProcessedGraphData } from "@/types";
import { createContext, useContext, useEffect, useState } from "react";

type DisplayedProperties = Record<string, string | undefined>;

type GraphContextType = {
  metadata: GraphMetadata | null;
  schema: GraphSchema | null;
  data: GraphData | null;
  processedData: ProcessedGraphData | null;
  isLoading: boolean;
  isLoaded: boolean;
  error: string | null;
  displayedNodeProperties: DisplayedProperties;
  displayedEdgeProperties: DisplayedProperties;
  updateDisplayedNodeProperty: (
    node_key: string,
    property_key: string | undefined
  ) => void;
  updateDisplayedEdgeProperty: (
    edge_key: string,
    property_key: string | undefined
  ) => void;
  focusNode: string | null;
  setFocusNode: (nodeId: string | null) => void;
  focusEdge: string | null;
  setFocusEdge: (edgeId: string | null) => void;
  action: Action | null;
  setAction: (action: Action | null) => void;
};

const GraphContext = createContext<GraphContextType | undefined>(undefined);

type GraphProviderProps = {
  graphId: string | null;
  children: React.ReactNode;
};

// todo: move this in backend ?
const processGraphData = (
  { graphData, graphSchema }: { graphData: GraphData; graphSchema: GraphSchema; }
): ProcessedGraphData => {
  const nodes = graphData.nodes.map((node) => {
    const nodeSchema = graphSchema.nodes.find((n) => n.key === node.key);
    const color = nodeSchema ? nodeSchema.color : "#888888";
    return {
      id: node.node_data_id,
      key: node.key,
      color,
      properties: node.properties
    };
  });

  const links = graphData.edges.map((edge) => {
    const edgeSchema = graphSchema.edges.find((e) => e.key === edge.key);
    const color = edgeSchema ? edgeSchema.color : "#888888";
    return {
      id: edge.edge_data_id,
      source: edge.from_node_data_id,
      target: edge.to_node_data_id,
      key: edge.key,
      color,
      properties: edge.properties
    };
  });

  return { nodes, links };
};

export const GraphProvider = ({ graphId, children }: GraphProviderProps) => {
  const { graphService } = ApiProvider;
  const [metadata, setMetadata] = useState<GraphMetadata | null>(null);
  const [data, setData] = useState<GraphData | null>(null);
  const [schema, setSchema] = useState<GraphSchema | null>(null);
  const [processedData, setProcessedData] = useState<ProcessedGraphData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoaded, setIsLoaded] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [displayedNodeProperties, setDisplayedNodeProperties] = useState<DisplayedProperties>({});
  const [displayedEdgeProperties, setDisplayedEdgeProperties] = useState<DisplayedProperties>({});
  const [focusNode, setFocusNode] = useState<string | null>(null);
  const [focusEdge, setFocusEdge] = useState<string | null>(null);
  const [action, setAction] = useState<Action | null>(null);

  const updateDisplayedNodeProperty = (label: string, property: string | undefined) => {
    setDisplayedNodeProperties((prev) => ({
      ...prev,
      [label]: property
    }));
  };

  const updateDisplayedEdgeProperty = (label: string, property: string | undefined) => {
    setDisplayedEdgeProperties((prev) => ({
      ...prev,
      [label]: property
    }));
  };

  const reset = () => {
    setMetadata(null);
    setData(null);
    setSchema(null);
    setProcessedData(null);
    setIsLoading(false);
    setIsLoaded(false);
    setError(null);
    setDisplayedNodeProperties({});
    setDisplayedEdgeProperties({});
    setFocusNode(null);
    setFocusEdge(null);
  };

  useEffect(() => {
    const fetchGraph = async () => {
      if (!graphId) {
        reset();
        setError("No graph ID provided");
        return;
      }

      try {
        setError(null);
        setIsLoading(true);
        const metadataRes = await graphService.getOneMetadata(graphId);
        setMetadata(metadataRes);
        const schemaRes = await graphService.getSchema(graphId);
        setSchema(schemaRes);
        const dataRes = await graphService.getData(graphId);
        setData(dataRes);
        setProcessedData(processGraphData({ graphData: dataRes, graphSchema: schemaRes }));
        setDisplayedNodeProperties((prev) =>
          Object.fromEntries(
            Object.keys(prev).filter((k) =>
              dataRes.nodes.some((node) => node.key === k)
            )
              .map((k) => [k, prev[k]])
          )
        );
        setDisplayedEdgeProperties((prev) =>
          Object.fromEntries(
            Object.keys(prev).filter((k) =>
              dataRes.edges.some((edge) => edge.key === k)
            )
              .map((k) => [k, prev[k]])
          )
        );
      } catch (err) {
        reset();
        console.error("Error fetching graph:", err);
        setError("Failed to load graph.");
      } finally {
        setIsLoading(false);
      }
    };

    fetchGraph();
  }, [graphId]);

  useEffect(() => {
    if (metadata && schema && data && processedData && !isLoading && !error) {
      setIsLoaded(true);
    }
  }, [metadata, schema, data, processedData, isLoading, error]);

  return (
    <GraphContext.Provider
      value={{
        metadata,
        schema,
        data,
        processedData,
        isLoading,
        isLoaded,
        error,
        displayedNodeProperties,
        displayedEdgeProperties,
        updateDisplayedNodeProperty,
        updateDisplayedEdgeProperty,
        focusNode,
        setFocusNode,
        focusEdge,
        setFocusEdge,
        action,
        setAction
      }}
    >
      {children}
    </GraphContext.Provider>
  );
};

export const useGraph = () => {
  const context = useContext(GraphContext);
  if (context === undefined) {
    throw new Error("useGraph must be used within a GraphProvider");
  }
  return context;
};
