"use client";

import { ApiProvider } from "@/lib/api/provider";
import { GraphData, GraphMetadata, GraphSchema, ProcessedGraphData } from "@/types";
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
    node_formatted_label: string,
    property_formatted_label: string | undefined
  ) => void;
  updateDisplayedEdgeProperty: (
    edge_formatted_label: string,
    property_formatted_label: string | undefined
  ) => void;
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
    const nodeSchema = graphSchema.nodes.find((n) => n.formatted_label === node.formatted_label);
    const color = nodeSchema ? nodeSchema.color : "#888888";
    return {
      id: node.node_id,
      formatted_label: node.formatted_label,
      color,
      properties: node.properties
    };
  });

  const links = graphData.edges.map((edge) => {
    const edgeSchema = graphSchema.edges.find((e) => e.formatted_label === edge.formatted_label);
    const color = edgeSchema ? edgeSchema.color : "#888888";
    return {
      source: edge.from_id,
      target: edge.to_id,
      formatted_label: edge.formatted_label,
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

  useEffect(() => {
    const fetchGraph = async () => {
      if (!graphId) {
        setMetadata(null);
        setSchema(null);
        setData(null);
        setProcessedData(null);
        setIsLoading(false);
        setDisplayedNodeProperties({});
        setDisplayedEdgeProperties({});
        setError("No graph ID provided");
        return;
      }

      try {
        setError(null);
        setIsLoading(true);
        setDisplayedNodeProperties({});
        setDisplayedEdgeProperties({});
        const metadata = await graphService.getMetadata(graphId);
        setMetadata(metadata);
        const schema = await graphService.getSchema(graphId);
        setSchema(schema);
        const data = await graphService.getData(graphId);
        setData(data);
        setProcessedData(processGraphData({ graphData: data, graphSchema: schema }));
      } catch (err) {
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
        updateDisplayedEdgeProperty
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
