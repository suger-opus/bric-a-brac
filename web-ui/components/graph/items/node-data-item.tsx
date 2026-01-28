"use client";

import { useGraph } from "@/contexts/graph-context";
import { NodeSchema, ProcessedNodeData } from "@/types";
import { useEffect, useEffectEvent, useState } from "react";
import GraphElementItem from "./element-data-item";

const GraphNodeItem = () => {
  const { metadata, schema, processedData, focusNode } = useGraph();
  const [nodeData, setNodeData] = useState<ProcessedNodeData | null>(null);
  const [nodeSchema, setNodeSchema] = useState<NodeSchema | null>(null);

  const findNode = useEffectEvent(() => {
    const foundData = processedData?.nodes.find((n) => n.id === focusNode);
    const foundSchema = schema?.nodes.find((n) => n.formatted_label === foundData?.formatted_label);
    setNodeData(foundData || null);
    setNodeSchema(foundSchema || null);
  });

  useEffect(() => {
    findNode();
  }, [focusNode]);

  return (nodeData && nodeSchema)
    ? (
      <GraphElementItem
        kind="node"
        id={focusNode!}
        label={nodeSchema.label}
        color={nodeSchema.color}
        schemaProperties={nodeSchema.properties}
        dataProperties={nodeData.properties}
        role={metadata!.user_role}
      />
    )
    : null;
};

export default GraphNodeItem;
