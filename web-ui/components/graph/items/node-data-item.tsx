"use client";

import { useGraph } from "@/contexts/graph-context";
import { useMemo } from "react";
import GraphElementItem from "./element-data-item";

const GraphNodeItem = () => {
  const { schema, processedData, focusNode } = useGraph();

  const match = useMemo(() => {
    if (!focusNode || !processedData || !schema) { return null; }
    const node = processedData.nodes.find((n) => n.id === focusNode);
    if (!node) { return null; }
    const nodeSchema = schema.nodes.find((n) => n.key === node.key);
    return nodeSchema ? { node, schema: nodeSchema } : null;
  }, [focusNode, processedData, schema]);

  if (!match) { return null; }

  return (
    <GraphElementItem
      kind="node"
      id={match.node.id}
      label={match.schema.label}
      color={match.schema.color}
      properties={match.node.properties}
    />
  );
};

export default GraphNodeItem;
