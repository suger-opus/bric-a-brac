"use client";

import { useGraph } from "@/contexts/graph-context";
import { useMemo } from "react";
import GraphElementItem from "./element-data-item";

const GraphEdgeItem = () => {
  const { schema, processedData, focusEdge } = useGraph();

  const match = useMemo(() => {
    if (!focusEdge || !processedData || !schema) return null;
    const edge = processedData.links.find((l) => l.id === focusEdge);
    if (!edge) return null;
    const edgeSchema = schema.edges.find((e) => e.key === edge.key);
    return edgeSchema ? { edge, schema: edgeSchema } : null;
  }, [focusEdge, processedData, schema]);

  if (!match) return null;

  return (
    <GraphElementItem
      kind="edge"
      id={match.edge.id}
      label={match.schema.label}
      color={match.schema.color}
      properties={match.edge.properties}
    />
  );
};

export default GraphEdgeItem;
